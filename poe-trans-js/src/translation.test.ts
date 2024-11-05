import { expect, test } from "bun:test";
import Assets from "cn-poe-export-db";
import { ZhToEn } from "cn-poe-translator";

import { diff } from 'deep-object-diff';

function dir(): string {
  const filePath = import.meta.path;
  if (filePath.includes(`/`)) {
    return filePath.substring(0, filePath.lastIndexOf(`/`) + 1);
  } else {
    return filePath.substring(0, filePath.lastIndexOf(`\\`) + 1);
  }
}

const factory = new ZhToEn.TranslatorFactory(Assets);
const jsonTranslator = factory.getJsonTranslator();

const root = dir() + "../../";

test("items translation", async () => {
  const items = JSON.parse(await Bun.file(root + "test/items.json").text());
  const items_rs = JSON.parse(await Bun.file(root + "test/items_rs.json").text());

  jsonTranslator.transItems(items);
  await Bun.write(root + "test/items_js.json", JSON.stringify(items));

  expect(diff(items, items_rs)).toEqual({});
});

test("passive skills translation", async () => {
  const skills = JSON.parse(await Bun.file(root + "test/passive_skills.json").text());
  const skills_rs = JSON.parse(await Bun.file(root + "test/passive_skills_rs.json").text())

  jsonTranslator.transPassiveSkills(skills);
  await Bun.write(root + "test/passive_skills_js.json", JSON.stringify(skills));

  expect(diff(skills, skills_rs)).toEqual({});
});