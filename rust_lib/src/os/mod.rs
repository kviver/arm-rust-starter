cfg_if! {
    if #[cfg(all(target_arch="arm", target_os="none"))] {
        pub mod cmsis_os;
        pub use self::cmsis_os::*;
    } else if #[cfg(unix)] {
        pub mod linux_os;
        pub use self::linux_os::*;
    }
}
