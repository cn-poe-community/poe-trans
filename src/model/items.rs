use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    pub character: Character,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub class: String,
    league: String,
    pub level: i32,
    name: String,
    realm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sockets: Option<Vec<Socket>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub influences: Option<Influences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tangled: Option<bool>,
    #[serde(rename = "abyssJewel", skip_serializing_if = "Option::is_none")]
    pub abyss_jewel: Option<bool>, //exist in abyss jewels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fractured: Option<bool>,
    pub name: String,
    #[serde(rename = "typeLine")]
    pub type_line: String,
    #[serde(rename = "baseType")]
    pub base_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<String>, //exist in equipments and abyss jewels
    pub ilvl: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corrupted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<ItemProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<ItemRequirement>>,
    #[serde(rename = "enchantMods", skip_serializing_if = "Option::is_none")]
    pub enchant_mods: Option<Vec<String>>,
    #[serde(rename = "implicitMods", skip_serializing_if = "Option::is_none")]
    pub implicit_mods: Option<Vec<String>>,
    #[serde(rename = "explicitMods", skip_serializing_if = "Option::is_none")]
    pub explicit_mods: Option<Vec<String>>,
    #[serde(rename = "craftedMods", skip_serializing_if = "Option::is_none")]
    pub crafted_mods: Option<Vec<String>>,
    #[serde(rename = "utilityMods", skip_serializing_if = "Option::is_none")]
    pub utility_mods: Option<Vec<String>>,
    #[serde(rename = "fracturedMods", skip_serializing_if = "Option::is_none")]
    pub fractured_mods: Option<Vec<String>>,
    #[serde(rename = "scourgeMods", skip_serializing_if = "Option::is_none")]
    pub scourge_mods: Option<Vec<String>>,
    #[serde(rename = "crucibleMods", skip_serializing_if = "Option::is_none")]
    pub crucible_mods: Option<Vec<String>>,
    #[serde(rename = "inventoryId", skip_serializing_if = "Option::is_none")]
    pub inventory_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duplicated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synthesised: Option<bool>,
    #[serde(rename = "socketedItems", skip_serializing_if = "Option::is_none")]
    pub socketed_items: Option<Vec<Box<Item>>>, // exist in equipments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<GemHybrid>, //exist in gems
    #[serde(rename = "frameType")]
    pub frame_type: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>, //exist in equipments

    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemProperty {
    pub name: String,
    pub values: Vec<ItemPropertyValueType>,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemPropertyValueType(pub String, pub i32);

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemRequirement {
    pub name: String,
    pub values: Vec<ItemRequirementValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemRequirementValueType(pub String, pub i32);

#[derive(Serialize, Deserialize, Debug)]
pub struct GemHybrid {
    #[serde(rename = "baseTypeName")]
    pub base_type_name: String,
    #[serde(rename = "isVaalGem", skip_serializing_if = "Option::is_none")]
    pub is_vaal_gem: Option<bool>,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Influences {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shaper: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warlord: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hunter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crusader: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeemer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Socket {
    pub group: usize,
    attr: String,
    #[serde(rename = "sColour")]
    pub s_colour: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_serialize() {
        let contents = fs::read_to_string("test/items.json").unwrap();
        let items: Items = serde_json::from_str(&contents).unwrap();
        assert!(items.items.len() > 0);
    }
}
