cfg_if! {
    if #[cfg(all(target_arch="arm", target_os="none"))] {
        pub use core::fmt::*;
    } else if #[cfg(unix)] {
        pub use std::fmt::*;
    }
}
