use core::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum ConvertError {
    UnknownFromEncoding,
    UnknownToEncoding,
    InvalidInput,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConvertLossyError {
    UnknownFromEncoding,
    UnknownToEncoding,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ConvertError::UnknownFromEncoding => "invalid from_encoding",
            ConvertError::UnknownToEncoding => "invalid to_encoding",
            ConvertError::InvalidInput => "input contains invalid data for the given from_encoding",
        })
    }
}

impl fmt::Display for ConvertLossyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ConvertLossyError::UnknownFromEncoding => "invalid from_encoding",
            ConvertLossyError::UnknownToEncoding => "invalid to_encoding",
        })
    }
}

impl From<ConvertLossyError> for ConvertError {
    fn from(err: ConvertLossyError) -> Self {
        match err {
            ConvertLossyError::UnknownFromEncoding => ConvertError::UnknownFromEncoding,
            ConvertLossyError::UnknownToEncoding => ConvertError::UnknownToEncoding,
        }
    }
}
