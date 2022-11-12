import { fetch } from "undici";
import mods from "../mods.json" assert { type: "json" };
import { writeFile } from "fs/promises";

for (const mod of mods) {
  const url = `https://api.modrinth.com/v2/project/${mod}/version`;

  const versions = (await fetch(url).then((res) => res.json())) as Version[];

  const version = versions[0].files.find((f) => f.primary);

  if (!version) break;

  const res = await fetch(version.url);

  const buf = await res.arrayBuffer();

  await writeFile(`./mods/${version.filename}`, Buffer.from(buf));

  console.log(`Downloaded ${version.filename}`);
}
