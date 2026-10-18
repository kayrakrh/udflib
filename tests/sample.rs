use udflib::{Align, Block, Rgba};

fn doc() -> udflib::Document {
    udflib::open(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sample.udf")).unwrap()
}

fn para(d: &udflib::Document, i: usize) -> &udflib::Paragraph {
    match &d.blocks[i] {
        Block::Paragraph(p) => p,
        other => panic!("blok {i} paragraf değil: {other:?}"),
    }
}

#[test]
fn meta() {
    let d = doc();
    assert_eq!(d.format_id, "1.8");
    assert_eq!(d.properties["uyapdogrulamakodu"], "kZ49DZgw");
    assert_eq!(d.properties["uyapsicil"], "1116150");
    // CMS SignedData: 30 82 .. .. 06 09 2A 86 48 86 F7 0D 01 07 02
    let sig = d.signature.as_ref().unwrap();
    assert_eq!(sig[0], 0x30);
    assert_eq!(&sig[4..15], &[0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02]);
    assert!(d.warnings.is_empty(), "{:?}", d.warnings);
}

#[test]
fn page_format() {
    let d = doc();
    assert!((d.page.left_margin - 42.5197).abs() < 0.01);
    assert!((d.page.bottom_margin - 14.1732).abs() < 0.01);
}

#[test]
fn paragraphs() {
    let d = doc();
    assert_eq!(d.blocks.len(), 12);
    assert_eq!(para(&d, 0).text(), "");
    assert_eq!(para(&d, 1).text(), "İLAN");
    assert_eq!(para(&d, 1).style.align, Align::Center);
    assert!(para(&d, 1).runs[0].style.bold);
    assert_eq!(para(&d, 2).text(), "GEBZE 2. ASLİYE HUKUK MAHKEMESİNDEN");
    assert_eq!(para(&d, 4).text(), "ESAS NO\t: 2026/256 Esas");
    assert_eq!(para(&d, 4).style.tabs.len(), 1);
    assert_eq!(para(&d, 7).text(), "Mahkememizin 11/09/2026 Tarihli kararı ile;");
    assert_eq!(para(&d, 7).style.align, Align::Justify);
    assert!((para(&d, 7).style.first_line_indent - 35.4375).abs() < 1e-3);
    assert!(para(&d, 7).runs[1].style.bold && para(&d, 7).runs[1].style.italic);
}

#[test]
fn field_runs() {
    let d = doc();
    let p = para(&d, 4);
    assert!(p.runs.iter().any(|r| r.field.as_deref() == Some("esasNo") && r.text == "2026/256"));
}

#[test]
fn signature_table() {
    let d = doc();
    let Block::Table(t) = &d.blocks[10] else { panic!("10. blok tablo olmalı") };
    assert_eq!(t.rows.len(), 1);
    assert_eq!(t.rows[0].len(), 4);
    let Block::Paragraph(p0) = &t.rows[0][0].blocks[0] else { panic!() };
    assert_eq!(p0.text(), "Yazı İşleri Müdürü 139210");
    let Block::Paragraph(p1) = &t.rows[0][0].blocks[1] else { panic!() };
    assert_eq!(p1.text(), "E-imzalıdır");
    assert_eq!(p1.runs[0].style.family, "Rage Italic");
    assert_eq!(p1.runs[0].style.size, 16.0);
    assert_eq!(p1.runs[0].style.foreground, Rgba(0xFD, 0, 0, 0xFF));
    assert_eq!(p1.style.align, Align::Center);
}

#[test]
fn json_roundtrip() {
    let d = doc();
    let j = udflib::to_json(&d).unwrap();
    let d2 = udflib::from_json(&j).unwrap();
    assert_eq!(d.format_id, d2.format_id);
    assert_eq!(d.page, d2.page);
    assert_eq!(d.blocks, d2.blocks);
    assert_eq!(d.properties, d2.properties);
}

#[test]
fn json_schema_mismatch() {
    assert!(udflib::from_json(r#"{"schema":999,"document":{}}"#).is_err());
}

#[test]
fn missing_file_message() {
    let err = udflib::open("/nonexistent/dir/missing.udf").unwrap_err();
    assert!(matches!(err, udflib::UdfError::FileNotFound(_)));
    assert_eq!(err.to_string(), "Could not find the selected file: /nonexistent/dir/missing.udf");
}
