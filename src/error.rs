use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ConvertError {
    UnknownConversion,
    InvalidInput,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConvertLossyError {
    UnknownConversion,
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ConvertError::UnknownConversion => "invalid from_encoding or to_encoding",
            ConvertError::InvalidInput => "input contains invalid data for the given from_encoding",
        })
    }
}

impl fmt::Display for ConvertLossyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ConvertLossyError::UnknownConversion => "invalid from_encoding or to_encoding",
        })
    }
}

impl From<ConvertLossyError> for ConvertError {
    fn from(err: ConvertLossyError) -> Self {
        match err {
            ConvertLossyError::UnknownConversion => ConvertError::UnknownConversion,
        }
    }
}
