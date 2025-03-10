import { createWriteStream } from "node:fs";
import { mkdir, readFile, stat, writeFile } from "node:fs/promises";
import tar from "tar-stream";
import { createGzip } from "zlib";
import { ArArchiveReader } from "../../archive/ar/read-ar.js";
import { ArArchiveWriter } from "../../archive/ar/write-ar.js";
import { streamTobuffer } from "../../utils/stream.js";
import type { DebOptions } from "./types.js";

export async function pack() {
  return packDeb({
    name: "example",
    description: "An example package",
    version: "1.0.0",
    maintainer: {
      name: "Foo bar",
      email: `foo@example.com`,
    },
  });
}

export async function packDeb(options: DebOptions) {
  const deb = new ArArchiveWriter();
  const out = createWriteStream("dist/foo.deb");
  const controlBuffer = await generateControl(options);
  const dataBuffer = await createDataArchive();
  deb.add(generateDebianBinary(), {
    name: "debian-binary",
  });
  deb.add(controlBuffer, {
    name: "control.tar.gz",
  });

  deb.pipe(out);

  deb.add(dataBuffer, {
    name: "data.tar.gz",
  });

  deb.finalize();

  const p = new Promise<void>((resolve, reject) => {
    out.on("end", () => {
      resolve();
    });

    out.on("error", (er) => {
      reject(er);
    });

    deb.on("error", (er) => reject(er));
  });

  await p;
  await extractDeb("dist/foo.deb");
}

async function extractDeb(file: string) {
  const foo = new ArArchiveReader(await readFile(file));
  for (const file of foo.files) {
    await mkdir("temp/deb-out", { recursive: true });
    await writeFile(`temp/deb-out/${file.name()}`, file.content());
  }
}

async function createDataArchive(): Promise<Buffer> {
  const archive = tar.pack();

  const fileStat = await stat("dist/foo");
  const file = await readFile("dist/foo");
  archive.entry(
    {
      name: "usr/bin/foo",
      mode: fileStat.mode,
      size: fileStat.size,
      uid: 0,
      gid: 0,
      type: "file",
      mtime: fileStat.mtime,
    },
    file,
  );
  const bufferPromise = streamTobuffer(archive.pipe(createGzip()));
  archive.finalize();

  return bufferPromise;
}

function generateControl(options: DebOptions): Promise<Buffer> {
  const archive = tar.pack();

  archive.entry({ name: "control" }, Buffer.from(createControlFile(options)));

  if (options.scripts) {
    // Adding preinst, postinst, prerm, postrm to package
    for (const [script, content] of Object.entries(options.scripts)) {
      archive.entry({ name: script }, Buffer.from(content));
    }
  }

  const bufferPromise = streamTobuffer(archive.pipe(createGzip()));
  archive.finalize();

  return bufferPromise;
}

function generateDebianBinary() {
  return Buffer.from("2.0\n");
}

function createControlFile(options: DebOptions): string {
  const items = [
    `Package: ${options.name}`,
    `Version: ${options.version}`,
    `Maintainer: ${options.maintainer.name} <${options.maintainer.email}>`,
    `Architecture: arm64`,
    `Description: ${options.description}`,
  ];

  return items.join("\n") + "\n";
}
