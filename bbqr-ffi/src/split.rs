use bbqr::split::Split as CoreSplit;

use crate::{
    error::SplitError,
    types::{Encoding, FileType, SplitOptions, Version},
};

#[derive(uniffi::Object)]
pub struct Split(CoreSplit);

#[uniffi::export]
impl Split {
    #[uniffi::constructor]
    pub fn try_from_data(
        bytes: &[u8],
        file_type: FileType,
        options: SplitOptions,
    ) -> Result<Self, SplitError> {
        let split = CoreSplit::try_from_data(bytes, file_type.into(), options.into())?;
        Ok(Self(split))
    }

    pub fn version(&self) -> Version {
        self.0.version.into()
    }

    pub fn parts(&self) -> Vec<String> {
        self.0.parts.clone()
    }

    pub fn encoding(&self) -> Encoding {
        self.0.encoding.into()
    }
}
