use std::fs::File;
use std::io::BufWriter;

mod hierarchy_map;
mod ntriple_parser;
mod utils;
use clap::{Arg, App};
use utils::{BINCODED_TERMMAP, BINCODED_HIERARCHY};

fn parse_and_save_nt_hashmap(nt_dump_path: &str, with_term_map: bool) {
    let (map, term_map) = ntriple_parser::parse_getty_nt_into_hashmap(nt_dump_path, with_term_map);
    let mut bincode_file = File::create(BINCODED_HIERARCHY).unwrap();
    let mut writer = BufWriter::new(bincode_file);
    if let Err(e) = bincode::serialize_into(writer, &map) {
        panic!("Failed to save hashmap: {}", e)
    }
    bincode_file = File::create(BINCODED_TERMMAP).unwrap();
    writer = BufWriter::new(bincode_file);
    if let Err(e) = bincode::serialize_into(writer, &term_map) {
        panic!("Failed to save termmap: {}", e)
    }
}

fn main() {
    let matches = App::new("Getty hierarchy builder")
        .version("1.0")
        .author("Alessio Molinari <alessio.molinari@isti.cnr.it>")
        .about("\nThis program will parse a getty nt triples file into two maps: one will be a hierarchy map, the other will be a term_id:label map (optional).\nFinally, these maps will be bincoded (or pickled, if you're familiar with Python) into two file .bin which you will find in your current working directory.\nThese files can be used in the Python module to quickly build the hierarchy or the term:label map.")
        .arg(Arg::with_name("NT_DUMP_FILE")
             .help("Getty NT triples dump file")
             .value_name("FILE")
             .required(true)
             .index(1))
        .arg(Arg::with_name("without term map")
             .long("no-term-map")
             .help("If set, the program won't generate the term:label map"))
        .get_matches();

    let nt_dump_path = matches.value_of("NT_DUMP_FILE").unwrap();
    parse_and_save_nt_hashmap(nt_dump_path, !matches.is_present("no-term-map"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use hierarchy_map::HierarchyMap;
    use std::collections::HashMap;
    use utils::load_nt_map_from_file;

    #[test]
    fn test_hierarchy_parents() {
        let hier = HierarchyMap::new(load_nt_map_from_file(BINCODED_HIERARCHY));
        let mut parents = Vec::new();
        hier.get_parents(300053049, &mut parents);

        let mut test_parents = "[300053043, 300229467, 300053003, 300053001, 300264090, ]";
        assert_eq!(
            hierarchy_map::Parent::Nodes(parents.clone()).to_string(),
            test_parents
        );

        // Test for multiple parents and make it so that preferred parent come first
        parents.clear();
        hier.get_parents(300073708, &mut parents);
        test_parents = "[[300055980, 300055126, 300264086, ][300389850, 300015646, 300264088, ]]";
        assert_eq!(
            hierarchy_map::Parent::Nodes(parents.clone()).to_string(),
            test_parents
        );
    }

    #[test]
    fn test_term_map() {
        let term_map: HashMap<u32, String> = load_nt_map_from_file(BINCODED_TERMMAP);
        assert_eq!("dyeing", term_map.get(&300053049).unwrap());
    }
}
