#[macro_export]
macro_rules! fl {
    ($expression:expr) => {
        0x1 << $expression
    };
}
