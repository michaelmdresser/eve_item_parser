use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    // We do the mapping in this direction because EVE has items with
    // dupe names but different codes. The iteration order over the CSV
    // should ensure that the highest ID associated with a given name
    // will be used.
    let mut mapping_name_to_id: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    let mut sde_reader = csv::ReaderBuilder::new()
        .from_path("data/invTypes.csv")
        .unwrap();

    for result in sde_reader.records() {
        let record = result.unwrap();
        mapping_name_to_id.insert(record[2].to_string(), record[0].to_string());
    }

    let mut builder_code_to_item = phf_codegen::Map::new();
    let mut builder_item_to_code = phf_codegen::Map::new();
    for (name, id) in mapping_name_to_id.iter() {
        let id_parsed: u64 = id.parse().unwrap();
        builder_code_to_item.entry(id_parsed, &format!("r#\"{}\"#", name));
        builder_item_to_code.entry(name, &format!("{}", id_parsed));
    }

    write!(
        &mut file,
        "static ITEM_TO_CODE: phf::Map<&'static str, u64> = {}",
        builder_item_to_code.build(),
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();

    write!(
        &mut file,
        "static CODE_TO_ITEM: phf::Map<u64, &'static str> = {}",
        builder_code_to_item.build(),
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}
