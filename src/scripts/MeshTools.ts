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
import icpts from "icpts";
import { StandardJawPCD } from "./MeshStore";

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

export function sampleMesh(mesh: Mesh, count: number): number[] {
  const vertices = [];
  const sampler = new MeshSurfaceSampler(mesh).build();
  const sampledPos = new Vector3();
  for (let _ = 0; _ < count; ++_) {
    sampler.sample(sampledPos);
    vertices.push(sampledPos.x, sampledPos.y, sampledPos.z);
  }
  return vertices;
}

export function registrationJaw(mesh: Mesh) {
  const sourcePCD = sampleMesh(mesh, StandardJawPCD.length / 3);

  const options = {
    // initialPose: IDENTITY, // [1, 0, 0, 0, 0, 1, ...]
    tolerance: 1e-10,
    maxIterations: 50
  };
  const { transform, error } = icpts.pointToPoint(sourcePCD, StandardJawPCD, options); // or icpts.pointToPlane

  console.log(transform, error);
}