use std::collections::HashMap;

use crate::db::Node;

pub struct Provider {
    notables_zh_idx: HashMap<String, Node>,
    keystones_zh_idx: HashMap<String, Node>,
    ascendants_zh_idx: HashMap<String, Node>,
}

impl Provider {
    pub fn new(notables: Vec<Node>, keystones: Vec<Node>, ascendants: Vec<Node>) -> Provider {
        let mut notables_zh_idx: HashMap<String, Node> = HashMap::new();
        let mut keystones_zh_idx: HashMap<String, Node> = HashMap::new();
        let mut ascendants_zh_idx: HashMap<String, Node> = HashMap::new();

        for node in notables {
            notables_zh_idx.insert(node.zh.clone(), node);
        }

        for node in keystones {
            keystones_zh_idx.insert(node.zh.clone(), node);
        }

        for node in ascendants {
            ascendants_zh_idx.insert(node.zh.clone(), node);
        }

        Provider {
            notables_zh_idx,
            keystones_zh_idx,
            ascendants_zh_idx,
        }
    }

    pub fn provide_notable_by_zh(&self, zh: &str) -> Option<&Node> {
        self.notables_zh_idx.get(zh)
    }

    pub fn provide_keystone_by_zh(&self, zh: &str) -> Option<&Node> {
        self.keystones_zh_idx.get(zh)
    }

    pub fn provide_ascendant_by_zh(&self, zh: &str) -> Option<&Node> {
        self.ascendants_zh_idx.get(zh)
    }
}
