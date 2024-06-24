use obj::{load_obj, Obj, Position};
use ply_rs::writer::Writer;
use reqwest;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
/**
 * API DOC: https://dental.scubot.com/docs
 */
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use url::Url;

use crate::converter::convert_obj_to_ply;
use crate::db::{get_mesh, insert_mesh};
use crate::submesh::extract_submesh;

fn clone_obj(obj: &Obj<Position, u32>) -> Obj<Position, u32> {
    Obj {
        name: obj.name.clone(),
        vertices: obj.vertices.clone(),
        indices: obj.indices.clone(),
    }
}

#[derive(Deserialize, Debug)]
pub struct RespToken {
    token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OBJPack {
    obj: Obj<Position, u32>,
    token: String,
}

pub async fn download_file(
    client: &reqwest::Client,
    token: &String,
) -> Result<Obj, Box<dyn std::error::Error>> {
    let url = Url::parse_with_params(
        "https://dental.scubot.com/restoration/extract",
        &[("token", &token)],
    )?;
    let resp = client
        .post(url)
        .basic_auth("GET MY TOKEN", Some("b19aa580-90e1-4f32-b065-e5f4c1b9c2cd"))
        .send()
        .await?;

    let content = Cursor::new(resp.text().await?);

    let buf_read = std::io::BufReader::new(content);

    let model: Obj = match load_obj(buf_read) {
        Err(why) => panic!("couldn't load {:?}", why),
        Ok(model) => model,
    };

    Ok(model)
}

pub async fn upload_file(
    client: &reqwest::Client,
    file_bytes: Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    // 一定要有file_name方法，且参数不能为空，否则数据上传失败
    let part = reqwest::multipart::Part::bytes(Cow::from(file_bytes)).file_name("defect.obj");
    let form = reqwest::multipart::Form::new().part("file", part);

    let resp = client
        .post("https://dental.scubot.com/register")
        .basic_auth("GET MY TOKEN", Some("b19aa580-90e1-4f32-b065-e5f4c1b9c2cd"))
        .multipart(form)
        .send()
        .await?;

    if resp.status() != 200 {
        return Err(resp.text().await?.into());
    }

    let resp_content = resp.json::<RespToken>().await?;

    Ok(resp_content.token)
}

pub async fn request_api_simple(
    client: &reqwest::Client,
    api: &str,
    token: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = Url::parse_with_params(
        format!(
            "{}{}",
            String::from("https://dental.scubot.com"),
            String::from(api)
        )
        .as_str(),
        &[("token", &token)],
    )?;

    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .basic_auth("GET MY TOKEN", Some("b19aa580-90e1-4f32-b065-e5f4c1b9c2cd"))
        .send()
        .await?;

    if resp.status() != 200 {
        return Err(resp.text().await?.into());
    }

    Ok(String::from("Ok"))
}

#[tauri::command]
pub async fn backend_segment_jaw(token: String, jawkind: String) -> HashMap<String, Vec<u32>> {
    let client = reqwest::Client::new();
    let url = Url::parse_with_params(
        "https://dental.scubot.com/segmentation",
        &[("token", &token), ("jaw_kind", &jawkind)],
    )
    .unwrap();

    let labels = client
        .post(url)
        .header("Content-Type", "application/json")
        .basic_auth("GET MY TOKEN", Some("b19aa580-90e1-4f32-b065-e5f4c1b9c2cd"))
        .send()
        .await
        .unwrap()
        .json::<HashMap<String, Vec<u32>>>()
        .await
        .unwrap();

    labels
}

#[tauri::command]
pub async fn backend_load_obj(file_path: String) -> OBJPack {
    let input = BufReader::new(match File::open(&file_path) {
        Err(why) => panic!("couldn't open {:?}", why),
        Ok(file) => file,
    });
    let model: Obj<Position, u32> = match load_obj(input) {
        Err(why) => panic!("couldn't load {:?}", why),
        Ok(model) => model,
    };

    let mut _ply = convert_obj_to_ply(&model).await;
    let mut buf = Vec::<u8>::new();
    let _ = Writer::new().write_ply(&mut buf, &mut _ply).unwrap();

    let client = reqwest::Client::new();
    let token = match upload_file(&client, buf).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(token) => token,
    };

    // save obj to db
    insert_mesh(token.clone(), clone_obj(&model));

    OBJPack {
        obj: model,
        token: token,
    }
}

#[tauri::command]
pub async fn backend_register_obj(file_path: String) -> String {
    let client = reqwest::Client::new();
    let file_bytes = std::fs::read(file_path).unwrap();
    let token = match upload_file(&client, file_bytes).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(token) => token,
    };
    token
}

#[tauri::command]
pub async fn backend_restore_preprocess(token: String) {
    let client = reqwest::Client::new();
    _ = match request_api_simple(&client, "/restoration/preprocess", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg,
    };
}

#[tauri::command]
pub async fn backend_restore_embedding(token: String) {
    let client = reqwest::Client::new();
    _ = match request_api_simple(&client, "/restoration/embedding", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg,
    };
}

#[tauri::command]
pub async fn backend_restore_download(token: String) -> Obj {
    let client = reqwest::Client::new();
    let model = match download_file(&client, &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(model) => model,
    };
    model
}

#[tauri::command]
pub async fn backend_restore_full(file_path: String) -> Obj {
    let client = reqwest::Client::new();

    let file_bytes = std::fs::read(file_path).unwrap();
    let token = match upload_file(&client, file_bytes).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(token) => token,
    };

    println!("Token => {}", token);

    _ = match request_api_simple(&client, "/restoration/preprocess", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg,
    };
    println!("Preprocess done.");

    _ = match request_api_simple(&client, "/restoration/embedding", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg,
    };
    println!("Emebedding done.");

    let model = match download_file(&client, &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(model) => model,
    };

    model
}

#[tauri::command]
pub async fn backend_submesh(
    token: String,
    subverts: Vec<u32>,
    label: String,
) -> Obj<Position, u32> {
    if let Some(mesh) = get_mesh(&token) {
        let submesh = extract_submesh(&mesh, subverts).await;
        insert_mesh(format!("autoseg_{}", label), clone_obj(&submesh));
        return submesh;
    }
    Obj {
        name: None,
        vertices: Vec::new(),
        indices: vec![0u32],
    }
}

#[tauri::command]
pub async fn backend_getmesh(token: String) -> Obj<Position, u32> {
    match get_mesh(&token) {
        None => Obj {
            name: Some(String::from("err")),
            vertices: Vec::new(),
            indices: vec![0u32],
        },
        Some(mesh) => mesh
    }
}
