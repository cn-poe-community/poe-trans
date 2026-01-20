use serde::{Deserialize, Serialize};

pub mod assets;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub zh: String,
    pub en: String,
    pub values: Option<Vec<Attribute>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeValue {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseType {
    pub zh: String,
    pub en: String,
    pub uniques: Option<Vec<Unique>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unique {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gem {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub zh: String,
    pub en: String,
    pub values: Option<Vec<PropertyValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyValue {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Requirement {
    pub zh: String,
    pub en: String,
    pub values: Option<Vec<RequirementValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequirementValue {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequirementSuffix {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    pub zh: String,
    pub en: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub amulets: Vec<BaseType>,
    pub belts: Vec<BaseType>,
    pub rings: Vec<BaseType>,
    #[serde(rename = "bodyArmours")]
    pub body_armours: Vec<BaseType>,
    pub boots: Vec<BaseType>,
    pub gloves: Vec<BaseType>,
    pub helmets: Vec<BaseType>,
    pub quivers: Vec<BaseType>,
    pub shields: Vec<BaseType>,
    pub weapons: Vec<BaseType>,
    pub flasks: Vec<BaseType>,
    pub jewels: Vec<BaseType>,
    pub tinctures: Vec<BaseType>,
    pub gems: Vec<Gem>,
    #[serde(rename = "hybridSkills")]
    pub hybrid_skills: Vec<Skill>,
    pub attributes: Vec<Attribute>,
    pub properties: Vec<Property>,
    pub requirements: Vec<Requirement>,
    #[serde(rename = "requirementSuffixes")]
    pub requirement_suffixes: Vec<RequirementSuffix>,
    pub ascendant: Vec<Node>,
    pub keystones: Vec<Node>,
    pub notables: Vec<Node>,
    pub stats: Vec<Stat>,
    pub tattoos: Vec<BaseType>,
    pub grafts: Vec<BaseType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assets_serialize() {
        let assets: Assets = serde_json::from_str(assets::ASSETS_DATA).unwrap();
        assert!(assets.amulets.len() > 0);
    }
}
