/// Example:
///     function_name(out_string: &mut *const c_char) {
///         let s = "Goodbye cruel world!".to_string();
///         assign_out_string!(out_string, s);
///     }
macro_rules! assign_out_string {
    ($input:expr, $string:expr) => {
        *$input = ffi_support::rust_string_to_c($string);
    };
}

macro_rules! handle_result {
    ($err:expr) => {
        // If not success then return false
        if $err.get_code().code() > 0 {
            0
        } else {
            1
        }
    };
}

macro_rules! impl_error {
    ($errname:ident, $errkind:ident) => {
       #[derive(Debug)]
        pub struct $errname {
            inner: failure::Context<$errkind>
        }

        impl failure::Fail for $errname {
            fn cause(&self) -> Option<&dyn failure::Fail> { self.inner.cause() }

            fn backtrace(&self) -> Option<&failure::Backtrace> { self.inner.backtrace() }
        }

        impl $errname {
            pub fn from_msg<D: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static>(kind: $errkind, msg: D) -> $errname {
                $errname {
                    inner: failure::Context::new(msg).context(kind)
                }
            }

            pub fn from_kind(kind: $errkind) -> $errname {
                $errname {
                    inner: failure::Context::new("").context(kind)
                }
            }

            pub fn kind(&self) -> $errkind {
                self.inner.get_context().clone()
            }
        }

        impl std::fmt::Display for $errname {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut first = true;

                for cause in Fail::iter_chain(&self.inner) {
                    if first {
                        first = false;
                        writeln!(f, "Error: {}", cause)?;
                    } else {
                        writeln!(f, "Caused by: {}", cause)?;
                    }
                }

                Ok(())
            }
        }

        impl From<failure::Context<$errkind>> for $errname {
            fn from(inner: failure::Context<$errkind>) -> $errname {
                $errname { inner }
            }
        }
    };
}

pub mod error;
pub mod wallet;

define_string_destructor!(aries_sdk_free_string);