use std::collections::HashMap;

use crate::{
    db::Stat,
    translator::util::{get_zh_body, LINE_SEPERATOR},
};

pub struct Provider {
    stats: Vec<Stat>,
    zh_body_idx: HashMap<String, Vec<usize>>,
    first_line_zh_body_idx: HashMap<String, MultilineStats>,
}

pub struct MultilineStat {
    pub id: usize,
    pub line_count: usize,
}

pub struct MultilineStats {
    pub stats: Vec<MultilineStat>,
    pub max_lines: usize,
}

impl Provider {
    pub fn new(stats: Vec<Stat>) -> Provider {
        let mut zh_body_idx: HashMap<String, Vec<usize>> = HashMap::new();
        let mut first_line_zh_body_idx: HashMap<String, MultilineStats> = HashMap::new();

        for (id, stat) in stats.iter().enumerate() {
            let zh_body = get_zh_body(&stat.zh);
            match zh_body_idx.get_mut(&zh_body) {
                Some(vec) => vec.push(id),
                None => {
                    let vec = vec![id];
                    zh_body_idx.insert(zh_body, vec);
                }
            }

            let lines: Vec<&str> = stat.zh.split(LINE_SEPERATOR).collect();
            let line_count = lines.len();
            if line_count == 1 {
                continue;
            }

            //multiline
            let first = lines[0];
            let first_body = get_zh_body(first);

            match first_line_zh_body_idx.get_mut(&first_body) {
                Some(entry) => {
                    entry.stats.push(MultilineStat { id, line_count });
                    if line_count > entry.max_lines {
                        entry.max_lines = line_count;
                    }
                }
                None => {
                    let entry = MultilineStats {
                        stats: vec![MultilineStat { id, line_count }],
                        max_lines: line_count,
                    };
                    first_line_zh_body_idx.insert(first_body, entry);
                }
            }

            // sort by desc
            // when translate multi stats, first try stats with more lines
            for val in first_line_zh_body_idx.values_mut() {
                if val.stats.len() > 1 {
                    val.stats
                        .sort_by(|a, b| a.line_count.cmp(&b.line_count).reverse());
                }
            }
        }

        Provider {
            stats,
            zh_body_idx,
            first_line_zh_body_idx,
        }
    }

    pub fn provide(&self, id: usize) -> &Stat {
        &self.stats[id]
    }

    pub fn provide_by_zh(&self, zh: &str) -> Option<Vec<&Stat>> {
        if let Some(ids) = self.zh_body_idx.get(zh) {
            let result: Vec<&Stat> = ids.iter().map(|x| &self.stats[*x]).collect();
            return Some(result);
        }
        None
    }

    pub fn provide_by_first_line_zh_body(&self, zh: &str) -> Option<&MultilineStats> {
        self.first_line_zh_body_idx.get(zh)
    }
}
