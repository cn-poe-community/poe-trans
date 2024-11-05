use std::sync::Arc;

use flate2::{write::ZlibEncoder, Compression};
use poe_trans::{
    model::{items::Items, passive_skills::PassiveSkills},
    transformer::{Options, Transformer},
    translator::translator::zh_to_en::Factory,
};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use warp::Filter;

#[derive(Serialize, Deserialize, Debug)]
struct JsonBuild {
    items: Items,
    #[serde(rename = "passiveSkills")]
    passive_skills: PassiveSkills,
}

#[tokio::main]
async fn main() {
    let factory = Arc::new(Factory::new());
    let factory = warp::any().map(move || factory.clone());

    let filter = warp::post()
        .and(warp::path("pob"))
        .and(warp::path("create"))
        .and(warp::body::content_length_limit(1024 * 300))
        .and(factory)
        .and(warp::body::json())
        .map(|zh2en: Arc<Factory>, mut json: JsonBuild| {
            let translator = zh2en.json_translator();
            translator.trans_items(&mut json.items);
            translator.trans_passive_skills(&mut json.passive_skills);

            let transformer = Transformer::new(json.items, json.passive_skills, Options::default());

            let build_xml = transformer.transform().to_string();
            let buffer = Vec::new();
            let mut d = ZlibEncoder::new(buffer, Compression::default());
            d.write_all(build_xml.as_bytes()).unwrap();

            use base64::{engine::general_purpose::URL_SAFE, Engine as _};
            let code = URL_SAFE.encode(d.finish().unwrap());

            warp::reply::html(code)
        });

    warp::serve(filter).run(([127, 0, 0, 1], 8001)).await;
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::{write::ZlibEncoder, Compression};

    #[test]
    fn test_encode() {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(b"0123456789").unwrap();

        use base64::{engine::general_purpose::URL_SAFE, Engine as _};
        let code = URL_SAFE.encode(e.finish().unwrap());

        assert_eq!(code, "eJwzMDQyNjE1M7ewBAAK_wIO")
    }
}
