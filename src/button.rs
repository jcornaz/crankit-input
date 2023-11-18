/// State of the playdate buttons
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct State {
    /// Buttons currently being pressed
    pub current: Set,
    /// Buttons that have just started to be pressed
    ///
    /// Meaning they were not pressed last frame, and are now currently pressed
    pub pushed: Set,
    /// Buttons that have just been released
    ///
    /// Meaning they were pressed last frame, and are no longer pressed
    pub released: Set,
}

impl State {
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
    pub fn is_any_pressed(&self, buttons: Set) -> bool {
        self.current.contains_any(buttons)
    }

    /// Returns true if any of the given button was just pressed
    #[inline]
    #[must_use]
    pub fn is_any_just_pressed(&self, buttons: Set) -> bool {
        self.pushed.contains_any(buttons)
    }

    /// Returns true if any of the given button was just released
    #[inline]
    #[must_use]
    pub fn is_any_just_released(&self, buttons: Set) -> bool {
        self.released.contains_any(buttons)
    }

    /// Returns the currently pressed state of the d-pad as a 2d vector
    ///
    /// See [`Self::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad<T: From<i8>>(self) -> [T; 2] {
        self.current.d_pad()
    }

    /// Returns the buttons of the d-pad that have just started to be pressed as a 2d vector
    ///
    /// See [`Self::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad_just_pressed<T: From<i8>>(self) -> [T; 2] {
        self.pushed.d_pad()
    }

    /// Returns the buttons of the d-pad that have just been released as a 2d vector
    ///
    /// See [`Self::d_pad`] for more details
    #[inline]
    #[must_use]
    pub fn d_pad_just_released<T: From<i8>>(self) -> [T; 2] {
        self.released.d_pad()
    }
}

/// Set of [`Button`]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Set(pub(crate) u8);

impl Set {
    /// The set of 4 D-Pad buttons (up, down, left, right)
    #[allow(clippy::cast_possible_truncation)]
    pub const D_PAD: Self =
        Self(Button::Left as u8 | Button::Right as u8 | Button::Up as u8 | Button::Down as u8);

    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, button: Button) {
        self.0 |= Set::from(button).0;
    }

    #[inline]
    #[must_use]
    pub fn contains(self, button: Button) -> bool {
        self.contains_any(button.into())
    }

    #[inline]
    #[must_use]
    pub fn contains_any(self, buttons: Set) -> bool {
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

impl Extend<Button> for Set {
    fn extend<T: IntoIterator<Item = Button>>(&mut self, iter: T) {
        iter.into_iter().for_each(|b| self.insert(b));
    }
}

impl FromIterator<Button> for Set {
    fn from_iter<T: IntoIterator<Item = Button>>(iter: T) -> Self {
        let mut result = Self::default();
        result.extend(iter);
        result
    }
}

impl From<&[Button]> for Set {
    fn from(value: &[Button]) -> Self {
        value.iter().copied().collect()
    }
}

impl<const N: usize> From<[Button; N]> for Set {
    fn from(value: [Button; N]) -> Self {
        value.into_iter().collect()
    }
}

impl From<Button> for Set {
    fn from(value: Button) -> Self {
        Self(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Button::A, Button::A, true)]
    #[case(Button::A, Button::B, false)]
    #[case(Button::B, Button::A, false)]
    #[case(Button::B, Button::B, true)]
    #[case([Button::A, Button::B], Button::B, true)]
    #[case([Button::A, Button::B], Button::A, true)]
    #[case([Button::A, Button::B], Button::Up, false)]
    #[case([Button::A, Button::B, Button::Up], Button::Up, true)]
    fn test_set_contains(
        #[case] set: impl Into<Set>,
        #[case] button: Button,
        #[case] expected: bool,
    ) {
        let set = set.into();
        assert_eq!(set.contains(button), expected);
        assert_eq!(set.contains_any(button.into()), expected);
    }

    #[rstest]
    #[case(Set::default(), Set::from_iter([Button::A]), false)]
    #[case(Set::default(), Set::from_iter([Button::A, Button::B]), false)]
    #[case(Set::default(), Set::default(), false)]
    #[case(Set::from_iter([Button::A]), Set::default(), false)]
    #[case(Set::from_iter([Button::A]), Set::from_iter([Button::A]), true)]
    #[case(Set::from_iter([Button::A, Button::B]), Set::from_iter([Button::A]), true)]
    #[case(Set::from_iter([Button::A, Button::B]), Set::from_iter([Button::A, Button::B]), true)]
    #[case(Set::from_iter([Button::A]), Set::from_iter([Button::A, Button::B]), true)]
    fn test_set_contains_any(#[case] set: Set, #[case] buttons: Set, #[case] expected: bool) {
        assert_eq!(set.contains_any(buttons), expected);
    }

    #[rstest]
    #[case(Set::default(), [0, 0])]
    #[case([Button::Up], [0, -1])]
    #[case([Button::Down], [0, 1])]
    #[case([Button::Left], [-1, 0])]
    #[case([Button::Right], [1, 0])]
    #[case([Button::Right, Button::Down, Button::Up], [1, 0])]
    #[case([Button::Left, Button::Right, Button::Up], [0, -1])]
    #[case([Button::Left, Button::Right, Button::Up, Button::Down], [0, 0])]
    fn d_pad_vector(#[case] set: impl Into<Set>, #[case] expected: [i8; 2]) {
        let set = set.into();
        assert_eq!(set.d_pad::<i8>(), expected);
        assert_eq!(set.d_pad::<i32>(), [expected[0].into(), expected[1].into()]);
        let _: [f32; 2] = set.d_pad::<f32>();
    }
}

/// A button on the playdate
#[repr(u8)]
#[allow(clippy::exhaustive_enums)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Button {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
    B = 16,
    A = 32,
}
