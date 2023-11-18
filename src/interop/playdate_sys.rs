use playdate_sys::ffi::PDButtons;

use crate::{Button, ButtonSet};

impl From<PDButtons> for ButtonSet {
    #[allow(clippy::cast_possible_truncation)]
    fn from(PDButtons(bits): PDButtons) -> Self {
        Self(bits as u8)
    }
}

impl From<Button> for PDButtons {
    fn from(value: Button) -> Self {
        Self(value as u32)
    }
}

impl From<ButtonSet> for PDButtons {
    #[allow(clippy::cast_lossless)]
    fn from(value: ButtonSet) -> Self {
        Self(value.0 as u32)
    }
}
