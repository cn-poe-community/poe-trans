use regex::Regex;

pub mod stat;

fn non_ascii_but_percent(text: &str) -> String {
    let re = Regex::new(r"[\u{0000}-\u{0024}\u{0026}-\u{007F}]").unwrap();
    return re.replace_all(text, "").into_owned();
}

pub fn get_zh_body(text: &str) -> String {
    return non_ascii_but_percent(text);
}

pub fn is_dynamic_property(text: &str) -> bool {
    return text.contains("{0}");
}

pub const LINE_SEPERATOR: &str = "\n";

mod tests {
    #[test]
    fn test_regex() {
        use crate::translator::util::non_ascii_but_percent;

        assert_eq!(
            non_ascii_but_percent(
                "近期内，你或你的召唤生物每击败一个敌人\n则每秒回复你 {0}% 能量护盾，每秒最多 10%"
            ),
            "近期内，你或你的召唤生物每击败一个敌人则每秒回复你%能量护盾，每秒最多%"
        );
    }
}
