// btleplug Source Code File
//
// Copyright 2020 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.
//
// Some portions of this file are taken and/or modified from blurmac
// (https://github.com/servo/devices), using a BSD 3-Clause license under the
// following copyright:
//
// Copyright (c) 2017 Akos Kiss.
//
// Licensed under the BSD 3-Clause License
// <LICENSE.md or https://opensource.org/licenses/BSD-3-Clause>.
// This file may not be copied, modified, or distributed except
// according to those terms.

use objc2::rc::Retained;
use objc2_core_bluetooth::CBUUID;
use objc2_foundation::NSData;
use uuid::Uuid;

use crate::api::bleuuid::{uuid_from_u16, uuid_from_u32};

/// Convert a CBUUID object to the standard Uuid type.
///
/// `CBUUID::data` is 2, 4, or 16 bytes (16-/32-/128-bit BLE UUID). Short forms
/// are expanded with the Bluetooth Base UUID.
pub fn cbuuid_to_uuid(cbuuid: &CBUUID) -> Uuid {
    let data = unsafe { cbuuid.data() };
    let bytes = data.bytes();
    match bytes.len() {
        2 => uuid_from_u16(u16::from_be_bytes(bytes.try_into().unwrap())),
        4 => uuid_from_u32(u32::from_be_bytes(bytes.try_into().unwrap())),
        16 => Uuid::from_bytes(bytes.try_into().unwrap()),
        len => panic!("unexpected CBUUID data length: {len}"),
    }
}

/// Convert a `Uuid` to a `CBUUID`.
pub fn uuid_to_cbuuid(uuid: Uuid) -> Retained<CBUUID> {
    unsafe { CBUUID::UUIDWithData(&NSData::with_bytes(uuid.as_bytes())) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cbuuid_from_bytes(bytes: &[u8]) -> Retained<CBUUID> {
        unsafe { CBUUID::UUIDWithData(&NSData::with_bytes(bytes)) }
    }

    #[test]
    fn parse_uuid_short() {
        let cbuuid = cbuuid_from_bytes(&[0x12, 0x34]);
        let uuid = cbuuid_to_uuid(&*cbuuid);
        assert_eq!(
            uuid,
            Uuid::from_u128(0x00001234_0000_1000_8000_00805f9b34fb)
        );
    }

    #[test]
    fn parse_uuid_32bit() {
        let cbuuid = cbuuid_from_bytes(&[0xab, 0xcd, 0x12, 0x34]);
        let uuid = cbuuid_to_uuid(&*cbuuid);
        assert_eq!(
            uuid,
            Uuid::from_u128(0xabcd1234_0000_1000_8000_00805f9b34fb)
        );
    }

    #[test]
    fn parse_uuid_long() {
        let cbuuid = cbuuid_from_bytes(&[
            0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x11, 0x11, 0x22, 0x22, 0x33, 0x33, 0x44, 0x44,
            0x55, 0x55,
        ]);
        let uuid = cbuuid_to_uuid(&*cbuuid);
        assert_eq!(
            uuid,
            Uuid::from_u128(0x12345678_0000_1111_2222_333344445555)
        );
    }

    #[test]
    fn cbuuid_roundtrip() {
        for uuid in [
            Uuid::from_u128(0x00001234_0000_1000_8000_00805f9b34fb),
            Uuid::from_u128(0xabcd1234_0000_1000_8000_00805f9b34fb),
            Uuid::from_u128(0x12345678_0000_1111_2222_333344445555),
        ] {
            assert_eq!(cbuuid_to_uuid(&*uuid_to_cbuuid(uuid)), uuid);
        }
    }
}
