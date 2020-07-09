use pyo3::prelude::*;
use std::collections::HashMap;
use std::include_bytes;

mod hierarchy_map;
mod utils;
use hierarchy_map::{HierarchyMap, Parent};
use utils::{load_nt_map_from_bytes, load_nt_map_from_file};


#[pyclass(module = "getty_hierarchy")]
struct Hierarchy {
    map: HierarchyMap,
    term_map: HashMap<u32, String>,
}

#[pymethods]
impl Hierarchy {
    #[new]
    fn new(obj: &PyRawObject) {
        let map_bytes = include_bytes!("bincoded_hierarchy.bin");
        let term_bytes = include_bytes!("bincoded_termmap.bin");
        obj.init({
            Hierarchy {
                map: HierarchyMap::new(load_nt_map_from_bytes(map_bytes)),
                term_map: load_nt_map_from_bytes(term_bytes),
            }
        })
    }

    #[staticmethod]
    fn from_custom_pickled(hierarchy_pickled_path: &str, term_pickled_path: &str) -> Hierarchy {
        Hierarchy {
            map: HierarchyMap::new(load_nt_map_from_file(hierarchy_pickled_path)),
            term_map: load_nt_map_from_file(term_pickled_path),
        }
    }

    fn get_parents(&self, py: Python<'_>, node_id: u32, preferred_only: bool) -> PyResult<PyObject> {
        let mut parents = Vec::new();
        self.map.get_parents(node_id, &mut parents, preferred_only);
        let object = Parent::Nodes(parents).to_object(py);
        Ok(object)
    }

    fn get_node_label(&self, _py: Python<'_>, node_id: u32) -> String {
        if let Some(label) = self.term_map.get(&node_id) {
            return label.to_owned();
        }
        String::from("")
    }
}

#[pymodule]
fn getty_hierarchy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Hierarchy>()?;
    Ok(())
}
