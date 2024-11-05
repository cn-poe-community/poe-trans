use std::fmt::Display;

const K_ENEMY_IS_BOSS: &str = "enemyIsBoss";
const V_ENEMY_SHAPER: &str = "Pinnacle";

pub struct Config {
    inputs: Vec<Input>,
}

impl Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            inputs: vec![Input::new_string(K_ENEMY_IS_BOSS, V_ENEMY_SHAPER)],
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Config>
{}
</Config>"#,
            self.inputs
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
        )
    }
}

struct Input {
    name: String,
    type_str: String,
    value: String,
}

impl Input {
    fn new_string(k: &str, v: &str) -> Input {
        Input {
            name: k.to_string(),
            type_str: "string".to_string(),
            value: v.to_string(),
        }
    }

    /*
    fn new_number(k:&str,v:&str)->Input{
        Input { name: k.to_string(), type_str: "number".to_string(), value: v.to_string() }
    }

    fn new_boolean(k:&str,v:&str)->Input{
        Input { name: k.to_string(), type_str: "boolean".to_string(), value: v.to_string() }
    }*/
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<Input name="{}" {}="{}"/>"#,
            self.name, self.type_str, self.value,
        )
    }
}
