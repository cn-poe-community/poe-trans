use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use indexmap::IndexMap;
use support::{get_class_id, CLUSTER_JEWEL_SIZE_LARGE, CLUSTER_JEWEL_SIZE_MEDIUM};
use xml::{
    items::Item,
    path_of_building::PathOfBuilding,
    skills::{Gem, Skill},
    slot::{Slot, SlotItem},
    tree::{MasteryEffect, Override, Socket},
};

use crate::model;

mod support;
mod xml;

pub struct Options {
    skip_weapon2: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { skip_weapon2: true }
    }
}

pub struct Transformer {
    supporter: support::Supporter,
    items: model::items::Items,
    skills: model::passive_skills::PassiveSkills,
    options: Options,
}

impl Transformer {
    pub fn new(
        items: model::items::Items,
        skills: model::passive_skills::PassiveSkills,
        options: Options,
    ) -> Transformer {
        return Transformer {
            supporter: support::Supporter::new(),
            items,
            skills,
            options,
        };
    }

    pub fn transform(&self) -> PathOfBuilding {
        let mut building = PathOfBuilding::default();
        let mut item_id_gen = 1;

        let build = &mut building.build;
        let character = &self.items.character;
        build.level = character.level;
        let (class_id, ascendancy_id) = support::get_class_id(&character.class);
        build.class_name = support::get_class(class_id).to_string();
        build.ascend_class_name = support::get_ascendancy(class_id, ascendancy_id).to_string();

        self.parse_items(&mut item_id_gen, &mut building);
        self.parse_tree(&mut item_id_gen, &mut building);

        return building;
    }

    fn parse_items(&self, item_id_gen: &mut i32, building: &mut PathOfBuilding) {
        let items = self.get_building_items();
        for data in items {
            let item_id = item_id_gen.clone();
            *item_id_gen += 1;

            let item = Item::new(item_id as usize, data);
            building.items.items.push(item);

            let slot_name =
                support::get_slot_name(data.inventory_id.as_ref().unwrap(), data.x.unwrap());
            building
                .items
                .item_set
                .slots
                .push(SlotItem::Slot(Slot::new_equipment(
                    &slot_name,
                    item_id as i32,
                )));

            if let Some(socketed_items) = &data.socketed_items {
                if socketed_items.len() > 0 {
                    let sockets = data.sockets.as_ref().unwrap();
                    let mut group: Vec<&Box<model::items::Item>> = vec![];
                    let mut prev_group_num: usize = 0;
                    let skills = &mut building.skills.skill_set.skills;
                    let mut abyss_jewel_count = 0;

                    for i in 0..socketed_items.len() {
                        let si = &socketed_items[i];
                        if let Some(true) = si.abyss_jewel {
                            abyss_jewel_count += 1;

                            let item_id = item_id_gen.clone();
                            *item_id_gen += 1;

                            let item = Item::new(item_id as usize, si);
                            building.items.items.push(item);

                            let si_slot_name =
                                format!("{} Abyssal Socket {}", slot_name, abyss_jewel_count);
                            building.items.item_set.slots.push(SlotItem::Slot(
                                Slot::new_equipment(&si_slot_name, item_id as i32),
                            ));
                        } else {
                            let group_num = sockets[i].group;
                            if group_num == prev_group_num {
                                group.push(si);
                            } else {
                                let mut skill = Skill::new(&slot_name);
                                for si in &group {
                                    skill.gems.push(Gem::new(*si));
                                }
                                skills.push(skill);
                                prev_group_num = group_num;
                                group = vec![si];
                            }
                        }
                    }

                    if group.len() > 0 {
                        let mut skill = Skill::new(&slot_name);
                        for si in &group {
                            skill.gems.push(Gem::new(*si));
                        }
                        skills.push(skill);
                    }
                }
            }
        }
    }

    fn get_building_items(&self) -> Vec<&model::items::Item> {
        return self
            .items
            .items
            .iter()
            .filter(|x| {
                if let Some(x) = &x.inventory_id {
                    match x.as_str() {
                        "Weapon2" | "Offhand2" => {
                            if self.options.skip_weapon2 {
                                return false;
                            }
                        }
                        "MainInventory" | "ExpandedMainInventory" => return false,
                        _ => {}
                    }
                }
                x.base_type != "THIEFS_TRINKET"
            })
            .collect();
    }

    fn parse_tree(&self, item_id_gen: &mut i32, building: &mut PathOfBuilding) {
        let character = &self.items.character;

        let spec = &mut building.tree.spec;

        for data in &self.skills.items {
            let item_id = item_id_gen.clone();
            *item_id_gen += 1;

            let item = Item::new(item_id as usize, data);
            building.items.items.push(item);

            let socket = Socket::new(
                support::node_id_of_expansion_slot(data.x.unwrap() as usize),
                item_id,
            );
            spec.sockets.sockets.push(socket);
        }

        let (class_id, ascendancy_id) = get_class_id(&character.class);
        spec.class_id = class_id as i32;
        spec.ascend_class_id = ascendancy_id as i32;

        spec.secondary_ascend_class_id = self.skills.alternate_ascendancy;
        if let model::passive_skills::MasteryEffects::Table(effects) = &self.skills.mastery_effects
        {
            for (k, v) in effects {
                spec.mastery_effects.push(MasteryEffect {
                    node_id: *k,
                    effect_id: *v,
                });
            }
        }

        spec.nodes = self.skills.hashes.clone();
        spec.nodes
            .append(&mut self.get_enabled_node_ids_of_jewels());

        for over in &self.skills.skill_overrides {
            spec.overrides.push(Override::new(&over.1.name, *over.0));
        }
    }

    pub fn get_encoded_tree(&self) -> String {
        let (class_id, ascendancy_id) = support::get_class_id(&self.items.character.class);
        let mut buffer: Vec<u8> = Vec::with_capacity(
            6 + 1 + self.skills.hashes.len() * 2 + 2 + self.skills.mastery_effects.len() * 4,
        );
        //write head
        buffer.push(0);
        buffer.push(0);
        buffer.push(0);
        buffer.push(6);
        buffer.push(class_id as u8);
        buffer.push(ascendancy_id as u8);

        buffer.push(self.skills.hashes.len() as u8);
        for hash in &self.skills.hashes {
            buffer.extend_from_slice(&(*hash as u16).to_be_bytes()[..]);
        }

        buffer.extend_from_slice(&(self.skills.mastery_effects.len() as u16).to_be_bytes()[..]);
        if let model::passive_skills::MasteryEffects::Table(effects) = &self.skills.mastery_effects
        {
            for (k, v) in effects {
                buffer.extend_from_slice(&(*k as u16).to_be_bytes()[..]);
                buffer.extend_from_slice(&(*v as u16).to_be_bytes()[..]);
            }
        }

        use base64::{engine::general_purpose::URL_SAFE, Engine as _};
        return URL_SAFE.encode(buffer);
    }

    fn get_enabled_node_ids_of_jewels(&self) -> Vec<i32> {
        let hash_ex = &self.skills.hashes_ex;
        let jewel_data = &self.skills.jewel_data;
        let items = &self.skills.items;

        let jewel_list = get_sorted_cluster_jewels(jewel_data, items);
        let mut hash_ex_set: HashSet<i32> = hash_ex.iter().map(|x| *x).collect();

        let mut socket_expansion_jewels =
            HashMap::<i32, (i32, &model::passive_skills::ExpansionJewel)>::new();

        let mut all_enabled_node_ids: Vec<i32> = vec![];
        let mut all_probable_node_ids: Vec<i32> = vec![];

        for jewel in jewel_list {
            let seq_num = jewel.seq_num;
            let data = jewel.data;
            let size = jewel.size;

            let mut id: Option<i32> = None;
            let mut expansion_jewel: Option<&model::passive_skills::ExpansionJewel> = None;

            if size == support::CLUSTER_JEWEL_SIZE_MEDIUM
                || size == support::CLUSTER_JEWEL_SIZE_SMALL
            {
                let expansion = data
                    .subgraph
                    .as_ref()
                    .unwrap()
                    .groups
                    .get(&format!("expansion_{}", seq_num))
                    .unwrap();
                let proxy = &expansion.proxy;
                let proxy = proxy.parse::<i32>().unwrap();

                if let Some((i, e)) = socket_expansion_jewels.get(&proxy) {
                    id = Some(*i);
                    expansion_jewel = Some(e);
                }
            }

            if id.is_none() {
                let slot_node_id = support::node_id_of_expansion_slot(seq_num as usize);
                expansion_jewel = self
                    .supporter
                    .tree_meta()
                    .nodes
                    .get(&slot_node_id)
                    .unwrap()
                    .expansion_jewel
                    .as_ref();
            }

            let (mut enabled_node_ids, mut probable_node_ids) = self.get_enabled_node_ids_of_jewel(
                &mut hash_ex_set,
                &jewel,
                expansion_jewel.unwrap(),
                id,
                &mut socket_expansion_jewels,
            );

            all_enabled_node_ids.append(&mut enabled_node_ids);
            all_probable_node_ids.append(&mut probable_node_ids);
        }

        let n = min(hash_ex_set.len(), all_probable_node_ids.len());
        all_enabled_node_ids.extend(&all_probable_node_ids[..n]);

        return all_enabled_node_ids;
    }

    fn get_enabled_node_ids_of_jewel<'a>(
        &self,
        hash_ex_set: &mut HashSet<i32>,
        jewel: &Jewel<'a>,
        expansion_jewel: &model::passive_skills::ExpansionJewel,
        id: Option<i32>,
        socket_ejs: &mut HashMap<i32, (i32, &'a model::passive_skills::ExpansionJewel)>,
    ) -> (Vec<i32>, Vec<i32>) {
        let mut enabled_node_ids = Vec::<i32>::new();
        let mut probable_node_ids = Vec::<i32>::new();

        let j_size = jewel.size;
        let j_meta = self.supporter.get_jewel_meta(j_size);

        let mut id = match id {
            Some(i) => i,
            None => 0x10000,
        };

        if expansion_jewel.size == 2 {
            id = id + (expansion_jewel.index << 6);
        } else if expansion_jewel.size == 1 {
            id = id + (expansion_jewel.index << 9);
        }

        let node_id_gen = id + (j_meta.size_index << 4);

        let mut notable_ids: Vec<i32> = vec![];
        let mut socket_ids: Vec<i32> = vec![];
        let mut small_ids: Vec<i32> = vec![];

        let group = jewel
            .data
            .subgraph
            .as_ref()
            .unwrap()
            .groups
            .get(&format!("expansion_{}", jewel.seq_num))
            .unwrap();
        let original_node_ids = &group.nodes;
        let jewel_nodes = &jewel.data.subgraph.as_ref().unwrap().nodes;

        if original_node_ids.len() == 0
            && jewel_nodes.len() == 0
            && jewel.item.rarity.as_ref().unwrap() == "Unique"
        {
            probable_node_ids.push(node_id_gen);
            return (enabled_node_ids, probable_node_ids);
        }

        for id_str in original_node_ids {
            let original_id: i32 = id_str.parse().unwrap();
            let node = jewel_nodes.get(&original_id).unwrap();

            if let Some(true) = node.is_notable {
                notable_ids.push(original_id);
            } else if let Some(true) = node.is_jewel_socket {
                socket_ids.push(original_id);

                let proxy: i32 = node
                    .expansion_jewel
                    .as_ref()
                    .unwrap()
                    .proxy
                    .parse()
                    .unwrap();
                socket_ejs.insert(proxy, (id, node.expansion_jewel.as_ref().unwrap()));
            } else if let Some(true) = node.is_mastery {
                //skip
            } else {
                small_ids.push(original_id);
            }
        }

        let node_count = notable_ids.len() + socket_ids.len() + small_ids.len();
        let mut pob_jewel_nodes: Vec<Rc<ClusterJewelNode>> = vec![];
        let mut indicies = HashMap::<i32, Rc<ClusterJewelNode>>::new();

        if j_size == CLUSTER_JEWEL_SIZE_LARGE && socket_ids.len() == 1 {
            let socket = jewel_nodes.get(&socket_ids[0]).unwrap();
            let pob_node = ClusterJewelNode {
                id: socket.skill.parse().unwrap(),
                o_idx: 6,
            };
            let rc = Rc::new(pob_node);
            pob_jewel_nodes.push(rc.clone());
            indicies.insert(rc.o_idx, rc.clone());
        } else {
            for i in 0..socket_ids.len() {
                let socket = jewel_nodes.get(&socket_ids[i]).unwrap();
                let pob_node = ClusterJewelNode {
                    id: socket.skill.parse().unwrap(),
                    o_idx: j_meta.socket_indicies[i],
                };
                let rc = Rc::new(pob_node);
                pob_jewel_nodes.push(rc.clone());
                indicies.insert(rc.o_idx, rc.clone());
            }
        }

        let mut notable_indicies: Vec<i32> = vec![];
        for n in &j_meta.notable_indicies {
            let mut n = *n;
            if notable_indicies.len() == notable_ids.len() {
                break;
            }
            if j_size == CLUSTER_JEWEL_SIZE_MEDIUM {
                if socket_ids.len() == 0 && notable_ids.len() == 2 {
                    n = match n {
                        6 => 4,
                        10 => 8,
                        _ => n,
                    };
                } else if node_count == 4 {
                    n = match n {
                        10 => 9,
                        2 => 3,
                        _ => n,
                    }
                }
            }
            if !indicies.contains_key(&n) {
                notable_indicies.push(n);
            }
        }
        notable_indicies.sort();

        for i in 0..notable_indicies.len() {
            let idx = notable_indicies[i];
            let pob_node = ClusterJewelNode {
                id: node_id_gen + idx,
                o_idx: idx,
            };
            let rc = Rc::new(pob_node);
            pob_jewel_nodes.push(rc.clone());
            indicies.insert(rc.o_idx, rc.clone());
        }

        let mut small_indicies: Vec<i32> = vec![];
        for n in &j_meta.small_indicies {
            let mut n = *n;
            if small_indicies.len() == small_ids.len() {
                break;
            }
            if j_size == CLUSTER_JEWEL_SIZE_MEDIUM {
                if node_count == 5 && n == 4 {
                    n = 3;
                } else if node_count == 4 {
                    n = match n {
                        8 => 9,
                        4 => 3,
                        _ => n,
                    }
                }
            }
            if !indicies.contains_key(&n) {
                small_indicies.push(n);
            }
        }

        for i in 0..small_indicies.len() {
            let idx = small_indicies[i];
            let pob_node = ClusterJewelNode {
                id: node_id_gen + idx,
                o_idx: idx,
            };
            let rc = Rc::new(pob_node);
            pob_jewel_nodes.push(rc.clone());
            indicies.insert(rc.o_idx, rc.clone());
        }

        let proxy_num: i32 = expansion_jewel.proxy.parse().unwrap();
        let proxy_node = self.supporter.tree_meta().nodes.get(&proxy_num).unwrap();
        let proxy_node_skill_per_orbit =
            self.supporter.tree_meta().constants.skills_per_orbit[proxy_node.orbit as usize];

        for node in pob_jewel_nodes {
            let proxy_node_oidx_relative_to_cluster_indicies = translate_oidx(
                proxy_node.orbit_index,
                proxy_node_skill_per_orbit,
                j_meta.total_indicies,
            );
            let corrected_node_oidx_relative_to_cluster_indicies =
                (node.o_idx + proxy_node_oidx_relative_to_cluster_indicies) % j_meta.total_indicies;
            let corrected_node_oidx_relative_to_tree_skills_per_orbit = translate_oidx(
                corrected_node_oidx_relative_to_cluster_indicies,
                j_meta.total_indicies,
                proxy_node_skill_per_orbit,
            );
            // it's hard to change and no need to change currently
            // but if you read node.oidx next, you should do change
            // node.o_idx = correctedNodeOidxRelativeToTreeSkillsPerOrbit;
            indicies.insert(
                corrected_node_oidx_relative_to_tree_skills_per_orbit,
                node.clone(),
            );
        }

        for i in original_node_ids {
            let origninal_id: i32 = i.parse().unwrap();
            let node = jewel_nodes.get(&origninal_id).unwrap();
            if hash_ex_set.contains(&origninal_id) {
                let pob_node = indicies.get(&node.orbit_index);
                if let Some(pob_node) = pob_node {
                    enabled_node_ids.push(pob_node.id);
                }
            }
            hash_ex_set.remove(&origninal_id);
        }

        return (enabled_node_ids, probable_node_ids);
    }
}

struct Jewel<'a> {
    seq_num: i32,
    item: &'a model::items::Item,
    data: &'a model::passive_skills::JewelData,
    size: usize,
}

/// Return jewels sorted by size desc.
fn get_sorted_cluster_jewels<'a>(
    jewel_data: &'a IndexMap<i32, model::passive_skills::JewelData>,
    items: &'a Vec<model::items::Item>,
) -> Vec<Jewel<'a>> {
    let mut item_idx = HashMap::<i32, &model::items::Item>::new();
    for item in items {
        item_idx.insert(item.x.unwrap(), &item);
    }

    let mut jewel_list: Vec<Jewel> = vec![];
    for (k, v) in jewel_data {
        let seq_num = *k;

        let size = support::cluster_jewel_size(&v.type_v);

        if size < 0 {
            //not cluster
            continue;
        }

        jewel_list.push(Jewel {
            seq_num,
            item: item_idx.get(&seq_num).unwrap(),
            data: v,
            size: size as usize,
        });
    }

    jewel_list.sort_by(|a, b| a.size.cmp(&b.size).reverse());

    return jewel_list;
}

struct ClusterJewelNode {
    id: i32,
    o_idx: i32,
}

fn translate_oidx(src_oidx: i32, src_nodes_per_orbit: i32, dest_nodes_per_orbit: i32) -> i32 {
    if src_nodes_per_orbit == dest_nodes_per_orbit {
        return src_oidx;
    } else if src_nodes_per_orbit == 12 && dest_nodes_per_orbit == 16 {
        return [0, 1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 15][src_oidx as usize];
    } else if src_nodes_per_orbit == 16 && dest_nodes_per_orbit == 12 {
        return [0, 1, 1, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9, 10, 10, 11][src_oidx as usize];
    } else {
        return src_oidx * dest_nodes_per_orbit / src_nodes_per_orbit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_transform() {
        let items_json = fs::read_to_string("test/items_rs.json").unwrap();
        let items: model::items::Items = serde_json::from_str(&items_json).unwrap();

        let skills_json = fs::read_to_string("test/passive_skills_rs.json").unwrap();
        let skills: model::passive_skills::PassiveSkills =
            serde_json::from_str(&skills_json).unwrap();

        let mut option = Options::default();
        option.skip_weapon2 = false;
        let transformer = Transformer::new(items, skills, option);
        let building = transformer.transform();

        let _ = fs::write("test/building.xml", building.to_string());
    }
}
