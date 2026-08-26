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

7. **Android: `platform::init` / `init_from_raw`** — Litten shares `jni = "=0.22.4"`
   with this crate (lockstep pin) and calls `init(&mut Env)`. `init_from_raw(*mut c_void)`
   remains for hosts that cannot share the `Env` type.

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

12. **objc2 0.6** — Apple CoreBluetooth stack is `objc2` 0.6.4 +
    `objc2-foundation` / `objc2-core-bluetooth` 0.3.2. `CentralDelegate` uses
    `define_class!`; `CBCentralManager` is initialized with `dispatch2::DispatchQueue`.
    `apple_util` types are 0.6 (`CBUUID` / `Retained`). Android / JNI is unchanged.
