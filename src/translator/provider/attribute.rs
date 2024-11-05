use std::collections::HashMap;

use crate::db::Attribute;

pub struct Provider {
    zh_idx: HashMap<String, Attribute>,
}

impl Provider {
    pub fn new(attributes: Vec<Attribute>) -> Provider {
        let mut zh_idx: HashMap<String, Attribute> = HashMap::new();
        for attr in attributes {
            zh_idx.insert(attr.zh.clone(), attr);
        }

        return Provider { zh_idx };
    }

    pub fn provide_by_zh(&self, zh: &str) -> Option<&Attribute> {
        return self.zh_idx.get(zh);
    }
}

mod tests {
    #[test]
    fn test_new() {
        use super::Provider;
        use crate::db::Attribute;

        let attr = Attribute {
            zh: String::from("物品类别"),
            en: String::from("Item Class"),
            values: None,
        };
        let attributes = vec![attr];

        let provider = Provider::new(attributes);

        assert_eq!(provider.provide_by_zh("物品类别").unwrap().en, "Item Class");
    }
}
