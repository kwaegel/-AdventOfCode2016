
//use std::collections::Vec;
use std::collections::HashMap;


pub struct KeyDatabase {
    // Mapping char key to vector of indices
    keys: HashMap<char, Vec<u32>>

    // Mapping char key to vector of indices
    //keys: [Vec<u32>; 256]
}

impl KeyDatabase {

    pub fn new() -> KeyDatabase {
        KeyDatabase{keys: HashMap::new() }
        //KeyDatabase{keys: [Vec; 256] }
    }

    pub fn insert(&mut self, key: char, index: u32) {
        let index_list = self.keys.entry(key).or_insert(Vec::new());
        index_list.push(index);
    }

//    pub fn insert_all(&mut self, keys: Vec<char>, index: u32) {
//        for key in keys {
//            self.insert(key, index);
//        }
//    }

    // Check if the database contains 'key' within the index range [lower, upper]
    pub fn contains_in_range(&mut self, key: char, lower: u32, upper: u32) -> Option<u32> {
        let index_list = self.keys.entry(key).or_insert(Vec::new());
        for &mut index in index_list {
            if index >= lower && index <= upper {
                return Some(index);
            }
        }
        None
    }
}
