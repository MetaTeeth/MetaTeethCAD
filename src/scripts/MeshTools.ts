import { convertFileSrc } from "@tauri-apps/api/tauri";
import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
import { STLLoader } from "three/addons/loaders/STLLoader.js";
import { PLYLoader } from "three/addons/loaders/PLYLoader.js";
import { PLYExporter } from "three/addons/exporters/PLYExporter.js";
import { MeshSurfaceSampler } from 'three/addons/math/MeshSurfaceSampler.js';
import {
  Object3D, BufferGeometry, MeshPhongMaterial,
  Color, DoubleSide, Mesh, Vector3
} from "three"; 

// reload
const loaderAdapter = new Map();
loaderAdapter.set("obj", OBJLoader);
loaderAdapter.set("stl", STLLoader);
loaderAdapter.set("ply", PLYLoader);


export function loadMeshUtil(
  filePath: string,
  onSuccess: Function = () => {},
  onProcess: Function = () => {},
  onError: Function = () => {},
) {
  const extname = filePath.substring(filePath.lastIndexOf(".") + 1).toLowerCase();
  if (loaderAdapter.has(extname)) {
    const loader = new (loaderAdapter.get(extname))();
    loader.load(
      convertFileSrc(filePath),
      (object: any) => {
        let geometry = null;
        if (object instanceof Object3D) {
          geometry = (object.children[0] as Mesh).geometry;
        } else if (object instanceof BufferGeometry) {
          geometry = object;
        } else {
          return;
        }
        // geometry.dynamic = true;
        geometry.computeVertexNormals();
        const material = new MeshPhongMaterial({
          color: new Color("rgb(230, 230, 230)"),
          side: DoubleSide,
        });
        const mesh = new Mesh(geometry, material);
        mesh.receiveShadow = true;
        mesh.name = mesh.uuid;

        onSuccess(mesh);
      },
      (xhr: any) => { onProcess(xhr); },
      (err: any) => { onError(err); }
    );
  }
  else {
    onError(`${extname} not supported`);
  }
}

export function exportPLY(object3D: Object3D) {
  const exporter = new PLYExporter();
  const data = exporter.parse(
    object3D,
    () => {},
    { binary: true },
  );
  return data;
}

export function sampleMesh(mesh: Mesh, count: number): [number[], number[]] {
  const verts = [];
  const norms = [];
  const sampler = new MeshSurfaceSampler(mesh).build();
  const sampledPos = new Vector3();
  const sampledNorm = new Vector3();
  for (let _ = 0; _ < count; ++_) {
    sampler.sample(sampledPos, sampledNorm);
    verts.push(sampledPos.x, sampledPos.y, sampledPos.z);
    norms.push(sampledPos.x, sampledPos.y, sampledPos.z);
  }
  return [verts, norms];
}

export function shiftCentroid(source: Mesh, target: Mesh): Mesh {
  source.geometry.computeBoundingBox();
  target.geometry.computeBoundingBox();

  let srcCenter = new Vector3();
  let tgtCenter = new Vector3();

  source.geometry.boundingBox.getCenter(srcCenter);
  source.localToWorld(srcCenter);
  target.geometry.boundingBox.getCenter(tgtCenter);
  target.localToWorld(tgtCenter);

  const trans = new Vector3();
  trans.subVectors(tgtCenter, srcCenter);

  source.translateX(trans.x).translateY(trans.y).translateZ(trans.z);

  return source;
}