//! Public Apple CoreBluetooth helpers (Litten soft-fork).
//!
//! Stable wrappers around internal UUID conversion used by the central backend
//! and host apps (e.g. Litten phone GATT server). Does not expose the rest of
//! `corebluetooth`.

use objc2::rc::Retained;
use objc2_core_bluetooth::CBUUID;
use objc2_foundation::NSUUID;
use uuid::Uuid;

/// Convert a `CBUUID` (2/4/16-byte) to a full [`Uuid`].
pub fn cbuuid_to_uuid(cbuuid: &CBUUID) -> Uuid {
    crate::corebluetooth::utils::core_bluetooth::cbuuid_to_uuid(cbuuid)
}

/// Convert a [`Uuid`] to a `CBUUID`.
pub fn uuid_to_cbuuid(uuid: Uuid) -> Retained<CBUUID> {
    crate::corebluetooth::utils::core_bluetooth::uuid_to_cbuuid(uuid)
}

/// Convert an `NSUUID` to a [`Uuid`].
pub fn nsuuid_to_uuid(uuid: &NSUUID) -> Uuid {
    crate::corebluetooth::utils::nsuuid_to_uuid(uuid)
}
