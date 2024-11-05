import Asssets from "cn-poe-export-db";

function dir(): string {
  const filePath = import.meta.path;
  if (filePath.includes(`/`)) {
    return filePath.substring(0, filePath.lastIndexOf(`/`) + 1);
  } else {
    return filePath.substring(0, filePath.lastIndexOf(`\\`) + 1);
  }
}

const root = dir() + "../../";

const data = JSON.stringify(Asssets, null, 2);

const path = root + "src/db/assets.rs";
const code = `pub static ASSETS_DATA: &str = r#"${data}"#;`;

//Bun is provided by bun.exe
await Bun.write(path, code);