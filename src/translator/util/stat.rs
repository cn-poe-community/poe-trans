use std::collections::HashMap;

use regex::Regex;

/**
 The template that can be parsed to segments and parameter numbers.

 Simple:
 "Chain Hook has a {0}% chance to grant +1 Rage if it Hits Enemies"

- segments: ["Chain Hook has a ", "% chance to grant +1 Rage if it Hits Enemies"]
- parameter numbers: [0]
*/
pub struct Template {
    segments: Vec<String>,
    param_nums: Vec<i32>,
}

impl Template {
    pub fn new(text: &str) -> Template {
        let bytes = text.as_bytes();

        let mut segments = Vec::new();
        let mut param_nums = Vec::new();

        let mut j = 0;
        let mut k = 0;
        let mut on_param = false;

        for i in 0..text.len() {
            if bytes[i] == '{' as u8 {
                k = i;
                on_param = true;
            } else if bytes[i] == '}' as u8 {
                if on_param {
                    segments.push(String::from(&text[j..k]));
                    param_nums.push(text[k + 1..i].parse::<i32>().unwrap());
                    j = i + 1;
                    on_param = false;
                }
            } else {
                if on_param {
                    if bytes[i] < '0' as u8 || bytes[i] > '9' as u8 {
                        // out of "0"~"9"
                        on_param = false;
                    }
                }
            }
        }

        //tail empty segment is needed when text ends with params, beacuse parse_params() relies it.
        //beginning empty segment is needed too for the same reason.
        segments.push(String::from(&text[j..]));

        Template {
            segments,
            param_nums,
        }
    }

    // parse_params parses the modifier and returns positional parameters.
    pub fn parse_params(&self, modifier: &str) -> Option<HashMap<i32, String>> {
        let joined = self
            .segments
            .iter()
            .map(|x| regex::escape(x))
            .collect::<Vec<String>>()
            .join(r"(\S+)");
        let re = Regex::new(&format!("^{}$", joined)).unwrap();
        let matches = re.captures(modifier);

        if let Some(caps) = matches {
            let mut params_map: HashMap<i32, String> = HashMap::new();
            for i in 1..caps.len() {
                let actual_param = String::from(&caps[i]);
                params_map.insert(self.param_nums[i - 1], actual_param);
            }

            return Some(params_map);
        };
        None
    }

    pub fn render(&self, params_map: HashMap<i32, String>) -> String {
        let mut buf: Vec<&str> = vec![];
        for i in 0..self.param_nums.len() {
            buf.push(&self.segments[i]);
            let param = params_map.get(&self.param_nums[i]);
            if let Some(param) = param {
                buf.push(param);
            }
        }

        buf.push(self.segments.last().unwrap());

        buf.join("")
    }
}

mod tests {

    #[test]
    fn test_get_zh_body() {
        use crate::translator::util::get_zh_body;

        assert_eq!(
            get_zh_body("低血时最大闪避值提高 {0}%"),
            "低血时最大闪避值提高%"
        );
    }

    #[test]
    fn test_regex() {
        use super::Template;

        let zh_mod = "5 秒内回复 1000 生命";
        let zh_tmpl = Template::new("{1} 秒内回复 {0} 生命");
        let en_tmpl = Template::new("Recovers {0} Life over {1} Seconds");

        let out = en_tmpl.render(zh_tmpl.parse_params(zh_mod).unwrap());
        assert_eq!(out, "Recovers 1000 Life over 5 Seconds");
    }
}
