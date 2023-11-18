#![no_std]

//! An ergonomic input API for the playdate
//!
//! The traits [`ButtonsStateSource`] and [`CrankStateSource`] describes the available API.
//!
//! ## Feature flags
//!
//! * `playdate-sys-v02`: provides implementations of the input source traits for the type `ffi::playdate_sys` and `ffi::PlaydateAPI` of the crate [`playdate-sys`](https://docs.rs/playdate-sys/0.2) (version `0.2`)

mod button;
mod interop;

pub use button::{Button, Set as ButtonSet, State as ButtonsState};

/// A source of button state
///
/// It is notably implemented for for the type `ffi::playdate_sys` and `ffi::PlaydateAPI` of the crate [`playdate-sys`](https://docs.rs/playdate-sys/0.2) (require the feature flag `playdate-sys-v02`)
pub trait ButtonsStateSource {
    /// Returns the current [`ButtonsState`]
    #[must_use]
    fn buttons_state(&self) -> ButtonsState;
}

/// A source of the crank state
///
/// It is notably implemented for for the type `ffi::playdate_sys` and `ffi::PlaydateAPI` of the crate [`playdate-sys`](https://docs.rs/playdate-sys/0.2) (require the feature flag `playdate-sys-v02`)
pub trait CrankStateSource {
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
