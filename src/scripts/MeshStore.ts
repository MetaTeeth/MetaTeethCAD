import { resolveResource } from '@tauri-apps/api/path'
import { loadMeshUtil } from './MeshTools';
import { Object3D, Mesh } from 'three';

function preloadStandard(func: Function) {
    resolveResource('resources/STANDARD_JAW.ply').then(
        resourcePath => {
            loadMeshUtil(
                resourcePath, 
                (object3D: Mesh) => {
                    console.log('===== PRELOADED =====');
                    func(object3D);
                }
            )
        }
    )
}

export let STANDARD_JAW: Object3D;

preloadStandard((obj: Object3D) => { STANDARD_JAW = obj; });
