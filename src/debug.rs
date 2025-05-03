#[macro_export]
macro_rules! ray_debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "ray_debug")]
        {
            println!($($arg)*);
        }
    };
}
