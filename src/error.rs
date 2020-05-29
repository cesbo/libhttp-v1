pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        Err(format!($($arg)*).into())
    }
}


#[macro_export]
macro_rules! ensure {
    ($cond:expr, $($arg:tt)*) => {
        if !($cond) { return err!($($arg)*) }
    }
}
