//! Kullanım: udfdump [--json] [--] dosya.udf
//!   --json : makine okunur çıktı (udf-okuyucu bunu kullanır), şema: udflib::JSON_SCHEMA
use udflib::{Block, Document};

fn dump_blocks(blocks: &[Block], depth: usize) {
    let pad = "  ".repeat(depth);
    for b in blocks {
        match b {
            Block::Paragraph(p) => {
                println!("{pad}¶ [{:?}] {:?}", p.style.align, p.text());
                for r in &p.runs {
                    if r.text.is_empty() {
                        continue;
                    }
                    let tag = r.field.as_deref().map(|f| format!(" field={f}")).unwrap_or_default();
                    println!(
                        "{pad}    {:?} {} {}pt{}{}{}{}",
                        r.text,
                        r.style.family,
                        r.style.size,
                        if r.style.bold { " B" } else { "" },
                        if r.style.italic { " I" } else { "" },
                        if r.style.underline { " U" } else { "" },
                        tag
                    );
                }
            }
            Block::Table(t) => {
                println!("{pad}▦ tablo border={} satır={}", t.border, t.rows.len());
                for (ri, row) in t.rows.iter().enumerate() {
                    for (ci, cell) in row.iter().enumerate() {
                        println!("{pad}  hücre[{ri}][{ci}]");
                        dump_blocks(&cell.blocks, depth + 2);
                    }
                }
            }
        }
    }
}

fn usage() {
    eprintln!("usage: udfdump [--json] [--] <file.udf>");
}

fn main() {
    let mut json = false;
    let mut path: Option<String> = None;
    let mut only_paths = false;
    for a in std::env::args().skip(1) {
        if only_paths {
            path = Some(a);
            continue;
        }
        match a.as_str() {
            "--" => only_paths = true,
            "--json" => json = true,
            "-h" | "--help" => {
                usage();
                return;
            }
            _ => path = Some(a),
        }
    }
    let Some(path) = path else {
        usage();
        std::process::exit(2);
    };
    let doc: Document = match udflib::open(&path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };
    if json {
        match udflib::to_json(&doc) {
            Ok(s) => println!("{s}"),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    println!("format_id : {}", doc.format_id);
    println!("sayfa     : {:?}", doc.page);
    println!("özellikler: {:?}", doc.properties);
    println!("imza      : {} bayt", doc.signature.as_ref().map_or(0, Vec::len));
    println!("webID     : {:?}", doc.web_id);
    println!("---");
    dump_blocks(&doc.blocks, 0);
    if !doc.warnings.is_empty() {
        println!("--- uyarılar");
        for w in &doc.warnings {
            println!("! {w}");
        }
    }
}
