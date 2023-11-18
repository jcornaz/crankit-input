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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(PDButtons::kButtonA, Button::A, true)]
    #[case(PDButtons::kButtonA, Button::B, false)]
    #[case(PDButtons::kButtonB, Button::A, false)]
    #[case(PDButtons::kButtonB, Button::B, true)]
    #[case(PDButtons::kButtonA | PDButtons::kButtonB, Button::B, true)]
    #[case(PDButtons::kButtonA | PDButtons::kButtonB, Button::A, true)]
    #[case(PDButtons::kButtonA | PDButtons::kButtonB, Button::Up, false)]
    #[case(PDButtons::kButtonA | PDButtons::kButtonB | PDButtons::kButtonUp, Button::Up, true)]
    fn test_set_contains(
        #[case] raw_set: PDButtons,
        #[case] button: Button,
        #[case] expected: bool,
    ) {
        let set: ButtonSet = raw_set.into();
        assert_eq!(set.contains(button), expected);
        assert_eq!(set.contains_any(button.into()), expected);
    }
}
