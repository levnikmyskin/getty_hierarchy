use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub const BINCODED_HIERARCHY: &'static str = "bincoded_hierarchy.bin";
pub const BINCODED_TERMMAP: &'static str = "bincoded_termmap.bin";

pub fn load_nt_map_from_bytes<T: DeserializeOwned>(map_bytes: &[u8]) -> HashMap<u32, T> {
    bincode::deserialize(map_bytes).expect("Could not load hierarchy map. Please regenerate it and use Hierarchy.from_custom_pickled method ")
}

pub fn load_nt_map_from_file<T: DeserializeOwned>(path: &str) -> HashMap<u32, T> {
    if let Ok(binfile) = File::open(path) {
        let reader = BufReader::new(binfile);
        bincode::deserialize_from(reader).expect("Could not load hierarchy map from pickled file. Please make sure the path provided is correct or regenerate the pickled binary file")
    } else {
        println!("WARNING: the '{}' path does not exist and won't be parsed into a map. If this is expected (eg. you didn't want the hierarchy or term map), you can ignore this message", path);
        HashMap::new()
    }
}
