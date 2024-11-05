use std::fmt::Display;

pub struct Tree {
    pub spec: Spec,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            spec: Spec::default(),
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Tree activeSpec="1">
{}
</Tree>"#,
            self.spec,
        )
    }
}

pub struct Spec {
    pub tree_version: String,
    pub ascend_class_id: i32,
    pub secondary_ascend_class_id: i32,
    pub class_id: i32,
    pub mastery_effects: Vec<MasteryEffect>,
    pub nodes: Vec<i32>,

    pub sockets: Sockets,
    pub overrides: Overrides,
}

impl Default for Spec {
    fn default() -> Self {
        Self {
            tree_version: "3_25".to_string(),
            ascend_class_id: 0,
            secondary_ascend_class_id: 0,
            class_id: 0,
            mastery_effects: vec![],
            nodes: vec![],
            sockets: Sockets::default(),
            overrides: Overrides::default(),
        }
    }
}

impl Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mastery_effects_view = self
            .mastery_effects
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let nodes_view = self
            .nodes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        write!(
            f,
            r#"<Spec treeVersion="{}" ascendClassId="{}" secondaryAscendClassId="{}" classId="{}" masteryEffects="{}" nodes="{}">
{}
{}
</Spec>"#,
            self.tree_version,
            self.ascend_class_id,
            self.secondary_ascend_class_id,
            self.class_id,
            mastery_effects_view,
            nodes_view,
            self.sockets,
            self.overrides
        )
    }
}

pub struct MasteryEffect {
    pub node_id: i32,
    pub effect_id: i32,
}

impl MasteryEffect {
    pub fn new(node_id: i32, effect_id: i32) -> MasteryEffect {
        return MasteryEffect { node_id, effect_id };
    }
}

impl Display for MasteryEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{{},{}}}"#, self.node_id, self.effect_id)
    }
}

pub struct Sockets {
    pub sockets: Vec<Socket>,
}

impl Default for Sockets {
    fn default() -> Self {
        Self { sockets: vec![] }
    }
}

impl Sockets {
    pub fn push(&mut self, socket: Socket) {
        self.sockets.push(socket);
    }
}

impl Display for Sockets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Sockets>
{}
</Sockets>"#,
            self.sockets
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

pub struct Socket {
    node_id: i32,
    item_id: i32,
}

impl Socket {
    pub fn new(node_id: i32, item_id: i32) -> Socket {
        Socket { node_id, item_id }
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Socket nodeId="{}" itemId="{}"/>"#,
            self.node_id, self.item_id,
        )
    }
}

pub struct Overrides {
    members: Vec<Override>,
}

impl Default for Overrides {
    fn default() -> Self {
        Self { members: vec![] }
    }
}

impl Overrides {
    pub fn push(&mut self, o: Override) {
        self.members.push(o);
    }
}

impl Display for Overrides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Overrides>
{}
</Overrides>"#,
            self.members
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

pub struct Override {
    pub dn: String,
    pub node_id: i32,
}

impl Override {
    pub fn new(dn: &str, node_id: i32) -> Override {
        return Override {
            dn: dn.to_string(),
            node_id,
        };
    }
}

impl Display for Override {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Override dn="{}" nodeId="{}">
</Override>"#,
            self.dn, self.node_id,
        )
    }
}
