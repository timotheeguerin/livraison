import { Readable } from "node:stream";
import { HeaderDimensions, MagicHeader } from "./common.js";

export interface WriteArOptions {
  /** Default user id for all files @default 0 */
  uid?: number;
  /** Default group id for all files @default 0 */
  gid?: number;
  /** Default model for all files @default 644 */
  mode?: number;
}

export interface ArEntryOptions {
  /** Filename */
  name: string;
  /** Mode */
  mode?: number;
  /** User id */
  uid?: number;
  /** Group id */
  gid?: number;
}

/**
 * Write an ar archive.
 */
export class ArArchiveWriter extends Readable {
  #uid: number;
  #gid: number;
  #mode: number;
  #finalized = false;
  #queue: Buffer[] = [];

  constructor(options: WriteArOptions = {}) {
    super();
    const resolved = {
      uid: 0,
      gid: 0,
      mode: 644,
      ...options,
    };

    this.#uid = resolved.uid;
    this.#gid = resolved.gid;
    this.#mode = resolved.mode;
    this.#queue.push(Buffer.from(MagicHeader, "ascii"));
  }

  add(fileBuffer: Buffer, options: ArEntryOptions) {
    const name = options.name;
    const size = fileBuffer.length;

    const stats = {
      mtime: new Date(),
      uid: this.#uid,
      gid: this.#gid,
      mode: this.#mode,
      ...options,
    };

    const paddedSize = getPaddingBytes(size, 2);
    if (paddedSize > 0) {
      fileBuffer = Buffer.concat([fileBuffer, Buffer.from(padLF(paddedSize), "ascii")], size + paddedSize);
    }

    const header = buildHeader(name, "0", stats.uid + "", stats.gid + "", stats.mode!.toString(8), size + "");
    this.#queue.push(Buffer.concat([header, fileBuffer]));

    return this;
  }

  finalize() {
    this.#finalized = true;
  }

  _read() {
    while (this.#queue.length > 0) {
      this.push(this.#queue.shift());
    }
    if (this.#finalized) {
      this.push(null);
      return;
    }
  }
}

function buildHeader(name: string, ts: string, uid: string, gid: string, mode: string, size: string) {
  const header =
    strictWidthField(name, HeaderDimensions.name) +
    strictWidthField(ts, HeaderDimensions.date) +
    strictWidthField(uid, HeaderDimensions.uid) +
    strictWidthField(gid, HeaderDimensions.gid) +
    strictWidthField(mode, HeaderDimensions.mode) +
    strictWidthField(size, HeaderDimensions.dataSize) +
    "`\n";

  return Buffer.from(header, "ascii");
}

function strictWidthField(str: string, width: number) {
  if (str.length > width) {
    return str.substring(0, width);
  } else {
    return padWhitespace(str, width);
  }
}

/**
 * Given something of size *size* bytes that needs to be aligned by *alignment*
 * bytes, returns the total number of padding bytes that need to be appended to
 * the end of the data.
 */
function getPaddingBytes(size: number, alignment: number) {
  return (alignment - (size % alignment)) % alignment;
}

function padWhitespace(str: string, width: number) {
  return str.padEnd(width, " ");
}

function padLF(width: number) {
  let str = "";
  while (str.length < width) {
    str += "\n";
  }
  return str;
}
