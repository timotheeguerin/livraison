import { createReadStream, createWriteStream } from "node:fs";
import { stat } from "node:fs/promises";
import { finished } from "node:stream/promises";
import tar from "tar-stream";
import { createGzip } from "zlib";
import { ArArchiveWriter } from "../../archive/ar/write-ar.js";
import { streamToBuffer } from "../../utils/stream.js";
import type { DataFile, DebOptions } from "./types.js";

export async function packDebArchive(destination: string, options: DebOptions) {
  const deb = new ArArchiveWriter();
  const out = createWriteStream(destination);
  const controlBuffer = await generateControl(options);
  const dataBuffer = await createDataArchive(options);

  deb.add(generateDebianBinary(), {
    name: "debian-binary",
  });
  deb.add(controlBuffer, {
    name: "control.tar.gz",
  });
  deb.add(dataBuffer, {
    name: "data.tar.gz",
  });

  deb.finalize();

  await finished(deb.pipe(out));
}

// async function extractDeb(file: string) {
//   const foo = new ArArchiveReader(await readFile(file));
//   for (const file of foo.files) {
//     await mkdir("temp/deb-out", { recursive: true });
//     await writeFile(`temp/deb-out/${file.name()}`, file.content());
//   }
// }

async function createDataArchive(options: DebOptions): Promise<Buffer> {
  const archive = tar.pack();

  const dirAdded = new Set<string>();
  function addIntermediateDirs(path: string) {
    const parts = path.split("/");
    for (let i = 0; i < parts.length; i++) {
      const dir = parts.slice(0, i + 1).join("/");
      if (!dirAdded.has(dir)) {
        archive.entry({ type: "directory", name: dir });
        dirAdded.add(dir);
      }
    }
  }

  const allFiles = [...(options.conffiles ?? []), ...(options.files ?? [])];
  for (const file of allFiles) {
    const archivePath = stripLeadingSlash(file.archivePath);

    addIntermediateDirs(archivePath);
    if ("content" in file) {
      archive.entry(
        {
          type: "file",
          name: archivePath,
          mode: file.stats?.mode,
        },
        file.content,
      );
    } else {
      const fileStat = await stat(file.localPath);
      const content = createReadStream(file.localPath);
      content.pipe(
        archive.entry({
          type: "file",
          name: archivePath,
          mode: fileStat.mode,
          size: fileStat.size,
          mtime: fileStat.mtime,
        }),
      );
      await finished(content);
    }
  }

  const bufferPromise = streamToBuffer(archive.pipe(createGzip()));
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

  if (options.conffiles && options.conffiles.length > 0) {
    archive.entry({ name: "conffiles" }, generateConffiles(options.conffiles));
  }

  const bufferPromise = streamToBuffer(archive.pipe(createGzip()));
  archive.finalize();

  return bufferPromise;
}

function generateConffiles(conffiles: DataFile[]) {
  return conffiles.map((file) => file.archivePath).join("\n") + "\n";
}

function stripLeadingSlash(path: string) {
  return path[0] === "/" ? path.slice(1) : path;
}

function generateDebianBinary() {
  return Buffer.from("2.0\n");
}

function createControlFile(options: DebOptions): string {
  options = { ...options, architecture: "all" };
  const items = [
    `Package: ${options.name}`,
    `Version: ${getVersion(options)}`,
    `Maintainer: ${options.maintainer.name} <${options.maintainer.email}>`,
    `Architecture: ${options.architecture}`,
    `Description: ${getDescription(options)}`,
    options.priority && `Priority: ${options.priority}`,
    options.section && `Section: ${options.section}`,
    options.depends?.length && `Depends: ${options.depends.join(", ")}`,
  ];

  return items.filter((x) => x).join("\n") + "\n";
}

function getVersion(options: DebOptions): string {
  const epoch = options.epoch ? `${options.epoch}:` : "";
  const revision = options.revision ? `-${options.revision}` : "";
  return `${epoch}${options.version}${revision}`;
}

function getDescription(options: DebOptions): string {
  const [first, ...lines] = options.description.split("\n");
  return [first, ...lines.map((line) => ` ${line}`)].join("\n");
}
