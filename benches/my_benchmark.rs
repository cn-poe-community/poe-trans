use criterion::{criterion_group, criterion_main, Criterion};
use poe_trans::{
    model::{items::Items, passive_skills::PassiveSkills},
    transformer::{Options, Transformer},
    translator::translator::zh_to_en::Factory,
};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let factory = Factory::new();
    let translator = factory.json_translator();

    let items_str = fs::read_to_string("test/items.json").unwrap();
    let skills_str = fs::read_to_string("test/passive_skills.json").unwrap();

    c.bench_function("transform and translate", |b| {
        b.iter(|| {
            let mut items: Items = serde_json::from_str(&items_str).unwrap();
            let mut skills: PassiveSkills = serde_json::from_str(&skills_str).unwrap();

            translator.trans_items(&mut items);
            translator.trans_passive_skills(&mut skills);

            let mut option = Options::default();
            option.skip_weapon2 = false;
            let transformer = Transformer::new(items, skills, option);
            let building = transformer.transform();
            building.to_string();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
