/** Every ar file must start with this header */
export const MagicHeader = "!<arch>\n";

export const HeaderDimensions = {
  name: 16,
  date: 12,
  uid: 6,
  gid: 6,
  mode: 8,
  dataSize: 10,
};
