use obj::{Obj, Position};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Obj<Position, u32>>>>;

static GLOBAL_DB: Lazy<Db> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn insert_mesh(key: String, value: Obj<Position, u32>) {
    let db = GLOBAL_DB.clone();
    let mut data = db.lock().unwrap();
    data.insert(key, value);
}

pub fn get_mesh(key: &str) -> Option<Obj<Position, u32>> {
    let db = GLOBAL_DB.clone();
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
    let db = GLOBAL_DB.clone();
    let mut data = db.lock().unwrap();
    data.remove(key)
}
