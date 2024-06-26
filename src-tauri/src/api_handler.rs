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
use crate::submesh::create_submeshes;

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
    label: &String,
) -> Result<Obj<Position, u32>, Box<dyn std::error::Error>> {
    let url = Url::parse_with_params(
        "https://dental.scubot.com/restoration/extract",
        &[("token", &token), ("label", &label)],
    )?;
    let resp = client
        .post(url)
        .basic_auth("GET MY TOKEN", Some("b19aa580-90e1-4f32-b065-e5f4c1b9c2cd"))
        .send()
        .await?;

    let content = Cursor::new(resp.text().await?);

    let buf_read = std::io::BufReader::new(content);

    let model: Obj<Position, u32> = match load_obj(buf_read) {
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

pub async fn request_api_simple_with_label(
    client: &reqwest::Client,
    api: &str,
    token: &String,
    label: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = Url::parse_with_params(
        format!(
            "{}{}",
            String::from("https://dental.scubot.com"),
            String::from(api)
        )
        .as_str(),
        &[("token", &token), ("label", &label)],
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
pub async fn backend_restore_tooth(token: String, label: u32) -> Obj<Position, u32> {
    let client = reqwest::Client::new();

    if let Some(obj) = get_mesh(&token) {
        let mut _ply = convert_obj_to_ply(&obj).await;
        let mut buf = Vec::<u8>::new();
        let _ = Writer::new().write_ply(&mut buf, &mut _ply).unwrap();

        let _token = match upload_file(&client, buf).await {
            Err(why) => panic!("Err {:?}", why),
            Ok(token) => token,
        };

        _ = match request_api_simple_with_label(
            &client,
            "/restoration/preprocess",
            &_token,
            &format!("{}", &label),
        )
        .await
        {
            Err(why) => panic!("Err {:?}", why),
            Ok(msg) => msg,
        };

        _ = match request_api_simple_with_label(
            &client,
            "/restoration/embedding",
            &_token,
            &format!("{}", &label),
        )
        .await
        {
            Err(why) => panic!("Err {:?}", why),
            Ok(msg) => msg,
        };

        let model = match download_file(&client, &_token, &format!("{}", &label)).await {
            Err(why) => panic!("Err {:?}", why),
            Ok(model) => model,
        };

        model
    } else {
        Obj {
            name: Some(String::from("err")),
            vertices: Vec::new(),
            indices: vec![0u32],
        }
    }
}

#[tauri::command]
pub async fn backend_submeshes(
    token: String,
    labels: Vec<u32>,
) -> HashMap<u32, Obj<Position, u32>> {
    if let Some(mesh) = get_mesh(&token) {
        let submeshes = create_submeshes(&mesh, labels).await;

        for (label, submesh) in &submeshes {
            // local save
            insert_mesh(format!("autoseg_{}", label), clone_obj(&submesh));
        }

        submeshes
    } else {
        HashMap::new()
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
        Some(mesh) => mesh,
    }
}
