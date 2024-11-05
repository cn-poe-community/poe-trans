use std::collections::HashMap;

use crate::db::BaseType;

pub struct Provider {
    base_types: Vec<BaseType>,
    zh_idx: HashMap<String, Vec<usize>>,
}

impl Provider {
    pub fn new(categories: Vec<Vec<BaseType>>) -> Provider {
        let mut list: Vec<BaseType> = vec![];
        let mut zh_idx: HashMap<String, Vec<usize>> = HashMap::new();

        for base_types in categories {
            for (i, b) in base_types.iter().enumerate() {
                match zh_idx.get_mut(&b.zh) {
                    Some(vec) => vec.push(i + list.len()),
                    None => {
                        let vec = vec![i + list.len()];
                        zh_idx.insert(b.zh.clone(), vec);
                    }
                }
            }
            list.extend(base_types.into_iter());
        }

        return Provider {
            base_types: list,
            zh_idx,
        };
    }

    pub fn provide_by_zh(&self, zh: &str) -> Option<Vec<&BaseType>> {
        if let Some(ids) = self.zh_idx.get(zh) {
            let result: Vec<&BaseType> = ids.iter().map(|x| &self.base_types[*x]).collect();
            return Some(result);
        }

        None
    }
}
