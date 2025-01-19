#[macro_export]
macro_rules! error {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Error, "{}", t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Warn, "{}", t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Info, "{}", t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Debug, "{}", t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Trace, "{}", t!($fmt, $($args)*));
    };
}