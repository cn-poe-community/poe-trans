use std::collections::HashMap;

use crate::{db::Property, translator::util::get_zh_body};

pub struct Provider {
    properties: Vec<Property>,
    zh_idx: HashMap<String, usize>,
    zh_body_idx: HashMap<String, Vec<usize>>,
}

impl Provider {
    pub fn new(properties: Vec<Property>) -> Provider {
        let mut zh_idx: HashMap<String, usize> = HashMap::new();
        let mut zh_body_idx: HashMap<String, Vec<usize>> = HashMap::new();

        for (i, prop) in properties.iter().enumerate() {
            zh_idx.insert(prop.zh.clone(), i);
            let body = get_zh_body(&prop.zh);
            match zh_body_idx.get_mut(&body) {
                Some(vec) => vec.push(i),
                None => {
                    let vec = vec![i];
                    zh_body_idx.insert(body, vec);
                }
            }
        }

        Provider {
            properties,
            zh_idx,
            zh_body_idx,
        }
    }

    pub fn provide_by_zh(&self, zh: &str) -> Option<&Property> {
        if let Some(i) = self.zh_idx.get(zh) {
            return Some(&self.properties[*i]);
        }
        None
    }

    pub fn provice_by_zh_body(&self, zh_body: &str) -> Option<Vec<&Property>> {
        if let Some(idx_list) = self.zh_body_idx.get(zh_body) {
            let mut result: Vec<&Property> = vec![];
            for i in idx_list {
                result.push(&self.properties[*i]);
            }

            return Some(result);
        }
        None
    }
}
