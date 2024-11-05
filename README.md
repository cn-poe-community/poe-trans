# poe-trans

Rewrite cn-poe-export and pob-building-creator by rust.

# usage

See `src/main.rs`.

## performance

Jmeter shows that the Rust version is as efficient as the JS version running on bun or node.js, but the former uses 1/5 of the memory of the latter.