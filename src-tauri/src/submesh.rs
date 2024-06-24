use obj::{Obj, Position};
/**
 * submesh from jaw mesh to teeth mesh
 */
use std::collections::{HashMap, HashSet};

pub async fn extract_submesh(
    original_obj: &Obj<Position, u32>,
    subverts: Vec<u32>,
) -> Obj<Position, u32> {
    let mut sub_vertices: Vec<Position> = Vec::new();
    let mut sub_indices: Vec<u32> = Vec::new();
    let mut vertex_map: HashMap<u32, u32> = HashMap::new(); // Original vertex index to new index map

    // HashSet to keep track of vertices already added to submesh
    let mut sub_vertex_set: HashSet<u32> = HashSet::new();

    // Process each triangle defined by original_obj's indices
    for chunk in original_obj.indices.chunks_exact(3) {
        let triangle_indices = [chunk[0], chunk[1], chunk[2]];
        let mut contains_subvert = false;

        // Check if the triangle contains any subvert
        for subvert in &subverts {
            if triangle_indices.contains(&subvert) {
                contains_subvert = true;
                break;
            }
        }

        // If the triangle contains any subvert, add all its vertices to submesh
        if contains_subvert {
            for &orig_index in &triangle_indices {
                if !sub_vertex_set.contains(&orig_index) {
                    let original_vertex = &original_obj.vertices[orig_index as usize];
                    let new_index = sub_vertices.len() as u32;
                    vertex_map.insert(orig_index, new_index);
                    sub_vertices.push(original_vertex.clone());
                    sub_vertex_set.insert(orig_index);
                }
                sub_indices.push(*vertex_map.get(&orig_index).unwrap());
            }
        }
    }

    Obj {
        name: None,
        vertices: sub_vertices,
        indices: sub_indices,
    }
}

pub async fn submesh_from_obj(
    mesh: &Obj<Position, u32>,
    sub_verts: Vec<u32>,
) -> Obj<Position, u32> {
    let mut sub_faces = vec![];
    let mut sub_vertex_set = HashSet::new();

    for f_ind in 0..mesh.indices.len() / 3 {
        if sub_verts.contains(&mesh.indices[3 * f_ind + 0])
            && sub_verts.contains(&mesh.indices[3 * f_ind + 1])
            && sub_verts.contains(&mesh.indices[3 * f_ind + 2])
        {
            sub_faces.push(mesh.indices[3 * f_ind + 0]);
            sub_faces.push(mesh.indices[3 * f_ind + 1]);
            sub_faces.push(mesh.indices[3 * f_ind + 2]);
            sub_vertex_set.insert(mesh.indices[3 * f_ind + 0]);
            sub_vertex_set.insert(mesh.indices[3 * f_ind + 1]);
            sub_vertex_set.insert(mesh.indices[3 * f_ind + 2]);
        }
    }

    let sub_vertices: Vec<Position> = mesh
        .vertices
        .iter()
        .enumerate()
        .filter(|(idx, _)| sub_vertex_set.contains(&(*idx as u32)))
        .map(|(_, v)| v.clone())
        .collect();

    Obj {
        name: None,
        vertices: sub_vertices,
        indices: sub_faces,
    }
}
