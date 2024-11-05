use std::fmt::Display;

pub struct Build {
    pub level: i32,
    pub class_name: String,
    pub ascend_class_name: String,
}

impl Build {}

impl Default for Build {
    fn default() -> Self {
        Self {
            level: 0,
            class_name: String::from("None"),
            ascend_class_name: String::from("None"),
        }
    }
}

impl Display for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Build level="{}" className="{}" ascendClassName="{}" targetVersion="3_0" mainSocketGroup="1" viewMode="ITEMS">
</Build>"#,
            self.level, self.class_name, self.ascend_class_name,
        )
    }
}
