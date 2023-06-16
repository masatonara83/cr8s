use reqwest::StatusCode;
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_rustaceans() {
    //準備
    let client = common::get_client_with_logged_in_admin();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);
    //実行
    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();
    //検証
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    //テストで追加した値を削除
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let response = client
        .post(format!("{}/rustaceans", common::APP_HOST))
        .json(&json!( {
            "name": "Foo bar",
            "email" : "foo@bar.com"
        }
        ))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!( {
            "id": rustacean["id"],
            "name": "Foo bar",
            "email" : "foo@bar.com",
            "created_at" : rustacean["created_at"]
        }
        )
    );

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans() {
    let client = common::get_client_with_logged_in_admin();

    let rustacean = common::create_test_rustacean(&client);
    let response = client
        .get(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!( {
            "id": rustacean["id"],
            "name": "Foo bar",
            "email" : "foo@bar.com",
            "created_at" : rustacean["created_at"]
        }
        )
    );

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_put_rustaceans() {
    let client = common::get_client_with_logged_in_admin();

    let rustacean = common::create_test_rustacean(&client);
    let response = client
        .put(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .json(&json!( {
            "name": "Foo123 bar",
            "email" : "foo123@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!( {
            "id": rustacean["id"],
            "name": "Foo123 bar",
            "email" : "foo123@bar.com",
            "created_at" : rustacean["created_at"]
        }
        )
    );

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
