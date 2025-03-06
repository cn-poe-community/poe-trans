mod basic;
mod json;

use std::sync::Arc;

pub use basic::Basic;
pub use json::Json;

use crate::{
    db::{assets::ASSETS_DATA, Assets},
    translator::provider::{attribute, base_type, gem, passive_skill, property, requirement, stat},
};

/// Factory for basic and json translator.
pub struct Factory {
    basic: Arc<Basic>,
}

impl Factory {
    /// Create a new Factory.
    pub fn new() -> Factory {
        let assets: Assets = serde_json::from_str(ASSETS_DATA).unwrap();

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
            assets.tinctures,
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

        let basic = Arc::new(basic);

        Factory { basic }
    }

    pub fn basic_translator(&self) -> Arc<Basic> {
        self.basic.clone()
    }

    pub fn json_translator(&self) -> Json {
        Json::new(self.basic.clone())
    }
}
