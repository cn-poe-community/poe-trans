use std::fmt::Display;

pub struct ItemSet {
    use_second_weapon_set: bool,
    id: i32,
    pub slots: Vec<SlotItem>,
}

impl Default for ItemSet {
    fn default() -> Self {
        Self {
            use_second_weapon_set: false,
            id: 1,
            slots: vec![],
        }
    }
}

impl Display for ItemSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<ItemSet useSecondWeaponSet="{}" id="{}">
{}
</ItemSet>"#,
            self.use_second_weapon_set,
            self.id,
            self.slots
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

pub enum SlotItem {
    Slot(Slot),
    SocketIdURL(SocketIdURL),
}

impl Display for SlotItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slot(s) => (s as &Slot).fmt(f),
            Self::SocketIdURL(url) => url.fmt(f),
        }
    }
}

pub struct Slot {
    name: String,
    item_pb_url: String,
    item_id: Option<i32>,
    node_id: Option<i32>,
    active: bool,
}

impl Slot {
    pub fn new_equipment(name: &str, item_id: i32) -> Slot {
        let mut slot = Self::default();
        slot.name = name.to_string();
        slot.item_id = Some(item_id);

        if slot.name.starts_with("Flask ") {
            slot.active = true;
        }

        slot
    }

    pub fn new_jewel(name: &str, node_id: i32) -> Slot {
        let mut slot = Self::default();
        slot.name = name.to_string();
        slot.node_id = Some(node_id);

        slot
    }
}

impl Default for Slot {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            item_pb_url: "".to_string(),
            item_id: None,
            node_id: None,
            active: false,
        }
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let active_attr = match self.active {
            true => r#"active="true""#,
            false => "",
        };
        let item_id_attr = match self.item_id {
            Some(id) => format!(r#"itemId="{}""#, id),
            None => "".to_string(),
        };
        let node_id_attr = match self.node_id {
            Some(id) => format!(r#"nodeId="{}""#, id),
            None => "".to_string(),
        };
        write!(
            f,
            r#"<Slot itemPbURL="{}" {} name="{}" {} {}/>"#,
            self.item_pb_url, active_attr, self.name, &item_id_attr, &node_id_attr
        )
    }
}

pub struct SocketIdURL {
    node_id: i32,
    name: String,
    item_pb_url: String,
}

impl Default for SocketIdURL {
    fn default() -> Self {
        Self {
            node_id: 0,
            name: "".to_string(),
            item_pb_url: "".to_string(),
        }
    }
}

impl Display for SocketIdURL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<SocketIdURL nodeId="{}" name="{}" itemPbURL="{}"/>"#,
            self.node_id, self.name, self.item_pb_url,
        )
    }
}
