# poe-trans

Rewrite cn-poe-export and pob-building-creator in Rust.

# usage

See `src/main.rs`.

## performance

JMeter shows that the Rust version is as efficient as the JS version running on Bun or Node.js, but the former uses 1/5 of the memory of the latter.
