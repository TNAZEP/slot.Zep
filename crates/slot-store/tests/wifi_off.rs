use slot_store::{read_slot_state, write_slot_state, SlotState};

#[test]
fn wifi_off_round_trips_with_an_explicit_default() {
    assert!(!SlotState::default().wifi_off);
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    write_slot_state(d.path(), &SlotState::default()).unwrap();
    let path = d.path().join("Config/slot.state");
    assert!(std::fs::read_to_string(&path)
        .unwrap()
        .contains("wifi_off=0\n"));
    assert!(!read_slot_state(d.path()).wifi_off);
    let off = SlotState {
        wifi_off: true,
        ..SlotState::default()
    };
    write_slot_state(d.path(), &off).unwrap();
    assert!(std::fs::read_to_string(&path)
        .unwrap()
        .lines()
        .any(|l| l == "wifi_off=1"));
    assert!(read_slot_state(d.path()).wifi_off);
}

#[test]
fn legacy_wifi_switch_migrates_but_native_setting_wins() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir(d.path().join("Config")).unwrap();
    let path = d.path().join("Config/slot.state");
    write_slot_state(d.path(), &SlotState::default()).unwrap();
    let base = std::fs::read_to_string(&path)
        .unwrap()
        .replace("home_wifi_enabled=0\n", "")
        .replace("wifi_off=0\n", "");
    for (text, off) in [
        ("home_wifi_enabled=0\n", true),
        ("home_wifi_enabled=1\n", false),
        ("home_wifi_enabled=0\nwifi_off=0\n", false),
        ("home_wifi_enabled=1\nwifi_off=1\n", true),
    ] {
        std::fs::write(&path, format!("{base}{text}")).unwrap();
        assert_eq!(slot_store::read_slot_state(d.path()).wifi_off, off);
    }
}
