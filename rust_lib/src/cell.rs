cfg_if! {
    if #[cfg(all(target_arch="arm", target_os="none"))] {
        pub use core::cell::*;
    } else if #[cfg(unix)] {
        pub use std::cell::*;
    }
}
