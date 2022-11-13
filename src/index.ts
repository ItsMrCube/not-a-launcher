import { fetch } from "undici";
import mods from "../mods.json" assert { type: "json" };
import fs from "fs/promises";
import path from "path";

for (const file of await fs.readdir(mods.folder)) {
  await fs.unlink(path.join(mods.folder, file));
}

for (const mod of mods.mods) {
  const url = `https://api.modrinth.com/v2/project/${mod}/version`;

  const versions = (await fetch(url).then((res) => res.json())) as Version[];

  const version = versions[0].files.find((f) => f.primary);

  if (!version) break;

  const res = await fetch(version.url);

  const buf = await res.arrayBuffer();

  await fs.writeFile(
    path.join(mods.folder, version.filename),
    Buffer.from(buf)
  );

  console.log(`Downloaded ${version.filename}`);
}
