#![no_std]

//! An ergonomic input API for the playdate
//!
//! The entry point is [`InputSystem`] from which it is notably possible to get a [`ButtonsState`]

mod button;
mod interop;

// Re-exports from [playdate-sys](https://crates.io/playdate-sys) of types used in the public API of this crate.
mod ffi {
    pub use playdate_sys::ffi::{playdate_sys as System, PDButtons as Buttons};
}

use core::ptr;

pub use button::{Button, Set as ButtonSet, State as ButtonsState};

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

    /// Returns the current position of the crank, in degrees (range from `0` to `360`).
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

    /// Returns the current position of the crank, in the radians (range from `0` to `2 * f32::consts::PI`).
    ///
    /// Zero is pointing up, and the value increases as the crank moves clockwise, as viewed from the right side of the device.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn crank_angle_rad(&self) -> f32 {
        self.crank_angle_deg().to_radians()
    }

    /// Returns the angle change (in degrees) of the crank since the last time this function was called.
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
