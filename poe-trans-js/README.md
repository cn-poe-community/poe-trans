# poe-trans-js

Some useful scripts or tests written in typescript：

`src/translation.test.ts` tests whether the translation result of poe-trans is consistent with cn-poe-translator：

```sh
bun test
```

`src/build_assets.ts` generates `/src/db/assets.rs`：

```sh
bun run .\src\build_assets.ts
```

## commands

First install [bun](https://bun.sh/).

To install dependencies:

```bash
bun install
```
