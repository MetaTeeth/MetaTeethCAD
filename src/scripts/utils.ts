import jsSHA from 'jssha';

const _SALT_ = new TextEncoder().encode('deadbeef');

export function getFileNameFromPath(filePath: string) {
  const pathSeparator = filePath.includes("/") ? "/" : "\\";
  const fileName = filePath.split(pathSeparator).pop();
  return fileName;
}

export function getHashToken(bin: ArrayBuffer): string {
  const _u8bin = new Uint8Array(bin);
  const concatBuffer = new Uint8Array(_u8bin.length + _SALT_.length);
  concatBuffer.set(_u8bin);
  concatBuffer.set(_SALT_, _u8bin.length);

  const shasum = new jsSHA("SHA-1", "UINT8ARRAY");
  shasum.update(concatBuffer);
  return shasum.getHash('HEX');
}