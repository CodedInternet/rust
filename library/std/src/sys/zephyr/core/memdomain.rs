use core::marker::PhantomData;

pub use super::super::raw::k_mem_domain;

use super::thread::ThreadId;

pub struct MemDomain<'a>(PhantomData<&'a ()>);

impl<'a> MemDomain<'a> {

    pub fn new() -> Self {
        MemDomain(PhantomData)
    }

    pub fn add_thread<C: MemDomainAPI>(&self, _thread: ThreadId) {
        #[cfg(usermode)]
        C::k_mem_domain_add_thread(self.0, _thread)
    }
}

pub trait MemDomainAPI {
    fn k_mem_domain_add_thread(domain: &k_mem_domain, thread: ThreadId);
}

impl MemDomainAPI for crate::context::Kernel {
    fn k_mem_domain_add_thread(domain: &k_mem_domain, thread: ThreadId) {
        unsafe {
            zephyr_sys::raw::k_mem_domain_add_thread(domain as *const _ as *mut _, thread.tid())
        }
    }
}

/// Get a static reference to an external mem domain
#[cfg(usermode)]
#[macro_export]
macro_rules! static_mem_domain {
    ($domain:ident) => {{
        extern "C" {
            #[no_mangle]
            static $domain: std::sys::zephyr::core::memdomain::k_mem_domain;
        }

        unsafe { std::sys::zephyr::core::memdomain::MemDomain::new(&$domain) }
    }};
}
/// Get a static reference to an external mem domain
#[cfg(not(usermode))]
#[macro_export]
macro_rules! static_mem_domain {
    ($domain:ident) => {{
        std::sys::zephyr::core::memdomain::MemDomain::new()
    }};
}
