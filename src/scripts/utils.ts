
export function getFileNameFromPath(filePath: string) {
  const pathSeparator = filePath.includes("/") ? "/" : "\\";
  const fileName = filePath.split(pathSeparator).pop();
  return fileName;
}
