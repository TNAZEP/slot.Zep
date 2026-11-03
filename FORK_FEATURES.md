# Imported features

This fork selectively ports work by Prashant Vaibhav from
[pvaibhav/slot](https://github.com/pvaibhav/slot), inspected at
`6fa0467` (2026-10-27). The original GPL-3.0-or-later license and third-party
notices remain in place. These are feature ports onto this fork's newer UI and
Japanese localization, rather than a merge of the entire source fork.

## Automatic 120 Hz

Supported RG SP, RG34XX SP v1, and RG34XX panels switch automatically at startup.
The model, panel driver, device-tree timings, and register access must all match
before a switch is attempted. Other revisions keep their existing mode. Writes
are verified, with a rollback attempted if verification fails, and the selected
mode is restored before the backlight on wake.

Emulation and audio follow the core's native timing independently of the display
refresh rate. For diagnostics, `high_refresh=0` in `Config/slot.state` disables
the automatic switch. Hardware validation is still required for each device;
software tests cannot measure input lag or prove panel operation.

## Battery life and heat

Upstream reverted `schedutil` after observing stutter on handheld hardware. This
fork follows that correction and uses the upstream performance governor. Lower
heat and longer battery life are therefore not guaranteed; reliable frame pacing
takes priority. Power consumption depends on the game, core, and enabled features.

## Home Wi-Fi

Use **Wireless → Network** to scan, join, and manage saved networks. The existing
**General → Home Wi-Fi** switch controls the same native Wi-Fi worker. Existing
`Config/wifi.toml` profiles are imported into `Config/wifi.conf` once, only when
the native file does not exist. The original file remains untouched; subsequent
changes should use the Network menu. See [network configuration](network/README.md).

The previous `slot-services` daemon is stopped during frontend startup so only
one component controls the radio. Bundles retain it for upgrade compatibility.

## Cart labels

Missing GB, GBC, and GBA labels download in the background. ROM checksums are
matched against the bundled LaunchBox index, with a title-search fallback.
Existing labels are preserved. Downloads have bounded timeouts and retries;
after repeated network failures, remaining labels are retried on the next launch.

For offline use, place a compatible `Artwork.pack` in `Labels/`. It is consulted
before network requests. The source fork distributes packs in its releases;
`task labels:pack` can also build one. Artwork is not bundled with this repository.
The pack/index generators live in `crates/slot-labels/examples`.

## Linking over home Wi-Fi

Connect both handhelds to the same home LAN and use the normal Host/Join controls
with the same game. The picker indicates **HOME WI-FI** or **DIRECT LINK**.
LAN hosts are discovered with mDNS (UDP 5353) and connect on TCP 7211. Networks
with client isolation or blocked multicast may prevent discovery. Without a home
connection, the normal direct link remains available; pending home association
is paused for the direct-link session and resumed afterwards.

## Port history and future updates

| Feature | Source commits |
| --- | --- |
| Home Wi-Fi/service | `92f812e`, `a388c4c`, `b977d34`, `3487939` |
| LAN linking | `8700a56` |
| Background labels | `12f500a`, `0387313`, `375b61a`, `bca7dcf` |
| Refresh and native pacing | `e8ff757` |
| CPU governor (already present) | `ad7ce10` |

The service and label crates are separate workspace members. Integration points
are the General menu/state, frontend label worker, link transport, panel power
backend, and device frame pacer. Future ports should preserve this fork's clock,
localization, rendering, and state-migration behavior. Achievements, cheats,
colour calibration, and power-button changes were not included in this port.

## Upstream integration (1.13)

Merged all 68 commits from `BrandonKowalski/slot` through `0b95d32`, retaining
the fork's Japanese localization, automatic 120 Hz, native-frame pacing,
automatic/offline cart labels, and LAN linking. New upstream features include
Shimmerless rendering, saturation/contrast, Boktai sunlight, reset/undo, guides,
fast-forward options, native network setup, Cart Studio, Gamebit, network time,
and Slot/BaseOS updates. The mGBA LCD fix is included as a core build patch.

Slot OTA manifests use `https://tnazep.github.io/slot.Zep/versions.json` and
download only from `TNAZEP/slot.Zep` releases. Publish this fork's release and
Pages manifest before expecting OTA checks to succeed. BaseOS updates continue
to use `pvaibhav/BaseOS`. No stock Slot update fallback is used.

Linux checks exclude macOS-only headless rendering and desktop-input targets.
Panel switching,
power consumption, audio, and two-device Wi-Fi linking still need hardware checks.
