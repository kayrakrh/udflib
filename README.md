# udflib

> [!WARNING]
> **Bu proje resmî bir UYAP veya Adalet Bakanlığı uygulaması değildir.** Herhangi bir kamu kurumuyla resmî bir bağlantısı bulunmamaktadır.

> [!NOTE]
> Bu projenin geliştirilmesinde yapay zekâ araçlarından yararlanılmıştır.

UYAP `.udf` dosyalarını okuyan, elle yazılmış Rust kütüphanesi. Yalnızca **veri sağlar**
(belge modeli, özellikler, imza baytları); çizim/yerleşim yapmaz. Grafik uygulaması: `../udf-okuyucu`.

```
cargo test
cargo run --bin udfdump -- tests/fixtures/sample.udf          # okunur döküm
cargo run --bin udfdump -- --json tests/fixtures/sample.udf   # makine okunur (udf-okuyucu bunu kullanır)

make && sudo make install     # /usr/bin/udfdump (PREFIX=/usr/local ile başka yere de kurulur)
```

`udfdump --json` çıktısı `{"schema":1,"document":{...}}` biçimindedir; `udflib::from_json` okur.
İmza baytları JSON'a dahil edilmez. Şema değişirse `JSON_SCHEMA` artar ve eski okuyucular açık hata verir.

```rust
let doc = udflib::open("evrak.udf")?;
for block in &doc.blocks { /* Block::Paragraph | Block::Table */ }
```

## Formatın gözlenen yapısı (tek örnek dosyadan)

- `.udf` = ZIP: `content.xml`, `documentproperties.xml` (Java Properties XML), `sign.sgn` (DER, CMS SignedData).
- `content.xml`: `<content><![CDATA[...]]>` tüm düz metin; `<elements>` içindeki `paragraph`/`table>row>cell`
  ve `content|field|space` düğümleri bu metne `startOffset` + `length` ile bakar.
  **Ofsetler UTF-16 kod birimi** (Java String). Paragrafın son karakteri `\n`'dir.
- Stiller: öğenin `resolver` özniteliği `<styles>` içindeki bir stile işaret eder; stiller `resolver` ile
  zincirlenir, sonda `default`. Öğedeki öznitelik stili ezer.
- Birimler pt. Renk: Java işaretli ARGB. `Alignment`: 0 sol, 1 orta, 2 sağ, 3 iki yana.

## Doğrulanmamış varsayımlar

- `mediaSizeName="1"`, `paperOrientation="1"` → A4 dikey sayılıyor.
- Resim, üst/alt bilgi, sayfa sonu öğeleri örnekte yok; `Document::warnings` tanınmayanları listeler.
- `TabSet` çoklu durak ayırıcısı, `Hanging`, `LineSpacing` anlamları.
- `sign.sgn` doğrulaması yok (yalnızca ham bayt).
