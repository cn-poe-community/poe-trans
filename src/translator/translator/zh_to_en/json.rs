use std::sync::Arc;

use regex::Regex;

use crate::model::{
    items::{Item, Items},
    passive_skills::PassiveSkills,
};

use super::Basic;

pub struct Json {
    basic: Arc<Basic>,
}

impl Json {
    pub fn new(basic: Arc<Basic>) -> Json {
        return Json { basic };
    }

    const ZH_THIEF_TRINKET: &str = "赏金猎人饰品";
    const ZH_FORBIDDEN_FLESH: &str = "禁断之肉";
    const ZH_FORBIDDEN_FLAME: &str = "禁断之火";
    const ZH_PASSIVE_SKILL_ASCENDANT_ASSASSIN: &str = "暗影";
    const ZH_PASSIVE_SKILL_ASCENDANT_ASSASSIN_FIXED: &str = "暗影（贵族）";
    const ZH_CLASS_SCION: &str = "贵族";

    pub const ZH_REQUIREMENT_NAME_CLASS: &str = "职业：";

    pub fn trans_items(&self, items: &mut Items) {
        items.items.retain(|x| self.is_pob_item(x));
        items.items.iter_mut().for_each(|x| {
            self.pre_handle_item(x);
            self.trans_item(x);
        });
    }

    fn is_pob_item(&self, item: &Item) -> bool {
        // skip items in package
        if let Some(inventory_id) = &item.inventory_id {
            if inventory_id == "MainInventory" || inventory_id == "ExpandedMainInventory" {
                return false;
            }
        }

        return item.base_type != Self::ZH_THIEF_TRINKET;
    }

    fn pre_handle_item(&self, item: &mut Item) {
        if item.name == Self::ZH_FORBIDDEN_FLAME || item.name == Self::ZH_FORBIDDEN_FLESH {
            let mut matched = false;
            if let Some(requirements) = &mut item.requirements {
                for req in requirements {
                    if req.name == Self::ZH_REQUIREMENT_NAME_CLASS {
                        let value = &req.values[0].0;
                        if value != Self::ZH_CLASS_SCION {
                            matched = true;
                        }
                        break;
                    }
                }
            }
            // 职业：贵族
            if matched {
                if let Some(explicits) = &mut item.explicit_mods {
                    for mod_str in explicits {
                        if mod_str.ends_with(Self::ZH_PASSIVE_SKILL_ASCENDANT_ASSASSIN) {
                            mod_str.truncate(
                                mod_str.len() - Self::ZH_PASSIVE_SKILL_ASCENDANT_ASSASSIN.len(),
                            );
                            mod_str.push_str(Self::ZH_PASSIVE_SKILL_ASCENDANT_ASSASSIN_FIXED);
                        }
                    }
                }
            }
        }

        let re = Regex::new(r"^元素伤害(提高|降低) \d+%$").unwrap();
        if let Some(enchants) = &mut item.enchant_mods {
            for mod_str in enchants {
                if re.is_match(mod_str) {
                    mod_str.insert_str(0, "该武器的");
                }
            }
        }
    }

    fn trans_item(&self, item: &mut Item) {
        let result = self
            .basic
            .trans_name_and_base_type(&item.name, &item.base_type);
        if let Some((name, base_type)) = result {
            item.name = name;
            item.base_type = base_type;
        }

        let result = self.basic.trans_type_line(&item.type_line);
        if let Some(type_line) = result {
            item.type_line = type_line;
        }

        if let Some(reqs) = &mut item.requirements {
            for r in reqs {
                let zh_name = r.name.clone();
                let result = self.basic.trans_req_name(&r.name);
                if let Some(name) = result {
                    r.name = name;
                }

                for v in &mut r.values {
                    let (_, result) = self.basic.trans_req(&zh_name, &v.0);
                    if let Some(result) = result {
                        v.0 = result;
                    }
                }

                if let Some(suffix) = &r.suffix {
                    let result = self.basic.trans_req_suffix(suffix);
                    if let Some(result) = result {
                        r.suffix = Some(result);
                    }
                }
            }
        }

        if let Some(props) = &mut item.properties {
            for p in props {
                let zh_name = p.name.clone();
                let result = self.basic.trans_prop_name(&p.name);
                if let Some(name) = result {
                    p.name = name;
                }

                for p in &mut p.values {
                    let (_, result) = self.basic.trans_prop(&zh_name, &p.0);
                    if let Some(result) = result {
                        p.0 = result;
                    }
                }
            }
        }

        if let Some(items) = &mut item.socketed_items {
            for item in items {
                if item.abyss_jewel.is_some() {
                    self.trans_item(item);
                } else {
                    self.trans_gem(item);
                }
            }
        }

        if let Some(mods) = &mut item.enchant_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }

        if let Some(mods) = &mut item.implicit_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }

        if let Some(mods) = &mut item.explicit_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }

        if let Some(mods) = &mut item.crafted_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }

        if let Some(mods) = &mut item.utility_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }

        if let Some(mods) = &mut item.fractured_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }

        if let Some(mods) = &mut item.scourge_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }

        if let Some(mods) = &mut item.crucible_mods {
            for mod_str in mods.iter_mut() {
                let value = self.basic.trans_mod(mod_str);
                if let Some(value) = value {
                    *mod_str = value;
                }
            }
        }
    }

    fn trans_gem(&self, gem: &mut Item) {
        let result = self.basic.trans_gem(&gem.base_type);
        if let Some(base_type) = result {
            gem.base_type = base_type;
        }

        let result = self.basic.trans_gem(&gem.type_line);
        if let Some(type_line) = result {
            gem.type_line = type_line;
        }

        if let Some(hybrid) = &mut gem.hybrid {
            let result = self.basic.trans_gem(&hybrid.base_type_name);
            if let Some(name) = result {
                hybrid.base_type_name = name;
            }
        }

        let result = self.basic.trans_type_line(&gem.type_line);
        if let Some(type_line) = result {
            gem.type_line = type_line;
        }

        if let Some(props) = &mut gem.properties {
            for p in props {
                let result = self.basic.trans_gem_prop(&p.name);
                if let Some(name) = result {
                    p.name = name;
                }
            }
        }
    }

    pub fn trans_passive_skills(&self, passive_skills: &mut PassiveSkills) {
        for item in &mut passive_skills.items {
            self.trans_item(item);
        }

        for o in passive_skills.skill_overrides.values_mut() {
            if let Some(true) = o.is_keystone {
                let name = self.basic.trans_keystone(&o.name);
                if let Some(name) = name {
                    o.name = name;
                }
            } else {
                let name = self.basic.trans_base_type(&o.name);
                if let Some(name) = name {
                    o.name = name;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_translation() {
        let factory = super::super::Factory::new();
        let translator = factory.json_translator();

        let items_str = fs::read_to_string("test/items.json").unwrap();
        let mut items: Items = serde_json::from_str(&items_str).unwrap();
        translator.trans_items(&mut items);
        let serialized = serde_json::to_string(&items).unwrap();
        let _ = fs::write("test/items_rs.json", serialized);

        let skills_str = fs::read_to_string("test/passive_skills.json").unwrap();
        let mut skills: PassiveSkills = serde_json::from_str(&skills_str).unwrap();
        translator.trans_passive_skills(&mut skills);
        let serialized = serde_json::to_string(&skills).unwrap();
        let _ = fs::write("test/passive_skills_rs.json", serialized);
    }
}