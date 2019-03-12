use std::collections::HashMap;

use rocket::http::{ContentType, Status, StatusClass};
use rocket::local::Client;

use crate::create_app;
use crate::models::Id;
use crate::models::post::{NewPost, Post};

fn create_client() -> Client {
    let rocket = create_app();
    Client::new(rocket).unwrap()
}

fn create_post() -> (Id, NewPost) {
    let client = create_client();

    let post = NewPost {
        title: "Test".to_string(),
        body: "Body".to_string(),
    };
    let post_serialized = serde_json::to_string(&post).unwrap();
    let req = client
        .post("/posts")
        .header(ContentType::JSON)
        .body(post_serialized);
    let mut response = req.dispatch();
    let id: Id = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    (id, post)
}

#[test]
fn test_create_post() {
    let client = create_client();

    let post = NewPost {
        title: "Test title".to_string(),
        body: "Test body".to_string(),
    };
    let serialized = serde_json::to_string(&post).unwrap();
    let req = client
        .post("/posts")
        .header(ContentType::JSON)
        .body(serialized);
    let mut response = req.dispatch();

    assert_eq!(Status::Ok, response.status());

    assert_eq!(
        ContentType::JSON,
        response
            .content_type()
            .expect("Could not read content type header")
    );

    let _: Id = serde_json::from_str(
        &response
            .body_string()
            .expect("Could not read the string body from response"),
    )
        .expect("Could not parse Id object from body");
}

#[test]
fn test_create_post_with_invalid_data() {
    let client = create_client();
    let invalid_data: HashMap<&str, &str> = [
        ("Empty data", ""),
        ("Invalid keys", r#"{"foo": "bar", "baz": "qux"}"#),
        ("Not an object", r#""hello""#),
        ("Empty values", r#"{"title": "", "body": ""}"#),
        ("Spaces", r#"{"title": " ", "body": " "}"#),
        ("Numbers", r#"{"title": 1, "body": 2}"#),
    ]
        .iter()
        .cloned()
        .collect();

    for (description, data) in invalid_data {
        let response = client
            .post(format!("/posts"))
            .body(data)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(
            StatusClass::ClientError,
            response.status().class(),
            "Assertion failed on {}",
            description
        );
        assert_eq!(
            ContentType::JSON,
            response
                .content_type()
                .expect("Response should contain content type header")
        )
    }
}

#[test]
fn test_get_post() {
    let (id, post) = create_post();
    let client = create_client();

    let mut response = client.get(format!("/posts/{}", id)).dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(ContentType::JSON, response.content_type().unwrap());

    let data: Post = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    assert_eq!(id, data.id);
    assert_eq!(post.body, data.body);
    assert_eq!(post.title, data.title);
}

#[test]
fn test_get_post_with_invalid_id() {
    let invalid_data: HashMap<&str, &str> = [
        ("Invalid number", "999999"),
        ("Char id", "hello"),
        ("Negative id", "-1"),
    ]
        .iter()
        .cloned()
        .collect();

    let client = create_client();

    for (description, data) in invalid_data {
        let response = client.get(format!("/posts/{}", data)).dispatch();
        assert_eq!(
            Status::NotFound,
            response.status(),
            "Assertion failed on {}",
            description
        );
        assert_eq!(
            ContentType::JSON,
            response
                .content_type()
                .expect("Response should contain content type header")
        )
    }
}

#[test]
fn test_update_post() {
    let (id, _) = create_post();
    let client = create_client();
    let body = r#"{"title":"Updated title", "body": "Updated body"}"#;
    let response = client
        .put(format!("/posts/{}", id))
        .body(body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(ContentType::JSON, response.content_type().unwrap());

    let mut updated_post_response = client.get(format!("/posts/{}", id)).dispatch();

    assert_eq!(Status::Ok, updated_post_response.status());
    assert_eq!(
        ContentType::JSON,
        updated_post_response.content_type().unwrap()
    );
    let updated_post: Post = serde_json::from_str(
        &updated_post_response
            .body_string()
            .expect("Could not read the string body from response"),
    )
        .expect("Could not parse Id object from body");

    assert_eq!(id, updated_post.id);
    assert_eq!("Updated body", updated_post.body);
    assert_eq!("Updated title", updated_post.title);
}

#[test]
fn test_update_post_with_invalid_data() {
    let (id, _) = create_post();
    let client = create_client();

    let invalid_data: HashMap<&str, &str> = [
        ("Empty data", ""),
        ("Invalid keys", r#"{"foo": "bar", "baz": "qux"}"#),
        ("Not an object", r#""hello""#),
        ("Empty values", r#"{"title": "", "body": ""}"#),
        ("Spaces", r#"{"title": " ", "body": " "}"#),
        ("Numbers", r#"{"title": 1, "body": 2}"#),
    ]
        .iter()
        .cloned()
        .collect();

    for (description, data) in invalid_data {
        let response = client
            .put(format!("/posts/{}", id))
            .body(data)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(
            StatusClass::ClientError,
            response.status().class(),
            "Assertion failed on {}",
            description
        );
        assert_eq!(
            ContentType::JSON,
            response
                .content_type()
                .expect("Response should contain content type header"),
            "Assertion failed on {}",
            description
        )
    }
}

#[test]
fn test_delete_post() {
    let (id, _) = create_post();
    let client = create_client();
    let mut response = client.delete(format!("/posts/{}", id)).dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(ContentType::JSON, response.content_type().expect("Couldn't read content type header"));

    response = client.get(format!("/posts/{}", id)).dispatch();

    assert_eq!(Status::NotFound, response.status());
    assert_eq!(ContentType::JSON, response.content_type().expect("Couldn't read content type header"));
}
