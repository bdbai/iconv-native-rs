use core::fmt;

#[cfg(doc)]
use crate::{convert, convert_lossy, decode, decode_lossy};

/// Error representation for [`decode`] and [`convert`].
#[derive(Debug, PartialEq, Eq)]
pub enum ConvertError {
    /// The encodings provided or the specific conversion pair is not supported
    /// by the implementation.
    UnknownConversion,
    /// The input data contains invalid data for the given encoding, or the
    /// input data contains a character that is not representable in the target
    /// encoding.
    InvalidInput,
}

/// Error representation for [`decode_lossy`] and [`convert_lossy`].
#[derive(Debug, PartialEq, Eq)]
pub enum ConvertLossyError {
    /// The encodings provided or the specific conversion pair is not supported
    /// by the implementation.
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
