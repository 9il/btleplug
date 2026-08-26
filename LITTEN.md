# btleplug soft-fork (Litten)

Rebased onto [deviceplug/btleplug](https://github.com/deviceplug/btleplug) `dev`
(includes `jni` 0.22). Original vendoring was [btleplug 0.12.0](https://crates.io/crates/btleplug/0.12.0)
(BSD-3-Clause / MIT / Apache-2.0).

## Litten patches (vs upstream `dev`)

1. **Effective ATT MTU (ULTIMATE T-013)** — CoreBluetooth `Peripheral::mtu()` used to
   stay at the BLE default (23) forever. Added `refresh_effective_mtu()` which queries
   `CBPeripheral.maximumWriteValueLengthForType(.withoutResponse) + 3` (Compose/Kable
   SoT) and stores the result in the existing `mtu` atomics.

2. **Scan duplicates (ULTIMATE T-062)** — `ScanFilter.allow_duplicates` (default
   `true`, preserving prior CB behavior). `start_discovery` passes the flag through to
   `CBCentralManagerScanOptionAllowDuplicatesKey` so LowPower/Opportunistic can turn
   duplicates off.

3. **Android: defer MTU + `Peripheral::request_mtu`** — `connect()` no longer auto-
   `requestMtu(517)` (upstream `dev` does). Callers request after DIS (Litten SoT).
   Trait method returns negotiated MTU; on Android 14+ the stack may still ask for
   517 on first request.

4. **Android: `ScanFilter.scan_mode` + `reportDelay(0)`** — Maps
   Balanced / LowLatency / LowPower / Opportunistic to `ScanSettings.SCAN_MODE_*`.

5. **Android: `TRANSPORT_LE`** — `connectGatt(..., TRANSPORT_LE)` on API ≥23 so
   dual-mode peers do not fall through to BR/EDR.

6. **Android: `PeripheralId: FromStr` + `From<BDAddr>`** — reconnect / add by MAC
   string without private constructors.

7. **Android: `platform::init_from_raw`** — hosts on `jni` 0.21 (`jni-sys` 0.3) can
   init droidplug (`jni` 0.22 / `jni-sys` 0.4) by passing `JNIEnv::get_raw()` as
   `*mut c_void`. Do not take this crate's `jni::sys::JNIEnv` — the pointer types
   do not unify across `jni-sys` major versions.

8. **`api::ATT_HEADER_BYTES`** — public ATT header size (3); used by effective MTU
   math (`payload + ATT_HEADER_BYTES`).

9. **`btleplug::apple_util`** — public re-export of `cbuuid_to_uuid` /
   `uuid_to_cbuuid` / `nsuuid_to_uuid` for host apps (e.g. Litten peripheral)
   without exposing the rest of `corebluetooth`.

10. **Android: `setLegacy(true)` on API 26+** — upstream `dev` uses `setLegacy(false)`,
    which requests extended-advertising results only and hides Bluetooth 4.x / YdS
    tracker advertisements.

11. **Android: dequeue `QueueStream` under the lock** — a second `pollNext` could both
    see a non-empty queue, then both `remove()` on `get()`; CheckJNI aborted on the
    leftover `NoSuchElementException`.

Phone apps do **not** HW-filter by exact name `"YdS"` (hub firmware still does).

Wired via `[patch.crates-io]` in Litten `crates/Cargo.toml`.
