use uniffi::{Enum, Record};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum FileType {
    Psbt,
    Transaction,
    Json,
    Cbor,
    UnicodeText,
}

#[derive(Debug, Clone, Record)]
pub struct SplitOptions {
    pub encoding: Encoding,
    pub min_split_number: u16,
    pub max_split_number: u16,
    pub min_version: Version,
    pub max_version: Version,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum Encoding {
    Hex,
    Base32,
    Zlib,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Enum)]
pub enum Version {
    V01 = 0,
    V02 = 1,
    V03 = 2,
    V04 = 3,
    V05 = 4,
    V06 = 5,
    V07 = 6,
    V08 = 7,
    V09 = 8,
    V10 = 9,
    V11 = 10,
    V12 = 11,
    V13 = 12,
    V14 = 13,
    V15 = 14,
    V16 = 15,
    V17 = 16,
    V18 = 17,
    V19 = 18,
    V20 = 19,
    V21 = 20,
    V22 = 21,
    V23 = 22,
    V24 = 23,
    V25 = 24,
    V26 = 25,
    V27 = 26,
    V28 = 27,
    V29 = 28,
    V30 = 29,
    V31 = 30,
    V32 = 31,
    V33 = 32,
    V34 = 33,
    V35 = 34,
    V36 = 35,
    V37 = 36,
    V38 = 37,
    V39 = 38,
    V40 = 39,
}

impl From<bbqr::file_type::FileType> for FileType {
    fn from(file_type: bbqr::file_type::FileType) -> Self {
        match file_type {
            bbqr::file_type::FileType::Psbt => Self::Psbt,
            bbqr::file_type::FileType::Transaction => Self::Transaction,
            bbqr::file_type::FileType::Json => Self::Json,
            bbqr::file_type::FileType::Cbor => Self::Cbor,
            bbqr::file_type::FileType::UnicodeText => Self::UnicodeText,
        }
    }
}

impl From<FileType> for bbqr::file_type::FileType {
    fn from(file_type: FileType) -> Self {
        match file_type {
            FileType::Psbt => Self::Psbt,
            FileType::Transaction => Self::Transaction,
            FileType::Json => Self::Json,
            FileType::Cbor => Self::Cbor,
            FileType::UnicodeText => Self::UnicodeText,
        }
    }
}

impl From<Encoding> for bbqr::encode::Encoding {
    fn from(encoding: Encoding) -> Self {
        match encoding {
            Encoding::Hex => Self::Hex,
            Encoding::Base32 => Self::Base32,
            Encoding::Zlib => Self::Zlib,
        }
    }
}

impl From<bbqr::encode::Encoding> for Encoding {
    fn from(encoding: bbqr::encode::Encoding) -> Self {
        match encoding {
            bbqr::encode::Encoding::Hex => Self::Hex,
            bbqr::encode::Encoding::Base32 => Self::Base32,
            bbqr::encode::Encoding::Zlib => Self::Zlib,
        }
    }
}

impl From<SplitOptions> for bbqr::split::SplitOptions {
    fn from(options: SplitOptions) -> Self {
        Self {
            encoding: options.encoding.into(),
            min_split_number: options.min_split_number as usize,
            max_split_number: options.max_split_number as usize,
            min_version: options.min_version.into(),
            max_version: options.max_version.into(),
        }
    }
}

impl From<Version> for bbqr::qr::Version {
    fn from(value: Version) -> Self {
        match value {
            Version::V01 => Self::V01,
            Version::V02 => Self::V02,
            Version::V03 => Self::V03,
            Version::V04 => Self::V04,
            Version::V05 => Self::V05,
            Version::V06 => Self::V06,
            Version::V07 => Self::V07,
            Version::V08 => Self::V08,
            Version::V09 => Self::V09,
            Version::V10 => Self::V10,
            Version::V11 => Self::V11,
            Version::V12 => Self::V12,
            Version::V13 => Self::V13,
            Version::V14 => Self::V14,
            Version::V15 => Self::V15,
            Version::V16 => Self::V16,
            Version::V17 => Self::V17,
            Version::V18 => Self::V18,
            Version::V19 => Self::V19,
            Version::V20 => Self::V20,
            Version::V21 => Self::V21,
            Version::V22 => Self::V22,
            Version::V23 => Self::V23,
            Version::V24 => Self::V24,
            Version::V25 => Self::V25,
            Version::V26 => Self::V26,
            Version::V27 => Self::V27,
            Version::V28 => Self::V28,
            Version::V29 => Self::V29,
            Version::V30 => Self::V30,
            Version::V31 => Self::V31,
            Version::V32 => Self::V32,
            Version::V33 => Self::V33,
            Version::V34 => Self::V34,
            Version::V35 => Self::V35,
            Version::V36 => Self::V36,
            Version::V37 => Self::V37,
            Version::V38 => Self::V38,
            Version::V39 => Self::V39,
            Version::V40 => Self::V40,
        }
    }
}

impl From<bbqr::qr::Version> for Version {
    fn from(value: bbqr::qr::Version) -> Self {
        use bbqr::qr::Version;

        match value {
            Version::V01 => Self::V01,
            Version::V02 => Self::V02,
            Version::V03 => Self::V03,
            Version::V04 => Self::V04,
            Version::V05 => Self::V05,
            Version::V06 => Self::V06,
            Version::V07 => Self::V07,
            Version::V08 => Self::V08,
            Version::V09 => Self::V09,
            Version::V10 => Self::V10,
            Version::V11 => Self::V11,
            Version::V12 => Self::V12,
            Version::V13 => Self::V13,
            Version::V14 => Self::V14,
            Version::V15 => Self::V15,
            Version::V16 => Self::V16,
            Version::V17 => Self::V17,
            Version::V18 => Self::V18,
            Version::V19 => Self::V19,
            Version::V20 => Self::V20,
            Version::V21 => Self::V21,
            Version::V22 => Self::V22,
            Version::V23 => Self::V23,
            Version::V24 => Self::V24,
            Version::V25 => Self::V25,
            Version::V26 => Self::V26,
            Version::V27 => Self::V27,
            Version::V28 => Self::V28,
            Version::V29 => Self::V29,
            Version::V30 => Self::V30,
            Version::V31 => Self::V31,
            Version::V32 => Self::V32,
            Version::V33 => Self::V33,
            Version::V34 => Self::V34,
            Version::V35 => Self::V35,
            Version::V36 => Self::V36,
            Version::V37 => Self::V37,
            Version::V38 => Self::V38,
            Version::V39 => Self::V39,
            Version::V40 => Self::V40,
        }
    }
}
