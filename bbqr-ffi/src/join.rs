use std::sync::Arc;
use std::sync::Mutex;

use uniffi::Enum;
use uniffi::Object;

use bbqr::continuous_join::ContinuousJoiner as CoreContinuousJoiner;

use crate::error::JoinError;
use crate::types::Encoding;
use crate::types::FileType;

#[derive(uniffi::Object)]
pub struct ContinuousJoiner(Mutex<CoreContinuousJoiner>);

#[derive(Debug, Clone, PartialEq, Eq, Object)]
pub struct Joined {
    pub encoding: bbqr::encode::Encoding,
    pub file_type: bbqr::file_type::FileType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Enum)]
pub enum ContinuousJoinResult {
    NotStarted,
    InProgress { parts_left: u16 },
    Complete { joined: Arc<Joined> },
}

impl Default for ContinuousJoiner {
    fn default() -> Self {
        Self::new()
    }
}

#[uniffi::export]
impl ContinuousJoiner {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self(Mutex::new(CoreContinuousJoiner::new()))
    }

    pub fn add_part(&self, part: String) -> Result<ContinuousJoinResult, JoinError> {
        let result = self.0.lock().unwrap().add_part(part)?;

        Ok(result.into())
    }
}

#[uniffi::export]
impl Joined {
    #[uniffi::constructor]
    pub fn try_from_parts(parts: Vec<String>) -> Result<Self, JoinError> {
        let joined = bbqr::join::Joined::try_from_parts(parts)?;
        Ok(Self::from(joined))
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding.into()
    }

    pub fn file_type(&self) -> FileType {
        self.file_type.into()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl From<bbqr::continuous_join::ContinuousJoinResult> for ContinuousJoinResult {
    fn from(result: bbqr::continuous_join::ContinuousJoinResult) -> Self {
        match result {
            bbqr::continuous_join::ContinuousJoinResult::NotStarted => Self::NotStarted,
            bbqr::continuous_join::ContinuousJoinResult::InProgress { parts_left } => {
                Self::InProgress {
                    parts_left: parts_left as u16,
                }
            }
            bbqr::continuous_join::ContinuousJoinResult::Complete(joined) => Self::Complete {
                joined: Arc::new(joined.into()),
            },
        }
    }
}

impl From<bbqr::join::Joined> for Joined {
    fn from(joined: bbqr::join::Joined) -> Self {
        Self {
            encoding: joined.encoding,
            file_type: joined.file_type,
            data: joined.data,
        }
    }
}
