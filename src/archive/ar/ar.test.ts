import { expect, it } from "vitest";
import { streamTobuffer } from "../../utils/stream.js";
import { ArArchiveReader } from "./read-ar.js";
import { ArArchiveWriter } from "./write-ar.js";

it("round trip", async () => {
  const writer = new ArArchiveWriter();
  writer.add(Buffer.from("hello"), { name: "hello.txt" });
  writer.add(Buffer.from("world"), { name: "world.txt" });
  writer.finalize();
  const buffer = await streamTobuffer(writer);
  const reader = new ArArchiveReader(buffer);
  const files = reader.files;
  expect(files.length).toBe(2);
  expect(files[0].name()).toBe("hello.txt");
  expect(files[0].content().toString()).toBe("hello");
  expect(files[1].name()).toBe("world.txt");
  expect(files[1].content().toString()).toBe("world");
});
