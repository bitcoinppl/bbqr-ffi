use bbqr::consts::{HEADER_LENGTH, MAX_PARTS};
use uniffi::Error;

#[derive(Debug, Error, thiserror::Error)]
pub enum SplitError {
    #[error("No data found")]
    Empty,

    #[error("Cannot make the data fit")]
    CannotFit,

    #[error("Max split size is too large, max is {MAX_PARTS}, got {got}")]
    MaxSplitSizeTooLarge { got: u16 },

    #[error("Min split size is too small, must atleast be 1")]
    MinSplitTooSmall,

    #[error("Invalid split min and max range, min is larger than max")]
    InvalidSplitRange,

    #[error("Invalid version min and max range, min is larger than max")]
    InvalidVersionRange,

    #[error(transparent)]
    EncodeError { error: EncodeError },
}

/// Errors that can occur when encoding data
#[derive(Debug, thiserror::Error, PartialEq, Eq, Error)]
pub enum EncodeError {
    #[error("No data to encode")]
    Empty,

    #[error("Unable to compress data")]
    CompressionError { error: String },
}

impl From<bbqr::split::SplitError> for SplitError {
    fn from(error: bbqr::split::SplitError) -> Self {
        match error {
            bbqr::split::SplitError::Empty => Self::Empty,
            bbqr::split::SplitError::CannotFit => Self::CannotFit,
            bbqr::split::SplitError::MinSplitTooSmall => Self::MinSplitTooSmall,
            bbqr::split::SplitError::InvalidSplitRange => Self::InvalidSplitRange,
            bbqr::split::SplitError::InvalidVersionRange => Self::InvalidVersionRange,
            bbqr::split::SplitError::EncodeError(error) => Self::EncodeError {
                error: error.into(),
            },
            bbqr::split::SplitError::MaxSplitSizeTooLarge(size) => {
                Self::MaxSplitSizeTooLarge { got: size as u16 }
            }
        }
    }
}

impl From<bbqr::encode::EncodeError> for EncodeError {
    fn from(error: bbqr::encode::EncodeError) -> Self {
        match error {
            bbqr::encode::EncodeError::Empty => Self::Empty,
            bbqr::encode::EncodeError::CompressionError(error) => Self::CompressionError {
                error: error.to_string(),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Error)]
pub enum HeaderParseError {
    #[error("No data found")]
    Empty,

    #[error("Invalid encoding {0}")]
    InvalidEncoding(String),

    #[error("Invalid FileType {0}")]
    InvalidFileType(String),

    #[error("Invalid fixed header")]
    InvalidFixedHeader,

    #[error("Invalid header size, not long enough, expected {HEADER_LENGTH} bytes, got {0}")]
    InvalidHeaderSize(u16),

    #[error("Invalid header parts {0}")]
    InvalidHeaderParts(String),
}

impl From<bbqr::header::HeaderParseError> for HeaderParseError {
    fn from(error: bbqr::header::HeaderParseError) -> Self {
        match error {
            bbqr::header::HeaderParseError::Empty => Self::Empty,
            bbqr::header::HeaderParseError::InvalidEncoding(encoding) => {
                Self::InvalidEncoding(encoding.to_string())
            }
            bbqr::header::HeaderParseError::InvalidFileType(file_type) => {
                Self::InvalidFileType(file_type.to_string())
            }
            bbqr::header::HeaderParseError::InvalidFixedHeader => Self::InvalidFixedHeader,
            bbqr::header::HeaderParseError::InvalidHeaderSize(size) => {
                Self::InvalidHeaderSize(size as u16)
            }
            bbqr::header::HeaderParseError::InvalidHeaderParts(parts) => {
                Self::InvalidHeaderParts(parts)
            }
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Error)]
pub enum JoinError {
    #[error("No data found")]
    Empty,

    #[error("Conflicting/variable file type/encodings/sizes")]
    ConflictingHeaders,

    #[error("Too many parts, expected {expected}, got {got}")]
    TooManyParts { expected: u16, got: u16 },

    #[error("Duplicated part index {index} has wrong content")]
    DuplicatePartWrongContent { index: u16 },

    #[error("Part with index {index} has no data")]
    PartWithNoData { index: u16 },

    #[error("Missing part, with index {index}")]
    MissingPart { index: u16 },

    #[error(transparent)]
    HeaderParseError { error: HeaderParseError },

    #[error(transparent)]
    DecodeError { error: DecodeError },
}

impl From<bbqr::join::JoinError> for JoinError {
    fn from(error: bbqr::join::JoinError) -> Self {
        match error {
            bbqr::join::JoinError::Empty => Self::Empty,
            bbqr::join::JoinError::ConflictingHeaders => Self::ConflictingHeaders,
            bbqr::join::JoinError::TooManyParts(expected, got) => Self::TooManyParts {
                expected: expected as u16,
                got: got as u16,
            },
            bbqr::join::JoinError::DuplicatePartWrongContent(index) => {
                Self::DuplicatePartWrongContent {
                    index: index as u16,
                }
            }
            bbqr::join::JoinError::PartWithNoData(index) => Self::PartWithNoData {
                index: index as u16,
            },
            bbqr::join::JoinError::MissingPart(index) => Self::MissingPart {
                index: index as u16,
            },
            bbqr::join::JoinError::HeaderParseError(error) => Self::HeaderParseError {
                error: error.into(),
            },
            bbqr::join::JoinError::DecodeError(error) => Self::DecodeError {
                error: error.into(),
            },
        }
    }
}

impl From<bbqr::continuous_join::ContinuousJoinError> for JoinError {
    fn from(value: bbqr::continuous_join::ContinuousJoinError) -> Self {
        match value {
            bbqr::continuous_join::ContinuousJoinError::HeaderParseError(error) => {
                JoinError::HeaderParseError {
                    error: error.into(),
                }
            }
            bbqr::continuous_join::ContinuousJoinError::JoinError(error) => error.into(),
            bbqr::continuous_join::ContinuousJoinError::DecodeError(error) => {
                JoinError::DecodeError {
                    error: error.into(),
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Error)]
pub enum DecodeError {
    #[error("Unable to decode hex part: {0}, error: {1}")]
    UnableToDecodeHex(u16, String),

    #[error("Unable to decode base32 part: {0}, error: {1}")]
    UnableToDecodeBase32(u16, String),

    #[error("Unable decompress zlib data: {0}")]
    UnableToInflateZlib(String),
}

impl From<bbqr::decode::DecodeError> for DecodeError {
    fn from(error: bbqr::decode::DecodeError) -> Self {
        match error {
            bbqr::decode::DecodeError::UnableToDecodeHex(index, error) => {
                Self::UnableToDecodeHex(index as u16, error.to_string())
            }
            bbqr::decode::DecodeError::UnableToDecodeBase32(index, error) => {
                Self::UnableToDecodeBase32(index as u16, error.to_string())
            }
            bbqr::decode::DecodeError::UnableToInflateZlib(error) => {
                Self::UnableToInflateZlib(error)
            }
        }
    }
}
