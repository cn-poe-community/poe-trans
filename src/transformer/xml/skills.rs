use std::{collections::HashMap, fmt::Display};

use phf::phf_set;

use crate::model::{self, items::ItemProperty};

use super::extract_number;

static TRANSFIGURED_GEMS: phf::Set<&'static str> = phf_set! {
    "Ice Nova of Frostbolts",
    "Ice Nova of Deep Freeze",
    "Leap Slam of Groundbreaking",
    "Blade Flurry of Incision",
    "Ground Slam of Earthshaking",
    "Cleave of Rage",
    "Double Strike of Impaling",
    "Double Strike of Momentum",
    "Lacerate of Butchering",
    "Lacerate of Haemorrhage",
    "Elemental Hit of the Spectrum",
    "Dual Strike of Ambidexterity",
    "Frenzy of Onslaught",
    "Detonate Dead of Scavenging",
    "Detonate Dead of Chain Reaction",
    "Volatile Dead of Confinement",
    "Volatile Dead of Seething",
    "Split Arrow of Splitting",
    "Cold Snap of Power",
    "Raise Zombie of Slamming",
    "Raise Zombie of Falling",
    "Caustic Arrow of Poison",
    "Righteous Fire of Arcane Devotion",
    "Discharge of Misery",
    "Flicker Strike of Power",
    "Spark of the Nova",
    "Spark of Unpredictability",
    "Ice Spear of Splitting",
    "Raise Spectre of Transience",
    "Infernal Blow of Immolation",
    "Glacial Hammer of Shattering",
    "Viper Strike of the Mamba",
    "Summon Skeletons of Archers",
    "Summon Skeletons of Mages",
    "Dominating Blow of Inspiring",
    "Rain of Arrows of Artillery",
    "Rain of Arrows of Saturation",
    "Firestorm of Meteors",
    "Firestorm of Pelting",
    "Lightning Strike of Arcing",
    "Power Siphon of the Archmage",
    "Puncture of Shanking",
    "Lightning Arrow of Electrocution",
    "Burning Arrow of Vigour",
    "Icicle Mine of Fanning",
    "Icicle Mine of Sabotage",
    "Bear Trap of Skewers",
    "Fire Trap of Blasting",
    "Ethereal Knives of Lingering Blades",
    "Ethereal Knives of the Massacre",
    "Ice Shot of Penetration",
    "Arc of Surging",
    "Arc of Oscillating",
    "Holy Flame Totem of Ire",
    "Incinerate of Expanse",
    "Incinerate of Venting",
    "Cyclone of Tumult",
    "Reave of Refraction",
    "Lightning Trap of Sparking",
    "Animate Guardian of Smiting",
    "Spectral Shield Throw of Shattering",
    "Spectral Throw of Materialising",
    "Animate Weapon of Self Reflection",
    "Animate Weapon of Ranged Arms",
    "Flameblast of Celerity",
    "Flameblast of Contraction",
    "Barrage of Volley Fire",
    "Ball Lightning of Orbiting",
    "Ball Lightning of Static",
    "Summon Raging Spirit of Enormity",
    "Flame Surge of Combusting",
    "Glacial Cascade of the Fissure",
    "Molten Strike of the Zenith",
    "Pyroclast Mine of Sabotage",
    "Tornado Shot of Cloudburst",
    "Lightning Tendrils of Eccentricity",
    "Lightning Tendrils of Escalation",
    "Kinetic Blast of Clustering",
    "Blink Arrow of Bombarding Clones",
    "Blink Arrow of Prismatic Clones",
    "Mirror Arrow of Bombarding Clones",
    "Mirror Arrow of Prismatic Clones",
    "Summon Chaos Golem of Hordes",
    "Summon Chaos Golem of the Maelstr枚m",
    "Summon Ice Golem of Hordes",
    "Summon Ice Golem of Shattering",
    "Summon Flame Golem of Hordes",
    "Summon Flame Golem of the Meteor",
    "Summon Lightning Golem of Hordes",
    "Ice Crash of Cadence",
    "Flame Dash of Return",
    "Frost Blades of Katabasis",
    "Wild Strike of Extremes",
    "Ice Trap of Hollowness",
    "Galvanic Arrow of Energy",
    "Galvanic Arrow of Surging",
    "Bladefall of Volleys",
    "Bladefall of Impaling",
    "Blade Vortex of the Scythe",
    "Frost Bomb of Instability",
    "Frost Bomb of Forthcoming",
    "Siege Ballista of Splintering",
    "Summon Stone Golem of Hordes",
    "Summon Stone Golem of Safeguarding",
    "Earthquake of Amplification",
    "Contagion of Subsiding",
    "Contagion of Transference",
    "Blight of Contagion",
    "Blight of Atrophy",
    "Essence Drain of Desperation",
    "Essence Drain of Wickedness",
    "Sunder of Earthbreaking",
    "Vortex of Projection",
    "Scorching Ray of Immolation",
    "Cremation of Exhuming",
    "Cremation of the Volcano",
    "Bodyswap of Sacrifice",
    "Tectonic Slam of Cataclysm",
    "Explosive Trap of Shrapnel",
    "Explosive Trap of Magnitude",
    "Lightning Spire Trap of Zapping",
    "Lightning Spire Trap of Overloading",
    "Seismic Trap of Swells",
    "Consecrated Path of Endurance",
    "Smite of Divine Judgement",
    "Scourge Arrow of Menace",
    "Toxic Rain of Sporeburst",
    "Toxic Rain of Withering",
    "Summon Holy Relic of Conviction",
    "Lancing Steel of Spraying",
    "Storm Brand of Indecision",
    "Shattering Steel of Ammunition",
    "Armageddon Brand of Volatility",
    "Armageddon Brand of Recall",
    "Purifying Flame of Revelations",
    "Soulrend of Reaping",
    "Soulrend of the Spiral",
    "Bane of Condemnation",
    "Divine Ire of Holy Lightning",
    "Divine Ire of Disintegration",
    "Bladestorm of Uncertainty",
    "Perforate of Duality",
    "Perforate of Bloodshed",
    "Frostblink of Wintry Blast",
    "Summon Carrion Golem of Hordes",
    "Summon Carrion Golem of Scavenging",
    "Shrapnel Ballista of Steel",
    "Artillery Ballista of Cross Strafe",
    "Artillery Ballista of Focus Fire",
    "Stormbind of Teleportation",
    "Kinetic Bolt of Fragmentation",
    "Blade Blast of Unloading",
    "Blade Blast of Dagger Detonation",
    "Penance Brand of Dissipation",
    "Penance Brand of Conduction",
    "Earthshatter of Fragility",
    "Earthshatter of Prominence",
    "Void Sphere of Rending",
    "Crackling Lance of Branching",
    "Crackling Lance of Disintegration",
    "Splitting Steel of Ammunition",
    "Hexblast of Contradiction",
    "Hexblast of Havoc",
    "Exsanguinate of Transmission",
    "Rage Vortex of Berserking",
    "Absolution of Inspiring",
    "Forbidden Rite of Soul Sacrifice",
    "Blade Trap of Greatswords",
    "Blade Trap of Laceration",
    "Storm Rain of the Conduit",
    "Storm Rain of the Fence",
    "Shield Crush of the Chieftain",
    "Summon Reaper of Revenants",
    "Summon Reaper of Eviscerating",
    "Boneshatter of Complex Trauma",
    "Boneshatter of Carnage",
    "Explosive Concoction of Destruction",
    "Eye of Winter of Finality",
    "Eye of Winter of Transience",
    "Poisonous Concoction of Bouncing",
    "Tornado of Elemental Turbulence",
    "Lightning Conduit of the Heavens",
    "Galvanic Field of Intensity",
    "Volcanic Fissure of Snaking",
    "Frozen Legion of Rallying",
};

pub struct Skills {
    pub skill_set: SkillSet,
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            skill_set: SkillSet::default(),
        }
    }
}

impl Display for Skills {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Skills activeSkillSet="1">
{}
</Skills>"#,
            self.skill_set
        )
    }
}

pub struct SkillSet {
    pub skills: Vec<Skill>,
}

impl Default for SkillSet {
    fn default() -> Self {
        Self { skills: vec![] }
    }
}

impl Display for SkillSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<SkillSet id="1">
{}
</SkillSet>"#,
            self.skills
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

pub struct Skill {
    slot: String,
    pub gems: Vec<Gem>,
}

impl Skill {
    pub fn new(slot_name: &str) -> Skill {
        let gem_list: Vec<Gem> = vec![];

        Skill {
            slot: slot_name.to_string(),
            gems: gem_list,
        }
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Skill enabled="true" slot="{}" mainActiveSkill="nil">
{}
</Skill>"#,
            self.slot,
            self.gems
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

pub struct Gem {
    level: i32,
    quality_id: String,
    quality: i32,
    name_spec: String,
    enable_global1: bool,
    enable_global2: bool,
}

impl Default for Gem {
    fn default() -> Self {
        Self {
            level: 20,
            quality_id: "Default".to_string(),
            quality: 0,
            name_spec: "".to_string(),
            enable_global1: true,
            enable_global2: false,
        }
    }
}

impl Gem {
    fn is_vaal_gem(&self) -> bool {
        return self.name_spec.starts_with("Vaal ");
    }

    pub fn new(data: &model::items::Item) -> Gem {
        let mut gem = Self::default();

        let mut prop_name_idx: HashMap<&str, &ItemProperty> = HashMap::new();
        if let Some(props) = &data.properties {
            for prop in props {
                prop_name_idx.insert(&prop.name, &prop);
            }
        }

        if let Some(prop) = prop_name_idx.get("Level") {
            gem.level = extract_number(&prop.values[0].0).unwrap();
        }
        if let Some(prop) = prop_name_idx.get("Quality") {
            gem.quality = extract_number(&prop.values[0].0).unwrap();
        }

        gem.name_spec = data.base_type.replace(" Support", "");

        if let Some(hybrid) = &data.hybrid {
            if let Some(true) = hybrid.is_vaal_gem {
                let hybrid_base_type_name = &hybrid.base_type_name;
                if TRANSFIGURED_GEMS.contains(hybrid_base_type_name.as_str()) {
                    gem.name_spec = format!("Vaal {}", hybrid_base_type_name);
                }
            }
        }

        if gem.is_vaal_gem() {
            gem.enable_global1 = false;
            gem.enable_global2 = true;
        }

        return gem;
    }
}

impl Display for Gem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Gem level="{}" qualityId="{}" quality="{}" nameSpec="{}" enabled="true" enableGlobal1="{}" enableGlobal2="{}"/>"#,
            self.level,
            self.quality_id,
            self.quality,
            self.name_spec,
            self.enable_global1,
            self.enable_global2
        )
    }
}

mod tests {
    #[test]
    fn test_transfigured_gems() {
        use super::TRANSFIGURED_GEMS;

        let name = format!("{} of {}", "Barrage", "Volley Fire");
        assert!(TRANSFIGURED_GEMS.contains(&name));
    }
}
