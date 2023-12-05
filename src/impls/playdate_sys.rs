use core::ptr;

use playdate_sys_v02::ffi::{PDButtons, PlaydateAPI};

use crate::{Button, ButtonSet, ButtonsState, ButtonsStateSource, CrankStateSource};

impl ButtonsStateSource for PlaydateAPI {
    fn buttons_state(&self) -> ButtonsState {
        unsafe { self.system.as_ref().unwrap().buttons_state() }
    }
}

impl CrankStateSource for PlaydateAPI {
    fn crank_angle_deg(&self) -> f32 {
        unsafe { self.system.as_ref().unwrap().crank_angle_deg() }
    }

    fn crank_change_deg(&self) -> f32 {
        unsafe { self.system.as_ref().unwrap().crank_change_deg() }
    }

    fn is_crank_docked(&self) -> bool {
        unsafe { self.system.as_ref().unwrap().is_crank_docked() }
    }
}

impl ButtonsStateSource for playdate_sys_v02::ffi::playdate_sys {
    fn buttons_state(&self) -> ButtonsState {
        let mut current = PDButtons(0);
        let mut pushed = PDButtons(0);
        let mut released = PDButtons(0);
        unsafe {
            self.getButtonState.unwrap()(
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
}

impl CrankStateSource for playdate_sys_v02::ffi::playdate_sys {
    fn crank_angle_deg(&self) -> f32 {
        unsafe { self.getCrankAngle.unwrap()() }
    }

    fn crank_change_deg(&self) -> f32 {
        unsafe { self.getCrankChange.unwrap()() }
    }

    fn is_crank_docked(&self) -> bool {
        unsafe { self.isCrankDocked.unwrap()() == 1 }
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
        Self(value as _)
    }
}

impl From<ButtonSet> for PDButtons {
    #[allow(clippy::cast_lossless)]
    fn from(value: ButtonSet) -> Self {
        Self(value.0 as _)
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
        assert_eq!(set.contains_any(button), expected);
    }
}
