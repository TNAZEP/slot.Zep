# Networking

Slot now uses the native upstream Wi-Fi worker and **Wireless → Network** menu.
Scan for a network, enter its password, and choose whether it should auto-join.
**General → Home Wi-Fi** and **Wireless → Wi-Fi** control the same saved switch.
The footer shows connection status and signal strength. Turning Wi-Fi off keeps
saved profiles but disconnects network services.

## Upgrading from the earlier fork

If `Config/wifi.conf` does not exist, Slot imports the ordered profiles in
`Config/wifi.toml` on startup. Open and WPA-PSK profiles remain supported; passwords
are converted to derived PSKs without logging them. Invalid legacy files are
left untouched and are not imported. Existing native configuration always wins.
After migration, manage networks through the device menu. Editing the old TOML
file no longer changes the active configuration.

The launcher stops the old `slot-services` daemon before starting the frontend.
The native worker and `System/slotlink.sh` own the radio. Legacy service commands
should not be used while the native frontend is running. Credentials remain on
the SD card; keep the card private. Network time is controlled by the clock menu.

## Multiplayer

Connect both handhelds to the same LAN, choose the same game, and use Host/Join.
The link screen indicates **HOME WI-FI** or **DIRECT LINK**. LAN discovery uses
mDNS (UDP 5353); the game connection uses TCP 7211 (or `SLOT_LINK_PORT`). Guest
networks, client isolation, or blocked multicast can prevent discovery.

An existing home connection is retained across gameplay and link menus. If no
home connection is established when linking starts, Slot uses upstream's direct
link setup. Both handhelds must use the same mode; turn Wi-Fi off on both to
force a direct link. Sleep and shutdown release the radio.

## Gamebit and updates

Upstream's Gamebit web interface and Cart Studio are available from the native
menus. Slot updates use this fork's release repository and Pages manifest;
BaseOS updates retain their upstream source. See [import details](../FORK_FEATURES.md).
