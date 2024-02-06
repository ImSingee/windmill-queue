use std::fmt::Display;

pub trait AsStr {
    /// Get the string representation of a structure
    fn as_str(&self) -> &str;
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str().to_string())
    }
}

macro_rules! define_codes {
    ($($key:ident,)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ErrorCode {
            $($key,)*
        }

        impl AsStr for ErrorCode {
            fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$key => stringify!($key),)*
                }
            }
        }
    };
}

define_codes!(
    E40001,
    E50001,
);