use regex::Regex;

pub mod build;
pub mod config;
pub mod items;
pub mod path_of_building;
pub mod skills;
pub mod slot;
pub mod tree;

pub fn extract_number(s: &str) -> Option<i32> {
    let re = Regex::new(r"\d+").unwrap(); // 匹配一个或多个数字
    let caps = re.captures(s)?;
    caps[0].parse::<i32>().ok()
}
