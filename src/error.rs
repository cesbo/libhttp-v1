pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };

    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into());
    };
}


#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) { bail!($e); }
    };

    ($cond:expr, $fmt:expr, $($arg:tt)+) => {
        if !($cond) { bail!($fmt, $($arg)+); }
    };
}
