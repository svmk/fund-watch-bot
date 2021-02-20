pub use anyhow::Error as Failure;
pub use anyhow::anyhow as error;

#[macro_export]
macro_rules! fail {
    ($msg:literal $(,)?) => {
        Err(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        Err(anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        Err(anyhow::anyhow!($fmt, $($arg)*))
    };
}
