#![no_std]

//! An ergonomic input API for the playdate
//!
//! The entry point is [`InputSystem`] from which it is notably possible to get a [`ButtonsState`]

// Re-exports from [playdate-sys](https://crates.io/playdate-sys) of types used in the public API of this crate.
mod ffi {
    pub use playdate_sys::ffi::{playdate_sys as System, PDButtons as Buttons};
}

use core::ptr;

/// Entry point to access the input system
///
/// * Instanciate it with [`InputSystem::from_c_api`]
/// * Get buttons state with [`InputSystem::buttons_state`]
/// * Get crank state with [`InputSystem::crank_angle`], [`InputSystem::crank_change`] and [`InputSystem::is_crank_docked`]
pub struct InputSystem<'a> {
    system: &'a ffi::System,
}

impl<'a> InputSystem<'a> {
    /// Create the input system from a reference to the playdate system API
    ///
    /// # Safety
    ///
    /// * The referenced api must be a valid and initialized playdate api that's safe to use for the lifetime `'a`
    ///
    #[must_use]
    pub unsafe fn from_c_api(ptr: &'a ffi::System) -> Self {
        Self { system: ptr }
    }

    /// Returns the current [`ButtonsState`]
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn buttons_state(&self) -> ButtonsState {
        let mut current = ffi::Buttons(0);
        let mut pushed = ffi::Buttons(0);
        let mut released = ffi::Buttons(0);
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

    /// Returns the current position of the crank, in the range 0-360.
    ///
    /// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
    #[must_use]
    #[deprecated = "explicitely choose the crank angle unit by calling `crank_angle_deg` or `crank_angle_rad`"]
    #[allow(clippy::missing_panics_doc)]
    pub fn crank_angle(&self) -> f32 {
        self.crank_angle_deg()
    }

    /// Returns the current position of the crank, in degrees (range from `0` to `360`).
    ///
    /// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn crank_angle_deg(&self) -> f32 {
        unsafe { self.system.getCrankAngle.unwrap()() }
    }

    /// Returns the current position of the crank, in the radians (reange from `0` to `2 * f32::consts::PI`).
    ///
    /// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn crank_angle_rad(&self) -> f32 {
        self.crank_angle_deg().to_radians()
    }

    /// Returns the angle change of the crank since the last time this function was called.
    ///
    /// Negative values are anti-clockwise.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    #[deprecated = "explicitely choose the crank change unit unit by calling `crank_change_deg` or `crank_change_rad`"]
    pub fn crank_change(&self) -> f32 {
        self.crank_change_deg()
    }

    /// Returns the angle change (in degrees) of the crank since the last time this function was called.
    ///
    /// Negative values are anti-clockwise.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn crank_change_deg(&self) -> f32 {
        unsafe { self.system.getCrankChange.unwrap()() }
    }

    /// Returns the angle change (in radians) of the crank since the last time this function was called.
    ///
    /// Negative values are anti-clockwise.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn crank_change_rad(&self) -> f32 {
        self.crank_change_deg().to_radians()
    }

    /// Returns whether or not the crank is folded into the unit.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn is_crank_docked(&self) -> bool {
        unsafe { self.system.isCrankDocked.unwrap()() == 1 }
    }
}

/// State of the playdate buttons
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ButtonsState {
    /// Buttons currently being pressed
    pub current: ButtonSet,
    /// Buttons that have just started to be pressed
    ///
    /// Meaning they were not pressed last frame, and are now currently pressed
    pub pushed: ButtonSet,
    /// Buttons that have just been released
    ///
    /// Meaning they were pressed last frame, and are no longer pressed
    pub released: ButtonSet,
}

impl ButtonsState {
    /// Returns true if the given button is currently pressed
    #[inline]
    #[must_use]
    pub fn is_pressed(self, button: Button) -> bool {
        self.current.contains(button)
    }

    /// Returns true if the given button is has just started to be pressed
    ///
    /// Meaning it was not pressed last frame, and is now currently pressed
    #[inline]
    #[must_use]
    pub fn is_just_pressed(self, button: Button) -> bool {
        self.pushed.contains(button)
    }

    /// Returns true if the given button is has just started to be pressed
    ///
    /// Meaning it was pressed last frame, and is no longer pressed
    #[inline]
    #[must_use]
    pub fn is_just_released(self, button: Button) -> bool {
        self.released.contains(button)
    }

    /// Returns true if any of the given button is currently pressed
    #[inline]
    #[must_use]
    pub fn is_any_pressed(&self, buttons: ButtonSet) -> bool {
        self.current.contains_any(buttons)
    }

    /// Returns true if any of the given button was just pressed
    #[inline]
    #[must_use]
    pub fn is_any_just_pressed(&self, buttons: ButtonSet) -> bool {
        self.pushed.contains_any(buttons)
    }

    /// Returns true if any of the given button was just released
    #[inline]
    #[must_use]
    pub fn is_any_just_released(&self, buttons: ButtonSet) -> bool {
        self.released.contains_any(buttons)
    }

    /// Returns the currently pressed state of the d-pad as a 2d vector
    ///
    /// See [`ButtonSet::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad<T: From<i8>>(self) -> [T; 2] {
        self.current.d_pad()
    }

    /// Returns the buttons of the d-pad that have just started to be pressed as a 2d vector
    ///
    /// See [`ButtonSet::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad_just_pressed<T: From<i8>>(self) -> [T; 2] {
        self.pushed.d_pad()
    }

    /// Returns the buttons of the d-pad that have just been released as a 2d vector
    ///
    /// See [`ButtonSet::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad_just_released<T: From<i8>>(self) -> [T; 2] {
        self.released.d_pad()
    }
}

/// Set of [`Button`]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct ButtonSet(u8);

impl ButtonSet {
    /// The set of 4 D-Pad buttons (up, down, left, right)
    #[allow(clippy::cast_possible_truncation)]
    pub const D_PAD: Self = Self(
        (ffi::Buttons::kButtonLeft.0
            | ffi::Buttons::kButtonUp.0
            | ffi::Buttons::kButtonRight.0
            | ffi::Buttons::kButtonDown.0) as u8,
    );

    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, button: Button) {
        self.0 |= ButtonSet::from(button).0;
    }

    #[inline]
    #[must_use]
    pub fn contains(self, button: Button) -> bool {
        self.contains_any(button.into())
    }

    #[inline]
    #[must_use]
    pub fn contains_any(self, buttons: ButtonSet) -> bool {
        (self.0 & buttons.0) > 0
    }

    /// Returns the d-pad buttons contained in this set as a 2d vector
    ///
    /// The axes correspond to the playdate screen coordinate system (`x` is right, and `y` is down):
    /// * Left is [-1, 0]
    /// * Right is [1, 0]
    /// * Down is [0, 1]
    /// * Up is [0, -1]
    ///
    /// If more than one D-Pad button is contained in the set, this method returns the sum of the vectors.
    #[must_use]
    pub fn d_pad<T: From<i8>>(self) -> [T; 2] {
        let mut x = 0;
        let mut y = 0;
        if self.contains(Button::Up) {
            y -= 1;
        }
        if self.contains(Button::Down) {
            y += 1;
        }
        if self.contains(Button::Left) {
            x -= 1;
        }
        if self.contains(Button::Right) {
            x += 1;
        }
        [x.into(), y.into()]
    }
}

impl Extend<Button> for ButtonSet {
    fn extend<T: IntoIterator<Item = Button>>(&mut self, iter: T) {
        iter.into_iter().for_each(|b| self.insert(b));
    }
}

impl FromIterator<Button> for ButtonSet {
    fn from_iter<T: IntoIterator<Item = Button>>(iter: T) -> Self {
        let mut result = Self::default();
        result.extend(iter);
        result
    }
}

impl From<&[Button]> for ButtonSet {
    fn from(value: &[Button]) -> Self {
        value.iter().copied().collect()
    }
}

impl<const N: usize> From<[Button; N]> for ButtonSet {
    fn from(value: [Button; N]) -> Self {
        value.into_iter().collect()
    }
}

impl From<ffi::Buttons> for ButtonSet {
    fn from(ffi::Buttons(bits): ffi::Buttons) -> Self {
        Self(bits.try_into().unwrap_or_default())
    }
}

impl From<Button> for ButtonSet {
    fn from(value: Button) -> Self {
        ffi::Buttons::from(value).into()
    }
}

/// A button on the playdate
#[allow(clippy::exhaustive_enums)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Button {
    Left,
    Right,
    Up,
    Down,
    A,
    B,
}

impl From<Button> for ffi::Buttons {
    fn from(value: Button) -> Self {
        match value {
            Button::Left => ffi::Buttons::kButtonLeft,
            Button::Right => ffi::Buttons::kButtonRight,
            Button::Up => ffi::Buttons::kButtonUp,
            Button::Down => ffi::Buttons::kButtonDown,
            Button::B => ffi::Buttons::kButtonB,
            Button::A => ffi::Buttons::kButtonA,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ffi::Buttons::kButtonA, Button::A, true)]
    #[case(ffi::Buttons::kButtonA, Button::B, false)]
    #[case(ffi::Buttons::kButtonB, Button::A, false)]
    #[case(ffi::Buttons::kButtonB, Button::B, true)]
    #[case(ffi::Buttons::kButtonA | ffi::Buttons::kButtonB, Button::B, true)]
    #[case(ffi::Buttons::kButtonA | ffi::Buttons::kButtonB, Button::A, true)]
    #[case(ffi::Buttons::kButtonA | ffi::Buttons::kButtonB, Button::Up, false)]
    #[case(ffi::Buttons::kButtonA | ffi::Buttons::kButtonB | ffi::Buttons::kButtonUp, Button::Up, true)]
    fn test_set_contains(
        #[case] raw_set: ffi::Buttons,
        #[case] button: Button,
        #[case] expected: bool,
    ) {
        let set: ButtonSet = ButtonSet(raw_set.0.try_into().unwrap());
        assert_eq!(set.contains(button), expected);
        assert_eq!(set.contains_any(button.into()), expected);
    }

    #[rstest]
    #[case(ButtonSet::default(), ButtonSet::from_iter([Button::A]), false)]
    #[case(ButtonSet::default(), ButtonSet::from_iter([Button::A, Button::B]), false)]
    #[case(ButtonSet::default(), ButtonSet::default(), false)]
    #[case(ButtonSet::from_iter([Button::A]), ButtonSet::default(), false)]
    #[case(ButtonSet::from_iter([Button::A]), ButtonSet::from_iter([Button::A]), true)]
    #[case(ButtonSet::from_iter([Button::A, Button::B]), ButtonSet::from_iter([Button::A]), true)]
    #[case(ButtonSet::from_iter([Button::A, Button::B]), ButtonSet::from_iter([Button::A, Button::B]), true)]
    #[case(ButtonSet::from_iter([Button::A]), ButtonSet::from_iter([Button::A, Button::B]), true)]
    fn test_set_contains_any(
        #[case] set: ButtonSet,
        #[case] buttons: ButtonSet,
        #[case] expected: bool,
    ) {
        assert_eq!(set.contains_any(buttons), expected);
    }

    #[rstest]
    #[case(ButtonSet::default(), [0, 0])]
    #[case([Button::Up], [0, -1])]
    #[case([Button::Down], [0, 1])]
    #[case([Button::Left], [-1, 0])]
    #[case([Button::Right], [1, 0])]
    #[case([Button::Right, Button::Down, Button::Up], [1, 0])]
    #[case([Button::Left, Button::Right, Button::Up], [0, -1])]
    #[case([Button::Left, Button::Right, Button::Up, Button::Down], [0, 0])]
    fn d_pad_vector(#[case] set: impl Into<ButtonSet>, #[case] expected: [i8; 2]) {
        let set = set.into();
        assert_eq!(set.d_pad::<i8>(), expected);
        assert_eq!(set.d_pad::<i32>(), [expected[0].into(), expected[1].into()]);
        let _: [f32; 2] = set.d_pad::<f32>();
    }
}
