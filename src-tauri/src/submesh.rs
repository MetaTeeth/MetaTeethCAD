use obj::{Obj, Position};
/**
 * submesh from jaw mesh to teeth mesh
 */
use std::collections::HashMap;

pub async fn create_submeshes(
    obj: &Obj<Position, u32>,
    labels: Vec<u32>,
) -> HashMap<u32, Obj<Position, u32>> {
    let mut submeshes: HashMap<u32, (Vec<Position>, Vec<u32>)> = HashMap::new();

    for (_, &label) in labels.iter().enumerate() {
        if label == 0 {
            continue; // Skip label 0
        }
        submeshes
            .entry(label)
            .or_insert_with(|| (Vec::new(), Vec::new()));
    }

    for face in obj.indices.chunks(3) {
        let label = labels[face[0] as usize];
        if label == 0 {
            continue; // Skip faces with any vertex labeled 0
        }
        if labels[face[1] as usize] == 0 || labels[face[2] as usize] == 0 {
            continue;
        }

        let (vertices, indices) = submeshes.get_mut(&label).unwrap();

        for &index in face {
            let vertex = obj.vertices[index as usize];
            if let Some(pos) = vertices.iter().position(|&v| v == vertex) {
                indices.push(pos as u32);
            } else {
                vertices.push(vertex);
                indices.push((vertices.len() - 1) as u32);
            }
        }
    }

    submeshes
        .into_iter()
        .map(|(label, (vertices, indices))| {
            (
                label,
                Obj {
                    name: None,
                    vertices: vertices,
                    indices: indices,
                },
            )
        })
        .collect()
}
