use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};
use reqwest;
use serde::Deserialize;
use std::io::Cursor;
use url::Url;
use std::borrow::Cow;


#[derive(Deserialize, Debug)]
pub struct RespToken {
    token: String
}

pub async fn download_file(client: &reqwest::Client, token: &String) -> Result<Obj, Box<dyn std::error::Error>> {
    let url = Url::parse_with_params(
        "https://dental.scubot.com/dental/restoration/extract",
        &[("token", &token)]
    )?;
    let resp = client.post(url)
    .basic_auth("GET MY TOKEN", Some("b19aa580-90e1-4f32-b065-e5f4c1b9c2cd"))
    .send()
    .await?;

    let content =  Cursor::new(resp.text().await?);

    let buf_read = std::io::BufReader::new(content);

    let model: Obj = match load_obj(buf_read) {
        Err(why) => panic!("couldn't load {:?}", why),
        Ok(model) => model,
    };

    Ok(model)
}

pub async fn upload_file(client: &reqwest::Client, file_path: &String) -> Result<String, Box<dyn std::error::Error>> {
    let file_byte=std::fs::read(file_path).unwrap();
    // 一定要有file_name方法，且参数不能为空，否则数据上传失败
    let part=reqwest::multipart::Part::bytes(Cow::from(file_byte)).file_name("defect.obj");
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

pub async fn request_api_simple(client: &reqwest::Client, api: &str, token: &String) -> Result<String, Box<dyn std::error::Error>> {
    let url = Url::parse_with_params(
        format!("{}{}", String::from("https://dental.scubot.com"), String::from(api)).as_str(), 
        &[("token", &token)]
    )?;

    let resp = client.post(url)
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
#[allow(dead_code)]
pub async fn backend_load_obj(file_path: String) -> Obj {
    let input = BufReader::new(match File::open(&file_path) {
        Err(why) => panic!("couldn't open {:?}", why),
        Ok(file) => file,
    });
    let model: Obj = match load_obj(input) {
        Err(why) => panic!("couldn't load {:?}", why),
        Ok(model) => model,
    };
    model
}

#[tauri::command]
pub async fn backend_register_obj(file_path: String) -> String {
    let client = reqwest::Client::new();
    let token = match upload_file(&client, &file_path).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(token) => token
    };
    token
}

#[tauri::command]
pub async fn backend_restore_preprocess(token: String) {
    let client = reqwest::Client::new();
    _ = match request_api_simple(&client, "/dental/restoration/preprocess", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg, 
    };
}

#[tauri::command]
pub async fn backend_restore_embedding(token: String) {
    let client = reqwest::Client::new();
    _ = match request_api_simple(&client, "/dental/restoration/embedding", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg, 
    };
}

#[tauri::command]
pub async fn backend_restore_download(token: String) -> Obj {
    let client = reqwest::Client::new();
    let model = match download_file(&client, &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(model) => model
    };
    model
}

#[tauri::command]
pub async fn backend_restore_full(file_path: String) -> Obj {
    let client = reqwest::Client::new();

    let token = match upload_file(&client, &file_path).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(token) => token
    };

    println!("Token => {}", token);

    _ = match request_api_simple(&client, "/dental/restoration/preprocess", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg, 
    };
    println!("Preprocess done.");
    
    _ = match request_api_simple(&client, "/dental/restoration/embedding", &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(msg) => msg, 
    };
    println!("Emebedding done.");
    
    let model = match download_file(&client, &token).await {
        Err(why) => panic!("Err {:?}", why),
        Ok(model) => model
    };

    model
}