use obj::{Obj, Position};
use align3d::pointcloud::PointCloud;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type DbMesh = Arc<Mutex<HashMap<String, Obj<Position, u32>>>>;
type DbPcd = Arc<Mutex<HashMap<String, PointCloud>>>;

static GLOBAL_DB_MESH: Lazy<DbMesh> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
static GLOBAL_DB_PCD: Lazy<DbPcd> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn insert_mesh(key: String, value: Obj<Position, u32>) {
    let db = GLOBAL_DB_MESH.clone();
    let mut data = db.lock().unwrap();
    data.insert(key, value);
}

pub fn get_mesh(key: &str) -> Option<Obj<Position, u32>> {
    let db = GLOBAL_DB_MESH.clone();
    let data = db.lock().unwrap();
    let some_obj = data.get(key);

    if let Some(obj) = some_obj {
        return Some(Obj {
            name: obj.name.clone(),
            vertices: obj.vertices.clone(),
            indices: obj.indices.clone()
        });
    }
    None
}

pub fn remove_mesh(key: &str) -> Option<Obj<Position, u32>> {
    let db = GLOBAL_DB_MESH.clone();
    let mut data = db.lock().unwrap();
    data.remove(key)
}

pub fn insert_pcd(key: String, value: PointCloud) {
    let db = GLOBAL_DB_PCD.clone();
    let mut data = db.lock().unwrap();
    data.insert(key, value);
}

pub fn get_pcd(key: &str) -> Option<PointCloud> {
    let db = GLOBAL_DB_PCD.clone();
    let data = db.lock().unwrap();
    let some_pcd = data.get(key);

    if let Some(pcd) = some_pcd {
        return Some(PointCloud {
            points: pcd.points.clone(),
            normals: pcd.normals.clone(),
            colors: None
        });
    }
    None
}

pub fn remove_pcd(key: &str) -> Option<PointCloud> {
    let db = GLOBAL_DB_PCD.clone();
    let mut data = db.lock().unwrap();
    data.remove(key)
}
