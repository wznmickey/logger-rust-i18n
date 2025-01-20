#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {
        log::log!(log::Level::Error, "{}", rust_i18n::t!(format!($($args)*)));
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {
        log::log!(log::Level::Warn, "{}", rust_i18n::t!(format!($($args)*)));
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {
        log::log!(log::Level::Info, "{}", rust_i18n::t!(format!($($args)*)));
    };
}

#[macro_export]
macro_rules! debug {
    ( $($args:tt)*) => {
        log::log!(log::Level::Debug, "{}", rust_i18n::t!(format!( $($args)*)));
    };
}

#[macro_export]
macro_rules! trace {
    ( $($args:tt)*) => {
        log::log!(log::Level::Trace, "{}", rust_i18n::t!(format!( $($args)*)));
    };
}
#[macro_export]
macro_rules! prompt {
    ( $($args:tt)*) => {
        rust_i18n::t!(format!( $($args)*));
    };
}
