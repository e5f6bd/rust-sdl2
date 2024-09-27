use std::error::Error;
use std::fmt;

#[cfg(not(feature = "no_more_string_error"))]
pub type SdlErrorString = String;
#[cfg(feature = "no_more_string_error")]
#[derive(Clone, Debug)]
/// Contains the description of an error returned by SDL
pub struct SdlErrorString(String);

#[cfg(feature = "no_more_string_error")]
impl From<String> for SdlErrorString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[cfg(feature = "no_more_string_error")]
impl fmt::Display for SdlErrorString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SDL error: {}", self.0)
    }
}

#[cfg(feature = "no_more_string_error")]
impl Error for SdlErrorString {}

/// A given integer was so big that its representation as a C integer would be
/// negative.
#[derive(Debug, Clone, PartialEq)]
pub enum IntegerOrSdlError {
    IntegerOverflows(&'static str, u32),
    SdlError(String),
}
/// Validates and converts the given u32 to a positive C integer.
pub fn validate_int(value: u32, name: &'static str) -> Result<::libc::c_int, IntegerOrSdlError> {
    use self::IntegerOrSdlError::*;
    // Many SDL functions will accept `int` values, even if it doesn't make sense
    // for the values to be negative.
    // In the cases that SDL doesn't check negativity, passing negative values
    // could be unsafe.
    // For example, `SDL_JoystickGetButton` uses the index argument to access an
    // array without checking if it's negative, which could potentially lead to
    // segmentation faults.
    if value >= 1 << 31 {
        Err(IntegerOverflows(name, value))
    } else {
        Ok(value as ::libc::c_int)
    }
}

impl fmt::Display for IntegerOrSdlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::IntegerOrSdlError::*;

        match *self {
            IntegerOverflows(name, value) => write!(f, "Integer '{}' overflows ({})", name, value),
            SdlError(ref e) => write!(f, "SDL error: {}", e),
        }
    }
}

impl Error for IntegerOrSdlError {}
