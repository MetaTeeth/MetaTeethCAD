import { resolveResource } from '@tauri-apps/api/path'
import { loadMeshUtil } from './MeshTools';
import { Object3D } from 'three';

export function preloadStandard(func: Function) {
    resolveResource('resources/STANDARD_JAW.ply').then(
        resourcePath => {
            loadMeshUtil(
                resourcePath, 
                (object3D: Object3D) => {
                    console.log('===== PRELOADED =====');
                    func(object3D);
                }
            )
        }
    )
}