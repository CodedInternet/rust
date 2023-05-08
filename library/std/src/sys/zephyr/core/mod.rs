#![cfg_attr(not(feature = "have_std"), no_std)]
#![feature(never_type)]

#[macro_use]
extern crate derive_more;

pub mod kobj;
pub mod memdomain;
pub mod mempool;
pub mod mutex;
pub mod mutex_alloc;
pub mod poll;
mod poll_signal;
pub mod semaphore;
pub mod thread;
mod time;

pub use time::*;
use super::raw::KERNEL_VERSION_NUMBER;
use core::fmt;

// Set from environment from build.rs
pub const CONFIG_USERSPACE: bool = cfg!(usermode);

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct KernelVersion(u32);

// Zephyr kernel version number
pub const KERNEL_VERSION: KernelVersion = KernelVersion(KERNEL_VERSION_NUMBER);

impl fmt::Display for KernelVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let major = (self.0 & 0xff0000) >> 16;
        let minor = (self.0 & 0x00ff00) >> 8;
        let patch = self.0 & 0x0000ff;
        write!(f, "{}.{}.{}", major, minor, patch)
    }
}

// Use this mem pool for global allocs instead of kmalloc
#[cfg(mempool)]
crate::global_sys_mem_pool!(rust_std_mem_pool);

/// Convert a negative error code to a Result
pub trait NegErr {
    fn neg_err(&self) -> Result<u32, u32>;
}

impl NegErr for i32 {
    fn neg_err(&self) -> Result<u32, u32> {
        if *self >= 0 {
            Ok(*self as u32)
        } else {
            Err((-*self) as u32)
        }
    }
}

pub mod context {
    /// Kernel, user, or runtime-detect (any)
    pub unsafe trait Context {}

    pub struct Kernel;
    unsafe impl Context for Kernel {}

    pub struct User;
    unsafe impl Context for User {}

    pub struct Any;
    unsafe impl Context for Any {}
}

macro_rules! zephyr_bindings {
    ($context:ident, $context_struct:path) => {
        #[inline(always)]
        pub fn k_str_out_raw(s: &[u8]) {
            unsafe { super::syscalls::$context::k_str_out(s.as_ptr() as *mut _, s.len()) };
        }

        #[inline(always)]
        pub fn k_str_out(s: &str) {
            k_str_out_raw(s.as_bytes())
        }

        #[inline(always)]
        pub fn k_uptime_ticks() -> crate::Ticks {
            unsafe { crate::Ticks(super::syscalls::$context::k_uptime_ticks()) }
        }

        #[inline(always)]
        pub fn k_sleep(timeout: crate::Timeout) -> crate::DurationMs {
            unsafe { crate::DurationMs::from(super::syscalls::$context::k_sleep(timeout.0)) }
        }

        #[inline(always)]
        pub fn k_thread_custom_data_get() -> *mut u8 {
            unsafe { super::syscalls::$context::k_thread_custom_data_get() as *mut u8 }
        }

        #[inline(always)]
        pub fn k_thread_custom_data_set(value: *mut u8) {
            unsafe { super::syscalls::$context::k_thread_custom_data_set(value as *mut _) };
        }

        #[cfg(clock)]
        #[inline(always)]
        pub fn clock_settime(timespec: super::raw::timespec) {
            use core::convert::TryInto;

            unsafe {
                super::raw::clock_settime(
                    super::raw::CLOCK_REALTIME.try_into().unwrap(),
                    &timespec,
                );
            }
        }

        #[cfg(clock)]
        #[inline(always)]
        pub fn clock_gettime() -> super::raw::timespec {
            use core::convert::TryInto;

            unsafe {
                let mut t: super::raw::timespec = core::mem::zeroed();
                super::syscalls::$context::clock_gettime(
                    super::raw::CLOCK_REALTIME.try_into().unwrap(),
                    &mut t,
                );
                t
            }
        }

        impl crate::mutex::MutexSyscalls for $context_struct {
            unsafe fn k_mutex_init(mutex: *mut super::raw::k_mutex) {
                super::syscalls::$context::k_mutex_init(mutex);
            }

            unsafe fn k_mutex_lock(
                mutex: *mut super::raw::k_mutex,
                timeout: super::raw::k_timeout_t,
            ) -> libc::c_int {
                super::syscalls::$context::k_mutex_lock(mutex, timeout)
            }

            unsafe fn k_mutex_unlock(mutex: *mut super::raw::k_mutex) {
                // TODO: return the error from here. Ignoring now for Zephyr 2.1 compat
                super::syscalls::$context::k_mutex_unlock(mutex);
            }
        }
    };
}

/// Functions only accessible from kernel mode
pub mod kernel {
    use core::alloc::{GlobalAlloc, Layout};
    use core::ptr;
    use libc::c_void;

    zephyr_bindings!(kernel, crate::context::Kernel);

    pub fn k_thread_user_mode_enter<F>(mut f: F) -> !
        where
            F: FnOnce() + Send + Sync,
    {
        extern "C" fn run_closure<F>(p1: *mut c_void, _p2: *mut c_void, _p3: *mut c_void)
            where
                F: FnOnce() + Send + Sync,
        {
            let f = unsafe { ptr::read(p1 as *mut F) };
            f();
        }
        unsafe {
            super::raw::k_thread_user_mode_enter(
                Some(run_closure::<F>),
                &mut f as *mut _ as *mut c_void,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
    }

    fn check_align(ptr: *mut u8, layout: Layout) -> *mut u8 {
        if ptr as usize & (layout.align() - 1) != 0 {
            unsafe {
                super::raw::printk(
                    "Rust unsatisfied alloc alignment\n\0".as_ptr() as *const libc::c_char
                );
            }
            core::ptr::null_mut()
        } else {
            ptr
        }
    }

    pub struct KMalloc;

    unsafe impl GlobalAlloc for KMalloc {
        #[inline]
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            #[cfg(not(zephyr250))]
                let ret = super::raw::k_malloc(layout.size()) as *mut _;
            #[cfg(zephyr250)]
                let ret = super::raw::k_aligned_alloc(layout.align(), layout.size()) as *mut _;

            check_align(ret, layout)
        }

        #[inline]
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let ret = super::raw::k_calloc(1, layout.size()) as *mut _;
            check_align(ret, layout)
        }

        #[inline]
        unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
            super::raw::k_free(ptr as *mut _)
        }
    }
}

// pub mod user {
//     zephyr_bindings!(user, crate::context::User);
// }
//
// pub mod any {
//     zephyr_bindings!(any, crate::context::Any);
// }
