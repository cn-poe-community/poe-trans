use std::{collections::HashMap, fmt::Display};

use phf::phf_map;

use crate::model::{
    self,
    items::{ItemProperty, ItemRequirement},
};

use super::{extract_number, slot::ItemSet};

static RARITY_TABLE: [&str; 11] = [
    "NORMAL", "MAGIC", "RARE", "UNIQUE", "", "", "", "", "", "RELIC", "RELIC",
];

static ITEM_NAME_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "Doppelgänger Guise" => "Doppelganger Guise",
    "Mjölner" => "Mjolner",
};

static POB_BASE_TYPE_ENERGY_BLADE: &str = "Energy Blade One Handed";

static BASE_TYPE_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "Maelström Staff"=> "Maelstrom Staff",
    "Energy Blade"=> POB_BASE_TYPE_ENERGY_BLADE,
};

pub struct Items {
    pub items: Vec<Item>,
    pub item_set: ItemSet,
}

impl Default for Items {
    fn default() -> Self {
        Self {
            items: vec![],
            item_set: ItemSet::default(),
        }
    }
}

impl Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Items>
{}
{}
</Items>"#,
            self.items
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
            self.item_set
        )
    }
}

pub struct Item {
    pub id: usize,
    rarity: String,
    name: String,
    type_line: String,
    base_type: String,
    evasion: Option<String>,
    energy_shield: Option<String>,
    armour: Option<String>,
    ward: Option<String>,
    unique_id: String,
    shaper: Option<bool>,
    elder: Option<bool>,
    warlord: Option<bool>,
    hunter: Option<bool>,
    crusader: Option<bool>,
    redeemer: Option<bool>,
    searing: Option<bool>,
    tangled: Option<bool>,
    ilvl: i32,
    quality: Option<i32>,
    sockets: Option<String>,
    radius: Option<String>,
    limited_to: Option<String>,
    requires_class: Option<String>,
    enchant_mods: Vec<String>,
    implicit_mods: Vec<String>,
    explicit_mods: Vec<String>,
    fractured_mods: Vec<String>,
    crafted_mods: Vec<String>,
    crucible_mods: Vec<String>,
    corrupted: bool,
}

impl Item {
    pub fn new(id: usize, item: &model::items::Item) -> Item {
        let name = match ITEM_NAME_MAP.get(&item.name) {
            Some(name) => String::from(*name),
            None => item.name.clone(),
        };
        let base_type = match BASE_TYPE_MAP.get(&item.base_type) {
            Some(base_type) => String::from(*base_type),
            None => item.base_type.clone(),
        };

        let type_line = match BASE_TYPE_MAP.get(&item.base_type) {
            Some(base_type) => item.type_line.replace(&item.base_type, *base_type),
            None => item.type_line.clone(),
        };

        let mut prop_name_idx: HashMap<&str, &ItemProperty> = HashMap::new();
        if let Some(props) = &item.properties {
            for prop in props {
                prop_name_idx.insert(&prop.name, &prop);
            }
        }

        let mut evasion: Option<String> = None;
        if let Some(prop) = prop_name_idx.get("Evasion Rating") {
            evasion = Some(prop.values[0].0.clone());
        }
        let mut energy_shield: Option<String> = None;
        if let Some(prop) = prop_name_idx.get("Energy Shield") {
            energy_shield = Some(prop.values[0].0.clone());
        }
        let mut armour: Option<String> = None;
        if let Some(prop) = prop_name_idx.get("Armour") {
            armour = Some(prop.values[0].0.clone());
        }
        let mut ward: Option<String> = None;
        if let Some(prop) = prop_name_idx.get("Ward") {
            ward = Some(prop.values[0].0.clone());
        }
        let mut quality: Option<i32> = None;
        if let Some(prop) = prop_name_idx.get("Quality") {
            quality = extract_number(&prop.values[0].0);
        }

        let mut shaper: Option<bool> = None;
        let mut elder: Option<bool> = None;
        let mut warlord: Option<bool> = None;
        let mut hunter: Option<bool> = None;
        let mut crusader: Option<bool> = None;
        let mut redeemer: Option<bool> = None;

        if let Some(influences) = &item.influences {
            shaper = influences.shaper;
            elder = influences.elder;
            warlord = influences.warlord;
            hunter = influences.hunter;
            crusader = influences.crusader;
            redeemer = influences.redeemer;
        }
        let searing = item.searing;
        let tangled = item.tangled;

        let mut abyssal_socket_count = 0;
        let mut pob_sockets: Option<String> = None;
        if let Some(sockets) = &item.sockets {
            let mut buffer: Vec<String> = vec![];
            for i in 0..sockets.len() {
                let socket = &sockets[i];
                if i > 0 {
                    if socket.group == sockets[i - 1].group {
                        buffer.push("-".to_string());
                    } else {
                        buffer.push(" ".to_string());
                    }
                }
                buffer.push(sockets[i].s_colour.clone());
                if "A" == sockets[i].s_colour {
                    abyssal_socket_count += 1;
                }
            }
            if buffer.len() > 0 {
                pob_sockets = Some(buffer.concat());
            }
        }

        let mut radius: Option<String> = None;
        if let Some(prop) = prop_name_idx.get("Radius") {
            radius = Some(prop.values[0].0.clone());
        }
        let mut limited_to: Option<String> = None;
        if let Some(prop) = prop_name_idx.get("Limited to") {
            limited_to = Some(prop.values[0].0.clone());
        }

        let mut req_name_idx: HashMap<&str, &ItemRequirement> = HashMap::new();
        if let Some(req) = &item.requirements {
            for prop in req {
                req_name_idx.insert(&prop.name, &prop);
            }
        }

        let mut requires_class: Option<String> = None;
        if let Some(req) = req_name_idx.get("Class:") {
            requires_class = Some(req.values[0].0.clone());
        }

        let mut enchant_mods: Vec<String> = vec![];
        if let Some(mods) = &item.enchant_mods {
            enchant_mods = mods
                .iter()
                .flat_map(|s| s.lines().map(|x| x.to_string()))
                .collect();
        }
        let mut implicit_mods: Vec<String> = vec![];
        if let Some(mods) = &item.implicit_mods {
            implicit_mods = mods
                .iter()
                .flat_map(|s| s.lines().map(|x| x.to_string()))
                .collect();
        }
        let mut explicit_mods: Vec<String> = vec![];
        if let Some(mods) = &item.explicit_mods {
            explicit_mods = mods
                .iter()
                .flat_map(|s| s.lines().map(|x| x.to_string()))
                .collect();
        }
        let mut fractured_mods: Vec<String> = vec![];
        if let Some(mods) = &item.fractured_mods {
            fractured_mods = mods
                .iter()
                .flat_map(|s| s.lines().map(|x| x.to_string()))
                .collect();
        }
        let mut crafted_mods: Vec<String> = vec![];
        if let Some(mods) = &item.crafted_mods {
            crafted_mods = mods
                .iter()
                .flat_map(|s| s.lines().map(|x| x.to_string()))
                .collect();
        }
        let mut crucible_mods: Vec<String> = vec![];
        if let Some(mods) = &item.crucible_mods {
            crucible_mods = mods
                .iter()
                .flat_map(|s| s.lines().map(|x| x.to_string()))
                .collect();
        }

        if item.base_type == POB_BASE_TYPE_ENERGY_BLADE {
            implicit_mods = vec![];
            if abyssal_socket_count > 0 {
                explicit_mods = vec![format!("`Has {} Abyssal Sockets`", abyssal_socket_count)];
            }
        }

        let corrupted = item.corrupted.unwrap_or(false);

        Item {
            id,
            rarity: RARITY_TABLE[item.frame_type].to_string(),
            name,
            type_line,
            base_type,
            evasion,
            energy_shield,
            armour,
            ward,
            unique_id: item.id.clone(),
            shaper,
            elder,
            warlord,
            hunter,
            crusader,
            redeemer,
            searing,
            tangled,
            ilvl: item.ilvl,
            quality,
            sockets: pob_sockets,
            radius,
            limited_to,
            requires_class,
            enchant_mods,
            implicit_mods,
            explicit_mods,
            fractured_mods,
            crafted_mods,
            crucible_mods,
            corrupted,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder: Vec<String> = vec![];

        //rarity
        builder.push(format!("Rarity: {}", &self.rarity));

        //name&base_type or typeline
        if self.name.is_empty() {
            builder.push(self.type_line.clone());
        } else {
            builder.push(self.name.clone());
            builder.push(self.base_type.clone());
        }

        if let Some(evasion) = &self.evasion {
            builder.push(format!("Evasion: {}", evasion));
        }
        if let Some(energy_shield) = &self.energy_shield {
            builder.push(format!("Energy Shield: {}", energy_shield));
        }
        if let Some(armour) = &self.armour {
            builder.push(format!("Armour: {}", armour));
        }
        if let Some(ward) = &self.ward {
            builder.push(format!("Ward: {}", ward));
        }

        builder.push(format!("Unique ID: {}", self.unique_id));

        if let Some(true) = self.shaper {
            builder.push("Shaper Item".to_string());
        }
        if let Some(true) = self.elder {
            builder.push("Elder Item".to_string());
        }
        if let Some(true) = self.warlord {
            builder.push("Warlord Item".to_string());
        }
        if let Some(true) = self.hunter {
            builder.push("Hunter Item".to_string());
        }
        if let Some(true) = self.crusader {
            builder.push("Crusader Item".to_string());
        }
        if let Some(true) = self.redeemer {
            builder.push("Redeemer Item".to_string());
        }

        if let Some(true) = self.searing {
            builder.push("Searing Exarch Item".to_string());
        }
        if let Some(true) = self.tangled {
            builder.push("Eater of Worlds Item".to_string());
        }
        builder.push(format!("Item Level: {}", self.ilvl));
        if let Some(quality) = self.quality {
            builder.push(format!("Quality: {}", quality));
        }
        if let Some(sockets) = &self.sockets {
            builder.push(format!("Sockets: {}", sockets));
        }

        if let Some(radius) = &self.radius {
            builder.push(format!("Radius: {}", radius));
        }

        if let Some(limited_to) = &self.limited_to {
            builder.push(format!("Limited to: {}", limited_to));
        }

        if let Some(requires_class) = &self.requires_class {
            builder.push(format!("Requires Class {}", requires_class));
        }

        let implicit_count = self.enchant_mods.len() + self.implicit_mods.len();
        builder.push(format!("Implicits: {}", implicit_count));

        for mod_str in &self.enchant_mods {
            builder.push(format!("{{crafted}}{}", mod_str));
        }

        for mod_str in &self.implicit_mods {
            builder.push(mod_str.to_string());
        }

        for mod_str in &self.explicit_mods {
            builder.push(mod_str.to_string());
        }

        for mod_str in &self.fractured_mods {
            builder.push(format!("{{fractured}}{}", mod_str));
        }

        for mod_str in &self.crafted_mods {
            builder.push(format!("{{crafted}}{}", mod_str));
        }

        for mod_str in &self.crucible_mods {
            builder.push(format!("{{crucible}}{}", mod_str));
        }

        if self.corrupted {
            builder.push("Corrupted".to_string());
        }

        write!(
            f,
            r#"<Item id="{}">
{}
</Item>"#,
            self.id,
            builder.join("\n"),
        )
    }
}
