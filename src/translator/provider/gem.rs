use std::collections::HashMap;

use crate::db::{Gem, Skill};

pub struct Provider {
    zh_idx: HashMap<String, Skill>,
}

impl Provider {
    pub fn new(gems: Vec<Gem>, hybrid_skills: Vec<Skill>) -> Provider {
        let mut zh_idx: HashMap<String, Skill> = HashMap::new();
        for gem in gems {
            zh_idx.insert(
                gem.zh.clone(),
                Skill {
                    zh: gem.zh,
                    en: gem.en,
                },
            );
        }

        for skill in hybrid_skills {
            zh_idx.insert(skill.zh.clone(), skill);
        }

        Provider { zh_idx }
    }

    pub fn provide_by_zh(&self, zh: &str) -> Option<&Skill> {
        self.zh_idx.get(zh)
    }
}
