import { HeaderDimensions, MagicHeader } from "./common.js";

/**
 * Contains information from a file's header.
 */
export interface ArFile {
  /** Filename */
  name(): string;
  /** File modification date */
  date(): Date;
  /** User id */
  uid(): number;
  /** Group id */
  gid(): number;
  /** File mode */
  mode(): number;
  /** File size */
  size(): number;
  /** Size of header */
  headerSize(): number;
  /** Total size in archive */
  totalSize(): number;
  /** Content buffer */
  content(): Buffer;
}

export class ArArchiveReader {
  #files: ArFile[] = [];

  constructor(private data: Buffer) {
    const magicHeaderLength = MagicHeader.length;
    if (data.toString("utf8", 0, magicHeaderLength) !== MagicHeader) {
      throw new Error(`Invalid ar file: Missing magic header '${MagicHeader}'`);
    }

    let offset = magicHeaderLength;
    let file: ArFile;
    while (offset < this.data.length) {
      file = new BsdArFile(this.data.subarray(offset));
      this.#files.push(file);
      offset += file.totalSize();
    }
  }

  /**
   * Get an array of the files in the archive.
   */
  public get files(): ArFile[] {
    return this.#files;
  }
}

/**
 * Given something of size *size* bytes that needs to be aligned by *alignment*
 * bytes, returns the total number of padding bytes that need to be appended to
 * the end of the data.
 */
function getPaddingBytes(size: number, alignment: number): number {
  return (alignment - (size % alignment)) % alignment;
}

/**
 * Trims trailing NULL characters.
 */
function trimNulls(str: string): string {
  return str.replace(/\0/g, "");
}

/**
 * All archive variants share this header before files, but the variants differ
 * in how they handle odd cases (e.g. files with spaces, long filenames, etc).
 *
 * char    ar_name[16]; File name
 * char    ar_date[12]; file member date
 * char    ar_uid[6]    file member user identification
 * char    ar_gid[6]    file member group identification
 * char    ar_mode[8]   file member mode (octal)
 * char    ar_size[10]; file member size
 * char    ar_fmag[2];  header trailer string
 */
export class ArCommonFile implements ArFile {
  constructor(public data: Buffer) {
    if (this.fmag() !== "`\n") {
      throw new Error("Record is missing header trailer string; instead, it has: " + this.fmag());
    }
  }

  public name(): string {
    // The name field is padded by whitespace, so trim any lingering whitespace.
    return this.data.toString("utf8", 0, HeaderDimensions.name).trimEnd();
  }
  public date(): Date {
    return new Date(parseInt(this.data.toString("ascii", 16, 28), 10));
  }
  public uid(): number {
    return parseInt(this.data.toString("ascii", 28, 34), 10);
  }
  public gid(): number {
    return parseInt(this.data.toString("ascii", 34, 40), 10);
  }
  public mode(): number {
    return parseInt(this.data.toString("ascii", 40, 48), 8);
  }
  /**
   * Total size of the data section in the record. Does not include padding bytes.
   */
  public size(): number {
    return parseInt(this.data.toString("ascii", 48, 58), 10);
  }

  private fmag(): string {
    return this.data.toString("ascii", 58, 60);
  }

  /**
   * Total size of the header, including padding bytes.
   */
  public headerSize(): number {
    // The common header is already two-byte aligned.
    return 60;
  }
  /**
   * Total size of this file record (header + header padding + file data +
   * padding before next archive member).
   */
  public totalSize(): number {
    const headerSize = this.headerSize();
    const dataSize = this.size();
    // All archive members are 2-byte aligned, so there's padding bytes after
    // the data section.
    return headerSize + dataSize + getPaddingBytes(dataSize, 2);
  }

  /**
   * Returns a *slice* of the backing buffer that has all of the file's data.
   */
  public content(): Buffer {
    const headerSize = this.headerSize();
    return this.data.subarray(headerSize, headerSize + this.size());
  }
}

/**
 * BSD variant of the file header.
 */
export class BsdArFile extends ArCommonFile implements ArFile {
  private appendedFileName: boolean;
  constructor(data: Buffer) {
    super(data);
    // Check if the filename is appended to the header or not.
    this.appendedFileName = super.name().substring(0, 3) === "#1/";
  }
  /**
   * Returns the number of bytes that the appended name takes up in the content
   * section.
   */
  private appendedNameSize(): number {
    if (this.appendedFileName) {
      return parseInt(super.name().substring(3), 10);
    }
    return 0;
  }
  /**
   * BSD ar stores extended filenames by placing the string "#1/" followed by
   * the file name length in the file name field.
   *
   * Note that this is unambiguous, as '/' is not a valid filename character.
   */
  public name(): string {
    let length,
      name = super.name(),
      headerSize;
    if (this.appendedFileName) {
      length = this.appendedNameSize();
      // The filename is stored right after the header.
      headerSize = super.headerSize();
      // Unfortunately, even though they give us the *explicit length*, they add
      // NULL bytes and include that in the length, so we must strip them out.
      name = trimNulls(this.data.toString("utf8", headerSize, headerSize + length));
    }
    return name;
  }
  /**
   * dataSize = appendedNameSize + fileSize
   */
  public fileSize(): number {
    return this.size() - this.appendedNameSize();
  }
  /**
   * Returns a *slice* of the backing buffer that has all of the file's data.
   * For BSD archives, we need to add in the size of the file name, which,
   * unfortunately, is included in the fileSize number.
   */
  public content(): Buffer {
    const headerSize = this.headerSize(),
      appendedNameSize = this.appendedNameSize();
    return this.data.subarray(headerSize + appendedNameSize, headerSize + appendedNameSize + this.fileSize());
  }
}
