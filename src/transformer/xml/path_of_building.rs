use std::fmt::Display;

use super::{build::Build, config::Config, items::Items, skills::Skills, tree::Tree};

pub struct PathOfBuilding {
    pub build: Build,
    pub skills: Skills,
    pub tree: Tree,
    pub items: Items,
    pub config: Config,
}

impl Default for PathOfBuilding {
    fn default() -> Self {
        let build = Build::default();
        let skills = Skills::default();
        let tree = Tree::default();
        let items = Items::default();
        let config = Config::default();

        PathOfBuilding {
            build,
            skills,
            tree,
            items,
            config,
        }
    }
}

impl Display for PathOfBuilding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<PathOfBuilding>
{}
{}
{}
{}
{}
</PathOfBuilding>"#,
            self.build, self.skills, self.tree, self.items, self.config
        )
    }
}
