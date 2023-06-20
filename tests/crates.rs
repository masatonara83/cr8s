use reqwest::StatusCode;
use serde_json::{json, Value};
pub mod common;

#[test]
fn test_get_crates() {
    //準備
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);

    //実行
    let client = common::get_client_with_logged_in_viewer();
    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();
    //検証
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    //テストで追加した値を削除
    let client = common::get_client_with_logged_in_admin();
    // common::delete_test_crate(&client, a_crate);
    // common::delete_test_crate(&client, b_crate);
    // common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_crate() {
    //準備
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    //実行
    let response = client
        .post(format!("{}/crates", common::APP_HOST))
        .json(&json!( {
            "rustacean_id": rustacean["id"],
            "code" : "foo",
            "name": "Foo bar crate",
            "version": "0.1",
            "description" : "Foo crate description"
        }
        ))
        .send()
        .unwrap();
    //検証
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!( {
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code" : "foo",
            "name": "Foo bar crate",
            "version": "0.1",
            "description" : "Foo crate description",
            "created_at" : a_crate["created_at"]
        }
        )
    );

    //後片付け
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate: Value = common::create_test_crate(&client, &rustacean);

    let client = common::get_client_with_logged_in_viewer();
    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    assert_eq!(
        a_crate,
        json!( {
            "id": a_crate["id"],
            "rustacean_id": rustacean["id"],
            "code" : "foo",
            "name": "Foo bar crate",
            "version": "0.1",
            "description" : "Foo crate description",
            "created_at" : a_crate["created_at"]
        }
        )
    );

    let client = common::get_client_with_logged_in_admin();
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate: Value = common::create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!( {
            "rustacean_id": rustacean["id"],
            "code" : "fooz2",
            "name": "Fooz2 bar crate",
            "version": "0.2",
            "description" : "Fooz2 crate description"
        }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!( {
            "rustacean_id": rustacean["id"],
            "code" : "fooz2",
            "name": "Fooz2 bar crate",
            "version": "0.2",
            "description" : "Fooz2 crate description"
        }
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate: Value = common::create_test_crate(&client, &rustacean);

    let response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    common::delete_test_rustacean(&client, rustacean);
}
