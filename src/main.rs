#[macro_export]
macro_rules! error {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Error, "{}", rust_i18n::t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Warn, "{}", rust_i18n::t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Info, "{}", rust_i18n::t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Debug, "{}", rust_i18n::t!($fmt, $($args)*));
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt:expr, $($args:tt)*) => {
        log::log!(log::Level::Trace, "{}", rust_i18n::t!($fmt, $($args)*));
    };
}

fn main() {
    // Your code here
}