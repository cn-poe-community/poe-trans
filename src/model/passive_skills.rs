use std::fmt;

use indexmap::IndexMap;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::Value;

use super::items;

#[derive(Serialize, Deserialize, Debug)]
pub struct PassiveSkills {
    pub character: i32,
    pub ascendancy: i32,
    pub alternate_ascendancy: i32,
    pub hashes: Vec<i32>,
    pub hashes_ex: Vec<i32>,
    pub mastery_effects: MasteryEffects,
    pub skill_overrides: IndexMap<i32, SkillOverride>,
    pub items: Vec<items::Item>,
    pub jewel_data: IndexMap<i32, JewelData>,
}

#[derive(Debug)]
pub enum MasteryEffects {
    Table(IndexMap<i32, i32>),
    Arr(Vec<i32>),
}

impl MasteryEffects {
    pub fn len(&self) -> usize {
        match self {
            Self::Table(effects) => effects.len(),
            Self::Arr(_) => 0,
        }
    }
}

impl Serialize for MasteryEffects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MasteryEffects::Table(table) => {
                let mut map = serializer.serialize_map(Some(self.len()))?;
                for (k, v) in table {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            MasteryEffects::Arr(vec) => {
                let mut seq = serializer.serialize_seq(Some(self.len()))?;
                for e in vec {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for MasteryEffects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = MasteryEffects;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an object or an array")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = IndexMap::with_capacity(access.size_hint().unwrap_or(0));

                // While there are entries remaining in the input, add them
                // into our map.
                while let Some((key, value)) = access.next_entry()? {
                    map.insert(key, value);
                }

                Ok(MasteryEffects::Table(map))
            }

            fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }

                Ok(MasteryEffects::Arr(vec))
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JewelData {
    #[serde(rename = "type")]
    pub type_v: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgraph: Option<SubGraph>,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubGraph {
    pub groups: IndexMap<String, Expansion>,
    pub nodes: IndexMap<i32, Node>,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Expansion {
    pub nodes: Vec<String>,
    pub proxy: String,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(rename = "isNotable", skip_serializing_if = "Option::is_none")]
    pub is_notable: Option<bool>,
    #[serde(rename = "isJewelSocket", skip_serializing_if = "Option::is_none")]
    pub is_jewel_socket: Option<bool>,
    #[serde(rename = "isMastery", skip_serializing_if = "Option::is_none")]
    pub is_mastery: Option<bool>,
    #[serde(rename = "expansionJewel", skip_serializing_if = "Option::is_none")]
    pub expansion_jewel: Option<ExpansionJewel>,
    pub skill: String,
    #[serde(rename = "orbitIndex")]
    pub orbit_index: i32,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpansionJewel {
    pub size: i32,
    pub index: i32,
    pub proxy: String,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillOverride {
    pub name: String,
    #[serde(rename = "isKeystone", skip_serializing_if = "Option::is_none")]
    pub is_keystone: Option<bool>,
    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_deserialize() {
        let contents = fs::read_to_string("test/passive_skills.json").unwrap();

        let de = &mut serde_json::Deserializer::from_str(&contents);
        let result: Result<PassiveSkills, _> = serde_path_to_error::deserialize(de);
        result.unwrap();
    }
}
