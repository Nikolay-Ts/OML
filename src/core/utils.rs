#[macro_export]
macro_rules! define_error {
    ($error_name:ident, $prefix:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $error_name {
            pub message: String
        }

        impl $error_name {
            pub fn new(message: String) -> Self {
                Self {
                    message
                }
            }
        }

        impl fmt::Display for $error_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}{}", $prefix, self.message)
            }
        }

        impl std::error::Error for $error_name {}
    };
}