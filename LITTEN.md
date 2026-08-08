# btleplug soft-fork (Litten)

Vendored from [btleplug 0.12.0](https://crates.io/crates/btleplug/0.12.0)
(BSD-3-Clause / MIT / Apache-2.0).

## Litten patches (vs upstream 0.12.0)

1. **Effective ATT MTU (ULTIMATE T-013)** — CoreBluetooth `Peripheral::mtu()` used to
   stay at the BLE default (23) forever. Added `refresh_effective_mtu()` which queries
   `CBPeripheral.maximumWriteValueLengthForType(.withoutResponse) + 3` (Compose/Kable
   SoT) and stores the result in the existing `mtu` atomics.

2. **Scan duplicates (ULTIMATE T-062)** — `ScanFilter.allow_duplicates` (default
   `true`, preserving prior CB behavior). `start_discovery` passes the flag through to
   `CBCentralManagerScanOptionAllowDuplicatesKey` so LowPower/Opportunistic can turn
   duplicates off.

3. **Android: defer MTU + `Peripheral::request_mtu`** — `connect()` no longer auto-
   `requestMtu(517)`. Callers request after DIS (Litten SoT). Trait method returns
   negotiated MTU; on Android 14+ the stack may still ask for 517 on first request.

4. **Android: `ScanFilter.scan_mode` + `reportDelay(0)`** — Maps
   Balanced / LowLatency / LowPower / Opportunistic to `ScanSettings.SCAN_MODE_*`.

5. **Android: `TRANSPORT_LE`** — `connectGatt(..., TRANSPORT_LE)` on API ≥23 so
   dual-mode peers do not fall through to BR/EDR.

6. **Android: `PeripheralId: FromStr` + `From<BDAddr>`** — reconnect / add by MAC
   string without private constructors.

7. **Android: `platform::init_from_raw`** — hosts on `jni` 0.21+ can init droidplug
   without sharing the 0.19 `JNIEnv` type (pass raw `JNIEnv*`).

Phone apps do **not** HW-filter by exact name `"YdS"` (hub firmware still does).

Wired via `[patch.crates-io]` in `crates/Cargo.toml` and `slint-ui/Cargo.toml`.
