use pyo3::prelude::*;
use pyo3::types::PyList;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Parent {
    Node(u32),
    Nodes(Vec<Parent>),
}

impl Display for Parent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut s = String::new();
        self.build_string_repr(&mut s);
        write!(f, "{}", s)
    }
}

impl ToPyObject for Parent {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        return match self {
            Parent::Nodes(val) => {
                let pylist = PyList::empty(py);
                for parent in val {
                    parent.build_python_list(py, pylist);
                }
                pylist.to_object(py)
            }
            Parent::Node(val) => PyList::new(py, vec![val]).to_object(py),
        };
    }
}

impl Parent {
    fn build_string_repr(&self, string: &mut String) {
        match self {
            Parent::Node(val) => {
                string.push_str(&format!("{}, ", val));
            }
            Parent::Nodes(val) => {
                string.push('[');
                for parent in val.iter() {
                    parent.build_string_repr(string);
                }
                string.push(']');
            }
        }
    }

    fn build_python_list(&self, py: Python<'_>, pylist: &PyList) {
        match self {
            Parent::Nodes(val) => {
                let inner_list = PyList::empty(py);
                pylist.append(inner_list);
                for el in val {
                    el.build_python_list(py, inner_list);
                }
            }
            Parent::Node(val) => {
                pylist.append(val);
            }
        };
    }
}

pub struct HierarchyMap {
    map: HashMap<u32, Vec<u32>>,
}

impl HierarchyMap {
    pub fn new(map: HashMap<u32, Vec<u32>>) -> HierarchyMap {
        HierarchyMap { map }
    }

    pub fn get_parents(&self, node: u32, parents: &mut Vec<Parent>, preferred_only: bool) {
        if let Some(this_parents) = self.map.get(&node) {
            if this_parents.len() == 1 || preferred_only {
                parents.push(Parent::Node(this_parents[0]));
                self.get_parents(this_parents[0], parents, preferred_only);
            } else {
                for parent in this_parents {
                    parents.push(Parent::Nodes(vec![Parent::Node(*parent)]));

                    // AFAIK Rust doesn't allow us to cast the enum into its value without
                    // pattern matching, so we're pretty obliged to do this check here
                    if let Parent::Nodes(val) = parents.last_mut().unwrap() {
                        self.get_parents(*parent, val, preferred_only);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_nt_map_from_file;

    #[test]
    fn test_display_parent() {
        let mut v = Vec::new();
        v.push(Parent::Node(1));

        let mut v2 = Vec::new();
        v2.push(Parent::Node(2));

        let mut v3 = Vec::new();
        v3.push(Parent::Node(3));
        v3.push(Parent::Node(4));
        v2.push(Parent::Nodes(v3));

        v.push(Parent::Nodes(v2));
        let parent = Parent::Nodes(v);

        let mut s = String::new();
        parent.build_string_repr(&mut s);
        assert_eq!("[1, [2, [3, 4, ]]]", s);
    }
}

// TODO 300212545 and 300036794 are weirdly related
