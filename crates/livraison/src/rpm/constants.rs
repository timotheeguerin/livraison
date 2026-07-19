//! Constants for the RPM binary format.
//!
//! Values taken from the RPM file format specification and the `rpm` sources
//! (`lib/rpmtag.h`).

/// Magic bytes at the start of the lead section.
pub const RPM_MAGIC: [u8; 4] = [0xed, 0xab, 0xee, 0xdb];

/// Magic bytes at the start of every header structure (signature and main).
pub const HEADER_MAGIC: [u8; 3] = [0x8e, 0xad, 0xe8];

/// Size in bytes of an index header and of a single index entry.
pub const INDEX_ENTRY_SIZE: i32 = 16;

/// Region tag marking the boundary of the immutable main header.
pub const RPMTAG_HEADERIMMUTABLE: u32 = 63;

/// Region tag marking the boundary of the signature header.
pub const RPMTAG_HEADERSIGNATURES: u32 = 62;

/// Main header tags.
pub const RPMTAG_NAME: u32 = 1000;
pub const RPMTAG_VERSION: u32 = 1001;
pub const RPMTAG_RELEASE: u32 = 1002;
pub const RPMTAG_SUMMARY: u32 = 1004;
pub const RPMTAG_DESCRIPTION: u32 = 1005;
pub const RPMTAG_SIZE: u32 = 1009;
pub const RPMTAG_LICENSE: u32 = 1014;
pub const RPMTAG_OS: u32 = 1021;
pub const RPMTAG_ARCH: u32 = 1022;
pub const RPMTAG_FILESIZES: u32 = 1028;
pub const RPMTAG_FILEMODES: u32 = 1030;
pub const RPMTAG_FILEMTIMES: u32 = 1034;
pub const RPMTAG_FILEDIGESTS: u32 = 1035;
pub const RPMTAG_FILEFLAGS: u32 = 1037;
pub const RPMTAG_FILEUSERNAME: u32 = 1039;
pub const RPMTAG_FILEGROUPNAME: u32 = 1040;
pub const RPMTAG_RPMVERSION: u32 = 1064;
pub const RPMTAG_DIRINDEXES: u32 = 1116;
pub const RPMTAG_BASENAMES: u32 = 1117;
pub const RPMTAG_DIRNAMES: u32 = 1118;
pub const RPMTAG_PAYLOADFORMAT: u32 = 1124;
pub const RPMTAG_PAYLOADCOMPRESSOR: u32 = 1125;
pub const RPMTAG_PAYLOADFLAGS: u32 = 1126;
pub const RPMTAG_FILEDIGESTALGO: u32 = 5011;
pub const RPMTAG_ENCODING: u32 = 5062;
/// SHA-256 digest of the (compressed) payload archive, as a hex string array.
pub const RPMTAG_PAYLOADDIGEST: u32 = 5092;
/// Digest algorithm identifier for `RPMTAG_PAYLOADDIGEST`.
pub const RPMTAG_PAYLOADDIGESTALGO: u32 = 5093;

/// Signature header tags.
pub const RPMSIGTAG_SIZE: u32 = 1000;
pub const RPMSIGTAG_SHA256: u32 = 273;

/// File digest algorithm identifier for SHA-256 (`RPMTAG_FILEDIGESTALGO`).
pub const RPM_DIGEST_ALGO_SHA256: u32 = 8;
