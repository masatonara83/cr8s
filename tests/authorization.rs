use std::process::Command;

use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_login() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    print!("{:?}", output);

    let client = Client::new();
    //正しいパターン
    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!( {
            "username" : "test_admin",
            "password": "1234",
        }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);

    //認証エラー
    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!( {
            "username" : "test_admin",
            "password": "12345",
        }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    //usernameが存在しない場合のエラー
    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!( {
            "username" : "tes",
            "password": "12345",
        }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
