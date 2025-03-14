export function streamToBuffer(stream: NodeJS.ReadableStream): Promise<Buffer> {
  return new Promise<Buffer>((resolve, reject) => {
    const _buf: Buffer[] = [];

    stream.on("data", (chunk) => _buf.push(chunk));
    stream.once("end", () => resolve(Buffer.concat(_buf)));
    stream.once("error", (err) => reject(err));
  });
}
