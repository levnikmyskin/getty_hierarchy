#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;
use rio_api::model::NamedNode;
use rio_api::parser::TriplesParser;
use rio_turtle::{NTriplesParser, TurtleError};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(/)(\d+)").unwrap();
}

const GETTY_BROADER: NamedNode = NamedNode {
    iri: "http://vocab.getty.edu/ontology#broader",
};
const GETTY_PREFERRED_PARENT: NamedNode = NamedNode {
    iri: "http://vocab.getty.edu/ontology#broaderPreferred",
};
const GETTY_TERM: NamedNode = NamedNode {
    iri: "http://vocab.getty.edu/ontology#term",
};
const GETTY_PREFLABEL: NamedNode = NamedNode {
    iri: "http://vocab.getty.edu/ontology#prefLabelGVP",
};

pub fn get_parser(nt_dump_path: &str) -> Result<NTriplesParser<BufReader<File>>, String> {
    match File::open(nt_dump_path) {
        Ok(file) => match NTriplesParser::new(BufReader::new(file)) {
            Ok(parser) => Ok(parser),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_node_id(node: &str) -> Option<u32> {
    match RE.captures(node) {
        Some(val) => Some(val[2].parse::<u32>().unwrap()),
        None => None,
    }
}

pub fn parse_getty_nt_into_hashmap(
    nt_dump_path: &str,
    need_term_map: bool,
) -> (HashMap<u32, Vec<u32>>, HashMap<u32, String>) {
    println!("Parsing getty dump, this will take a while...");
    let mut parser: NTriplesParser<BufReader<File>>;
    match get_parser(nt_dump_path) {
        Ok(p) => parser = p,
        Err(e) => panic!(e),
    }
    let mut map: HashMap<u32, Vec<u32>> = HashMap::with_capacity(52546);
    let mut term_map: HashMap<u32, String> = HashMap::new();
    let mut temp_map: HashMap<u32, u32> = HashMap::new();
    let mut id_child = 0;
    let mut id_parent = 0;

    if need_term_map {
        term_map = HashMap::with_capacity(157855);
        temp_map = HashMap::with_capacity(52546);
    }

    while !parser.is_end() {
        parser.parse_step(&mut |t| {
                match t.predicate {
                    GETTY_BROADER => {
                        match get_node_id(t.subject.to_string().as_str()) {
                            Some(val) => id_child = val,
                            None => return Ok(()) as Result<(), TurtleError>,
                        }

                        match get_node_id(t.object.to_string().as_str()) {
                            Some(val) => id_parent = val,
                            None => return Ok(()) as Result<(), TurtleError>,
                        }
                        let val = map.entry(id_child).or_insert(Vec::new());
                        val.push(id_parent);
                    }
                    GETTY_PREFERRED_PARENT => {
                        match get_node_id(t.subject.to_string().as_str()) {
                            Some(val) => id_child = val,
                            None => return Ok(()) as Result<(), TurtleError>,
                        }

                        match get_node_id(t.object.to_string().as_str()) {
                            Some(val) => id_parent = val,
                            None => return Ok(()) as Result<(), TurtleError>,
                        }

                        let val = map.entry(id_child).or_insert(Vec::new());
                        if let Some(index) = val.iter().position(|&r| r == id_parent) {
                            let el = val.remove(index);
                            val.insert(0, el);
                        } else {
                            val.insert(0, id_parent);
                        }
                    }
                    GETTY_TERM => {
                        if need_term_map {
                            if let Some(id_term) = get_node_id(t.subject.to_string().as_str()) {
                                let object = t.object.to_string();
                                if !term_map.contains_key(&id_term) && object.ends_with("@en") {
                                    if let Some(id_label) = temp_map.get(&id_term) {
                                        term_map.insert(
                                            *id_label,
                                            object[..object.len() - 3].replace("\"", ""),
                                        );
                                    } else {
                                        term_map.insert(
                                            id_term,
                                            object[..object.len() - 3].replace("\"", ""),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    GETTY_PREFLABEL => {
                        if need_term_map {
                            if let Some(id_obj) = get_node_id(t.object.to_string().as_str()) {
                                if let Some(id_subj) = get_node_id(t.subject.to_string().as_str()) {
                                    if let Some(val) = term_map.remove(&id_obj) {
                                        term_map.insert(id_obj, val);
                                    } else {
                                        temp_map.insert(id_obj, id_subj);
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                };
                Ok(()) as Result<(), TurtleError>
            })
            .expect("Something went wrong while parsing the nt triples file");
    }

    (map, term_map)
}
