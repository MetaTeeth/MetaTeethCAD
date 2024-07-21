import { convertFileSrc } from "@tauri-apps/api/tauri";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { STLLoader } from "three/addons/loaders/STLLoader.js";
import { PLYLoader } from "three/addons/loaders/PLYLoader.js";


const loaderAdapter = new Map();
loaderAdapter.set("obj", OBJLoader);
loaderAdapter.set("stl", STLLoader);
loaderAdapter.set("ply", PLYLoader);

export function load_mesh(
    filePath: string,
    success_func: Function = () => {},
    process_func: Function = () => {},
    error_func: Function = () => {}
) {
  const extname = filePath.substring(filePath.lastIndexOf(".") + 1).toLowerCase();
  if (loaderAdapter.has(extname)) {
    const loader = new (loaderAdapter.get(extname))();
    loader.load(
      convertFileSrc(filePath),
      (obj: any) => { success_func(obj); },
      (xhr: any) => { process_func(xhr); },
      (err: any) => { error_func(err); }
    );
  }
  else {
    error_func(`${extname} not supported`);
  }
}
