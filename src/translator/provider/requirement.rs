use std::collections::HashMap;

use crate::db::{Requirement, RequirementSuffix};

pub struct Provider {
    requirements_zh_idx: HashMap<String, Requirement>,
    suffixes_zh_idx: HashMap<String, RequirementSuffix>,
}

impl Provider {
    pub fn new(requirements: Vec<Requirement>, suffixes: Vec<RequirementSuffix>) -> Provider {
        let mut requirements_zh_idx: HashMap<String, Requirement> = HashMap::new();
        let mut suffixes_zh_idx: HashMap<String, RequirementSuffix> = HashMap::new();

        for r in requirements {
            requirements_zh_idx.insert(r.zh.clone(), r);
        }

        for s in suffixes {
            suffixes_zh_idx.insert(s.zh.clone(), s);
        }

        Provider {
            requirements_zh_idx,
            suffixes_zh_idx,
        }
    }

    pub fn provide_by_zh(&self, zh: &str) -> Option<&Requirement> {
        self.requirements_zh_idx.get(zh)
    }

    pub fn provide_suffix_by_zh(&self, zh: &str) -> Option<&RequirementSuffix> {
        self.suffixes_zh_idx.get(zh)
    }
}
