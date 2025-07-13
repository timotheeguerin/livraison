// ========================================================================= //

macro_rules! already_exists {
    ($e:expr_2021) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::AlreadyExists,
                                         $e))
    };
    ($fmt:expr_2021, $($arg:tt)+) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::AlreadyExists,
                                         format!($fmt, $($arg)+)))
    };
}

macro_rules! invalid_data {
    ($e:expr_2021) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData,
                                         $e))
    };
    ($fmt:expr_2021, $($arg:tt)+) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData,
                                         format!($fmt, $($arg)+)))
    };
}

macro_rules! invalid_input {
    ($e:expr_2021) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput,
                                         $e))
    };
    ($fmt:expr_2021, $($arg:tt)+) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput,
                                         format!($fmt, $($arg)+)))
    };
}

macro_rules! not_found {
    ($e:expr_2021) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::NotFound, $e))
    };
    ($fmt:expr_2021, $($arg:tt)+) => {
        return Err(::std::io::Error::new(::std::io::ErrorKind::NotFound,
                                         format!($fmt, $($arg)+)))
    };
}

// ========================================================================= //
