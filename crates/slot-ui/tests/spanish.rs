use slot_i18n::Language;
use slot_ui::{menu_face, quick_label_face, text, QuickRow};

#[test]
fn spanish_ui_renders_accents_and_fits_the_screen() {
    let font = text::label_font().unwrap();
    for ch in "áéíóúüñÁÉÍÓÚÜÑ¿¡".chars() {
        let face = text::glyph_font(font, ch);
        assert_ne!(face.lookup_glyph_index(ch), 0, "missing glyph: {ch}");
        let (_, bitmap) = face.rasterize(ch, 24.0);
        assert!(bitmap.iter().any(|a| *a != 0), "empty glyph: {ch}");
    }
    let catalog = include_str!("../../slot-i18n/locales/es.tsv");
    for (_, value) in catalog
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
    {
        for ch in value.chars().filter(|c| !c.is_whitespace()) {
            assert_ne!(
                text::glyph_font(font, ch).lookup_glyph_index(ch),
                0,
                "{ch}: {value}"
            );
        }
    }
    slot_i18n::set_language(Language::Spanish);
    let timestamp = slot_store::parse_stamp("2026-10-28_15-05-00").unwrap();
    let picker = slot_ui::ClockPicker::new(timestamp, slot_store::Zone::default(), false);
    assert_eq!(picker.value_text(slot_ui::ClockRow::Date), "28 OCT 2026");
    for row in QuickRow::ALL {
        let face = quick_label_face(row, true);
        assert!(face.w <= 640, "{row:?}: width {}", face.w);
    }
    let prompt = menu_face("Reset all shelf keys?");
    assert!(prompt.w <= 608, "prompt width {}", prompt.w);
    assert_eq!(
        slot_i18n::format("{index} of {total}", &[("index", "2"), ("total", "5")]),
        "2 de 5"
    );
    slot_i18n::set_language(Language::English);
}
