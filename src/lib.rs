#![no_std]

//! An ergonomic input API for the playdate
//!
//! The trait [`InputSource`] abstract the input API.
//!
//! To get an implementation of the [`InputSource`] trait,
//! enable the cargo feature `playdate-sys-v02` and call `interop::playdate_sys_v02::PlaydateInput::from_c_api`.

mod button;
pub mod interop;

pub use button::{Button, Set as ButtonSet, State as ButtonsState};

pub trait InputSource: private::Sealed {
    /// Returns the current [`ButtonsState`]
    #[must_use]
    fn buttons_state(&self) -> ButtonsState;

    /// Returns the current position of the crank, in degrees (range from `0` to `360`).
    ///
    /// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
    #[must_use]
    fn crank_angle_deg(&self) -> f32;

    /// Returns the current position of the crank, in the radians (range from `0` to `2 * f32::consts::PI`).
    ///
    /// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
    #[must_use]
    fn crank_angle_rad(&self) -> f32 {
        self.crank_angle_deg().to_radians()
    }

    /// Returns the angle change (in degrees) of the crank since the last time this function was called.
    ///
    /// Negative values are anti-clockwise.
    #[must_use]
    fn crank_change_deg(&self) -> f32;

    /// Returns the angle change (in radians) of the crank since the last time this function was called.
    ///
    /// Negative values are anti-clockwise.
    #[must_use]
    fn crank_change_rad(&self) -> f32 {
        self.crank_angle_deg().to_radians()
    }

    /// Returns whether or not the crank is folded into the unit.
    #[must_use]
    fn is_crank_docked(&self) -> bool;
}

mod private {
    pub trait Sealed {}
}
