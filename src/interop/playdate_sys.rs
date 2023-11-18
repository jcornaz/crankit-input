use core::ptr;

use playdate_sys::ffi::PDButtons;

use crate::{Button, ButtonSet, ButtonsState, InputSource};

/// Implementation of [`InputSource`] that calls the playdate system
///
/// Can only be used in real playdate simulator or device.
pub struct PlaydateInput<'a> {
    system: &'a playdate_sys::ffi::playdate_sys,
}

impl<'a> PlaydateInput<'a> {
    /// Create the input system from a reference to the playdate system API
    ///
    /// # Safety
    ///
    /// * The referenced api must be a valid and initialized playdate api that's safe to use for the lifetime `'a`
    ///
    #[must_use]
    pub unsafe fn from_c_api(system: &'a playdate_sys::ffi::playdate_sys) -> Self {
        Self { system }
    }
}

impl<'a> crate::private::Sealed for PlaydateInput<'a> {}

impl<'a> InputSource for PlaydateInput<'a> {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    fn buttons_state(&self) -> ButtonsState {
        let mut current = PDButtons(0);
        let mut pushed = PDButtons(0);
        let mut released = PDButtons(0);
        unsafe {
            self.system.getButtonState.unwrap()(
                ptr::addr_of_mut!(current),
                ptr::addr_of_mut!(pushed),
                ptr::addr_of_mut!(released),
            );
        }
        ButtonsState {
            current: current.into(),
            pushed: pushed.into(),
            released: released.into(),
        }
    }

    #[must_use]
    fn crank_angle_deg(&self) -> f32 {
        unsafe { self.system.getCrankAngle.unwrap()() }
    }

    #[must_use]
    fn crank_change_deg(&self) -> f32 {
        unsafe { self.system.getCrankChange.unwrap()() }
    }

    #[must_use]
    fn is_crank_docked(&self) -> bool {
        unsafe { self.system.isCrankDocked.unwrap()() == 1 }
    }
}

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
