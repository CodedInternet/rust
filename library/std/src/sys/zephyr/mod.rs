use crate::io::ErrorKind;

pub mod raw;

pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
pub mod locks;
#[path = "../unix/memchr.rs"]
pub mod memchr;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/once.rs"]
pub mod once;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
pub mod thread;
pub mod thread_local_key;
pub mod time;

#[path = "../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
#[allow(unused)]
mod common;
pub use common::{abort_internal, cleanup, hashmap_random_keys, init, unsupported, unsupported_err};

// Could import user-only here with a feature
use zephyr_core::any as zephyr;

pub mod env {
    pub mod os {
        pub const FAMILY: &str = "zephyr";
        pub const OS: &str = "zephyr";
        pub const DLL_PREFIX: &str = "";
        pub const DLL_SUFFIX: &str = "";
        pub const DLL_EXTENSION: &str = "";
        pub const EXE_SUFFIX: &str = "";
        pub const EXE_EXTENSION: &str = "";
    }
}

pub use libc::strlen;

pub fn decode_error_kind(errno: i32) -> crate::io::ErrorKind {
    match errno as u32 {
        self::raw::ECONNREFUSED => ErrorKind::ConnectionRefused,
        self::raw::ECONNRESET => ErrorKind::ConnectionReset,
        self::raw::EPERM | self::raw::EACCES => ErrorKind::PermissionDenied,
        self::raw::EPIPE => ErrorKind::BrokenPipe,
        self::raw::ENOTCONN => ErrorKind::NotConnected,
        self::raw::ECONNABORTED => ErrorKind::ConnectionAborted,
        self::raw::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        self::raw::EADDRINUSE => ErrorKind::AddrInUse,
        self::raw::ENOENT => ErrorKind::NotFound,
        self::raw::EINTR => ErrorKind::Interrupted,
        self::raw::EINVAL => ErrorKind::InvalidInput,
        self::raw::ETIMEDOUT => ErrorKind::TimedOut,
        self::raw::EEXIST => ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == self::raw::EAGAIN || x == self::raw::EWOULDBLOCK => {
            ErrorKind::WouldBlock
        }

        _ => ErrorKind::Other,
    }
}
