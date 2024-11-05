use regex::Regex;

use crate::{
    db::{BaseType, Stat},
    translator::{
        provider::{attribute, base_type, gem, passive_skill, property, requirement, stat},
        util::{get_zh_body, stat::Template, LINE_SEPERATOR},
    },
};

/// Basic translator supports basic services.
pub struct Basic {
    attribute_provider: attribute::Provider,
    basetype_provider: base_type::Provider,
    gem_provider: gem::Provider,
    passive_skill_provider: passive_skill::Provider,
    property_provider: property::Provider,
    requirement_provider: requirement::Provider,
    stat_provider: stat::Provider,
}

impl Basic {
    pub fn new(
        attribute_provider: attribute::Provider,
        basetype_provider: base_type::Provider,
        gem_provider: gem::Provider,
        passive_skill_provider: passive_skill::Provider,
        property_provider: property::Provider,
        requirement_provider: requirement::Provider,
        stat_provider: stat::Provider,
    ) -> Basic {
        return Basic {
            attribute_provider,
            basetype_provider,
            gem_provider,
            passive_skill_provider,
            property_provider,
            requirement_provider,
            stat_provider,
        };
    }
}

/// base type
impl Basic {
    const ZH_SUPERIOR_PREFIX: &str = "精良的 ";
    //const EN_SUPERIOR_PREFIX: &str = "Superior ";
    const ZH_SYNTHESISED_PREFIX: &str = "忆境 ";
    //const EN_SYNTHESISED_PREFIX: &str = "Synthesised ";

    const EN_DEFALUT_NAME: &str = "Item";

    /// Return the first one matches name.
    fn find_base_type(&self, name: &str) -> Option<&BaseType> {
        let list = self.basetype_provider.provide_by_zh(name);
        if list.is_none() {
            return None;
        }

        let list = list.unwrap();

        return Some(list[0]);
    }

    /// Return the base type contains target unique.
    fn find_base_type_by_unique(&self, name: &str, base_type: &str) -> Option<&BaseType> {
        let list = self.basetype_provider.provide_by_zh(base_type);
        if list.is_none() {
            return None;
        }

        let list = list.unwrap();

        for b in list {
            match &b.uniques {
                None => continue,
                Some(uniques) => {
                    for unique in uniques {
                        if unique.zh == name {
                            return Some(b);
                        }
                    }
                }
            }
        }

        return None;
    }

    fn find_base_type_by_type_line(&self, type_line: &str) -> Option<&BaseType> {
        let mut type_line = type_line;
        if type_line.starts_with(Self::ZH_SUPERIOR_PREFIX) {
            type_line = &type_line[Self::ZH_SUPERIOR_PREFIX.len()..];
        }

        if type_line.starts_with(Self::ZH_SYNTHESISED_PREFIX) {
            type_line = &type_line[Self::ZH_SYNTHESISED_PREFIX.len()..];
        }

        let mut result = self.find_base_type(type_line);
        if result.is_some() {
            return result;
        }

        let re = Regex::new(r".+?[之的]").unwrap();
        re.find_iter(type_line).any(|x| {
            let possible = &type_line[x.end()..];
            result = self.find_base_type(possible);
            if result.is_some() {
                return true;
            }
            return false;
        });

        return result;
    }

    pub fn trans_name_and_base_type(
        &self,
        name: &str,
        base_type: &str,
    ) -> Option<(String, String)> {
        // check if item is unique
        if name.len() > 0 {
            let b = self.find_base_type_by_unique(name, base_type);
            if let Some(b) = b {
                if let Some(uniques) = &b.uniques {
                    for unique in uniques {
                        if unique.zh == name {
                            return Some((unique.en.clone(), b.en.clone()));
                        }
                    }
                }
            }
        }

        let b = self.find_base_type(base_type);
        if let Some(b) = b {
            let name = match name.len() {
                0 => "",
                _ => Self::EN_DEFALUT_NAME,
            };
            return Some((String::from(name), b.en.clone()));
        }

        return None;
    }

    pub fn trans_base_type(&self, base_type: &str) -> Option<String> {
        let b = self.find_base_type(base_type);
        match b {
            Some(b) => Some(b.en.clone()),
            None => None,
        }
    }

    pub fn trans_type_line(&self, type_line: &str) -> Option<String> {
        let b = self.find_base_type_by_type_line(type_line);
        if let Some(b) = b {
            return Some(b.en.clone());
        }
        return None;
    }
}

static GEM_PROPERTY_NAMES: [(&str, &str); 2] = [("等级", "Level"), ("品质", "Quality")];

/// attribute,gem,prop,requirements...
impl Basic {
    /// Translate attribute name and value.
    pub fn trans_attr(&self, name: &str, value: &str) -> (Option<String>, Option<String>) {
        let attr = self.attribute_provider.provide_by_zh(name);

        match attr {
            Some(attr) => {
                if let Some(values) = &attr.values {
                    for v in values {
                        if v.zh == value {
                            return (Some(attr.en.clone()), Some(v.en.clone()));
                        }
                    }

                    return (Some(attr.en.clone()), None);
                }
            }
            None => {}
        }

        return (None, None);
    }

    /// Translate only attribute name.
    pub fn trans_attr_name(&self, zh_name: &str) -> Option<String> {
        let attr = self.attribute_provider.provide_by_zh(zh_name);
        match attr {
            Some(attr) => Some(attr.en.clone()),
            None => None,
        }
    }

    fn fmt_gem_name(zh: &str) -> String {
        return zh.replace("(", "（").replace(")", "）");
    }

    pub fn trans_gem(&self, name: &str) -> Option<String> {
        let name = Self::fmt_gem_name(name);
        let gem = self.gem_provider.provide_by_zh(&name);
        if let Some(gem) = gem {
            return Some(gem.en.clone());
        }

        return None;
    }

    pub fn trans_gem_prop(&self, zh: &str) -> Option<String> {
        for property in GEM_PROPERTY_NAMES {
            if property.0 == zh {
                return Some(String::from(property.1));
            }
        }

        return None;
    }

    pub fn trans_notable(&self, zh: &str) -> Option<String> {
        let node = self.passive_skill_provider.provide_notable_by_zh(zh);
        return match node {
            Some(b) => Some(b.en.clone()),
            None => None,
        };
    }

    pub fn trans_keystone(&self, zh: &str) -> Option<String> {
        let node = self.passive_skill_provider.provide_keystone_by_zh(zh);
        return match node {
            Some(b) => Some(b.en.clone()),
            None => None,
        };
    }

    pub fn trans_ascendant(&self, zh: &str) -> Option<String> {
        let node = self.passive_skill_provider.provide_ascendant_by_zh(zh);
        return match node {
            Some(b) => Some(b.en.clone()),
            None => None,
        };
    }

    pub fn trans_prop(&self, name: &str, value: &str) -> (Option<String>, Option<String>) {
        let prop = self.property_provider.provide_by_zh(name);
        if prop.is_none() {
            return (None, None);
        }
        let prop = prop.unwrap();

        match &prop.values {
            Some(values) => {
                for v in values {
                    if v.zh == value {
                        return (Some(prop.en.clone()), Some(v.en.clone()));
                    }
                }
            }
            None => {}
        }

        return (Some(prop.en.clone()), None);
    }

    pub fn trans_prop_name(&self, name: &str) -> Option<String> {
        let prop = self.property_provider.provide_by_zh(name);
        if let Some(prop) = prop {
            return Some(prop.en.clone());
        }

        let props = self
            .property_provider
            .provice_by_zh_body(&&get_zh_body(name));
        if let Some(props) = props {
            for prop in props {
                let zh_tmpl = Template::new(&prop.zh);
                let params = zh_tmpl.parse_params(name);
                if params.is_none() {
                    continue;
                }
                let params = params.unwrap();
                let en_tmpl = Template::new(&prop.en);
                return Some(en_tmpl.render(params));
            }
        }

        return None;
    }

    /// Translate requirement.
    pub fn trans_req(&self, name: &str, value: &str) -> (Option<String>, Option<String>) {
        let req = self.requirement_provider.provide_by_zh(name);
        if req.is_none() {
            return (None, None);
        }
        let req = req.unwrap();

        if let Some(values) = &req.values {
            for v in values {
                if v.zh == value {
                    return (Some(req.en.clone()), Some(v.en.clone()));
                }
            }
        }

        return (Some(req.en.clone()), None);
    }

    pub fn trans_req_name(&self, zh: &str) -> Option<String> {
        let req = self.requirement_provider.provide_by_zh(zh);
        return match req {
            Some(b) => Some(b.en.clone()),
            None => None,
        };
    }

    pub fn trans_req_suffix(&self, zh: &str) -> Option<String> {
        let req = self.requirement_provider.provide_suffix_by_zh(zh);
        return match req {
            Some(b) => Some(b.en.clone()),
            None => None,
        };
    }
}

//stat
impl Basic {
    const ZH_ANOINTED_MOD_PREFIX: &str = "配置 ";
    const ZH_FORBIDDEN_FLAME_MOD_PREFIX: &str = "禁断之火上有匹配的词缀则配置 ";
    const ZH_FORBIDDEN_FLESH_MOD_PREFIX: &str = "禁断之肉上有匹配的词缀则配置 ";

    const ZH_UNIQUE_ENEMY_IN_YOUR_PRESENCE: &str = "有一个传奇怪物出现在你面前：";
    const EN_UNIQUE_ENEMY_IN_YOUR_PRESENCE: &str = "While a Unique Enemy is in your Presence, ";
    const ZH_PINNACLE_ATLAS_BOSS_IN_YOUR_PRESENCE: &str = "有一个异界图鉴最终首领出现在你面前：";
    const EN_PINNACLE_ATLAS_BOSS_IN_YOUR_PRESENCE: &str =
        "While a Pinnacle Atlas Boss is in your Presence, ";

    fn is_anointed_mod(&self, mod_str: &str) -> bool {
        return mod_str.starts_with(Self::ZH_ANOINTED_MOD_PREFIX);
    }

    fn trans_anointed_mod(&self, mod_str: &str) -> Option<String> {
        let notable = &mod_str[Self::ZH_ANOINTED_MOD_PREFIX.len()..];
        let trans = self.trans_notable(notable);

        return match trans {
            Some(trans) => Some(format!("Allocates {}", trans)),
            None => None,
        };
    }

    fn is_forbidden_flame_mod(&self, mod_str: &str) -> bool {
        return mod_str.starts_with(Self::ZH_FORBIDDEN_FLAME_MOD_PREFIX);
    }

    fn trans_forbidden_flame_mod(&self, mod_str: &str) -> Option<String> {
        let ascendant = &mod_str[Self::ZH_FORBIDDEN_FLAME_MOD_PREFIX.len()..];
        let trans = self.trans_ascendant(ascendant);

        return match trans {
            Some(trans) => Some(format!(
                "Allocates {} if you have the matching modifier on Forbidden Flame",
                trans
            )),
            None => None,
        };
    }

    fn is_forbidden_flesh_mod(&self, mod_str: &str) -> bool {
        return mod_str.starts_with(Self::ZH_FORBIDDEN_FLESH_MOD_PREFIX);
    }

    fn trans_forbidden_flesh_mod(&self, mod_str: &str) -> Option<String> {
        let ascendant = &mod_str[Self::ZH_FORBIDDEN_FLESH_MOD_PREFIX.len()..];
        let trans = self.trans_ascendant(ascendant);

        return match trans {
            Some(trans) => Some(format!(
                "Allocates {} if you have the matching modifier on Forbidden Flesh",
                trans
            )),
            None => None,
        };
    }

    fn is_eldritch_implicit_mod(&self, mod_str: &str) -> bool {
        return mod_str.starts_with(Self::ZH_UNIQUE_ENEMY_IN_YOUR_PRESENCE)
            || mod_str.starts_with(Self::ZH_PINNACLE_ATLAS_BOSS_IN_YOUR_PRESENCE);
    }

    fn trans_eldritch_implicit_mod(&self, mod_str: &str) -> Option<String> {
        if mod_str.starts_with(Self::ZH_UNIQUE_ENEMY_IN_YOUR_PRESENCE) {
            let sub_mod = &mod_str[Self::ZH_UNIQUE_ENEMY_IN_YOUR_PRESENCE.len()..];
            let trans = self.trans_mod(sub_mod);
            match trans {
                Some(trans) => {
                    return Some(format!(
                        "{}{}",
                        Self::EN_UNIQUE_ENEMY_IN_YOUR_PRESENCE,
                        &trans
                    ))
                }
                None => return None,
            }
        }

        if mod_str.starts_with(Self::ZH_PINNACLE_ATLAS_BOSS_IN_YOUR_PRESENCE) {
            let sub_mod = &mod_str[Self::ZH_PINNACLE_ATLAS_BOSS_IN_YOUR_PRESENCE.len()..];
            let trans = self.trans_mod(sub_mod);
            match trans {
                Some(trans) => {
                    return Some(format!(
                        "{}{}",
                        Self::EN_PINNACLE_ATLAS_BOSS_IN_YOUR_PRESENCE,
                        &trans
                    ))
                }
                None => return None,
            }
        }

        return None;
    }

    pub fn trans_mod(&self, mod_str: &str) -> Option<String> {
        if self.is_anointed_mod(mod_str) {
            return self.trans_anointed_mod(mod_str);
        }
        if self.is_forbidden_flame_mod(mod_str) {
            return self.trans_forbidden_flame_mod(mod_str);
        }
        if self.is_forbidden_flesh_mod(mod_str) {
            return self.trans_forbidden_flesh_mod(mod_str);
        }
        if self.is_eldritch_implicit_mod(mod_str) {
            return self.trans_eldritch_implicit_mod(mod_str);
        }

        return self.trans_mod_inner(mod_str);
    }

    fn trans_mod_inner(&self, mod_str: &str) -> Option<String> {
        let body = get_zh_body(mod_str);
        let stats = self.stat_provider.provide_by_zh(&body);

        if let Some(stats) = stats {
            for stat in stats {
                let result = self.do_trans_mod(stat, mod_str);
                if result.is_some() {
                    return result;
                }
            }
        }

        return None;
    }

    fn do_trans_mod(&self, stat: &Stat, mod_str: &str) -> Option<String> {
        if mod_str == stat.zh {
            return Some(stat.en.clone());
        }

        let zh_tmpl = Template::new(&stat.zh);
        let params = zh_tmpl.parse_params(mod_str);
        if let Some(params) = params {
            let en_tmpl = Template::new(&stat.en);
            return Some(en_tmpl.render(params));
        }

        return None;
    }

    pub fn get_max_lines_of_multiline_mod(&self, first_line: &str) -> usize {
        let body = get_zh_body(first_line);
        let entry = self.stat_provider.provide_by_first_line_zh_body(&body);
        if let Some(entry) = entry {
            return entry.max_lines;
        };

        return 0;
    }
    /// Translate multiline mod for text item.
    ///
    /// Caller should use `get_max_lines_of_multiline_mod()` before to get the max lines of candidates.
    /// The method uses the `lines` to infer a multiline mod, returns the translation.
    pub fn trans_multiline_mod(&self, lines: Vec<&str>) -> Option<String> {
        let first_line_body = get_zh_body(lines[0]);
        let entry = self
            .stat_provider
            .provide_by_first_line_zh_body(&first_line_body);
        if let Some(entry) = entry {
            for m_stat in &entry.stats {
                if m_stat.line_count > lines.len() {
                    continue;
                }
                let mod_str = lines[..m_stat.line_count].join(LINE_SEPERATOR);

                let stat = self.stat_provider.provide(m_stat.id);
                if get_zh_body(&mod_str) == get_zh_body(&stat.zh) {
                    let result = self.do_trans_mod(stat, &mod_str);
                    if result.is_some() {
                        return result;
                    }
                }
            }
        };

        return None;
    }
}

#[test]
fn test_translate() {
    use crate::db::{assets, Assets};
    use std::fs;

    let contents = fs::read_to_string(assets::ASSETS_DATA).unwrap();
    let assets: Assets = serde_json::from_str(&contents).unwrap();

    let attribute_provider = attribute::Provider::new(assets.attributes);
    let basetype_provider = base_type::Provider::new(vec![
        assets.amulets,
        assets.belts,
        assets.rings,
        assets.body_armours,
        assets.boots,
        assets.gloves,
        assets.helmets,
        assets.quivers,
        assets.shields,
        assets.flasks,
        assets.jewels,
        assets.weapons,
        assets.tattoos,
    ]);
    let gem_provider = gem::Provider::new(assets.gems, assets.hybrid_skills);
    let passive_skill_provider =
        passive_skill::Provider::new(assets.notables, assets.keystones, assets.ascendant);
    let property_provider = property::Provider::new(assets.properties);
    let requirement_provider =
        requirement::Provider::new(assets.requirements, assets.requirement_suffixes);
    let stat_provider = stat::Provider::new(assets.stats);
    let basic = Basic::new(
        attribute_provider,
        basetype_provider,
        gem_provider,
        passive_skill_provider,
        property_provider,
        requirement_provider,
        stat_provider,
    );

    assert_eq!(
        basic.trans_mod("低血时最大闪避值提高 10%").unwrap(),
        "10% increased Global Evasion Rating when on Low Life"
    );

    assert!(&basic.trans_gem("投射物归返（辅）").unwrap() == "Returning Projectiles Support");
    assert!(&basic.trans_gem("增幅(辅)").unwrap() == "Enhance Support");
}
