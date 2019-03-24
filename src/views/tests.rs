use std::collections::HashMap;

use rocket::http::{ContentType, Header, Status, StatusClass};
use rocket::local::Client;

use crate::create_app;
use crate::models::Id;
use crate::models::post::{NewPost, Post};
use crate::models::user::NewUser;

fn create_client() -> Client {
    let rocket = create_app();
    Client::new(rocket).unwrap()
}

fn create_post() -> (Id, NewPost) {
    let client = create_client();
    let token = create_user();
    let post = NewPost {
        title: "Test".to_string(),
        body: "Body".to_string(),
    };
    let post_serialized = serde_json::to_string(&post).unwrap();
    let req = client
        .post("/api/posts")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .header(ContentType::JSON)
        .body(post_serialized);
    let mut response = req.dispatch();
    let id: Id = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    (id, post)
}

fn create_user() -> String {
    let client = create_client();

    let user_form = NewUser {
        username: "test".to_string(),
        password: "test".to_string(),
        user_roles: vec![],
    };
    let user_serialized = serde_json::to_string(&user_form).unwrap();
    let req = client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(user_serialized)
        .dispatch();

    let mut response = client
        .post("/api/users/token")
        .header(ContentType::JSON)
        .body(r#"{"username": "test", "password": "test"}"#)
        .dispatch();

    serde_json::from_str(response.body_string().unwrap().as_ref()).unwrap()
}

#[test]
#[ignore]
fn test_create_post() {
    let client = create_client();
    let token = create_user();

    let post = NewPost {
        title: "Test title".to_string(),
        body: "Test body".to_string(),
    };
    let serialized = serde_json::to_string(&post).unwrap();
    let req = client
        .post("/api/posts")
        .header(ContentType::JSON)
        .header(Header::new("Authorization", format!("Bearer {}", token)))
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
#[ignore]
fn test_create_post_with_invalid_data() {
    let client = create_client();
    let token = create_user();

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
            .header(Header::new("Authorization", format!("Bearer {}", token)))
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
#[ignore]
fn test_get_post() {
    let (id, post) = create_post();
    let client = create_client();

    let mut response = client.get(format!("/api/posts/{}", id)).dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(ContentType::JSON, response.content_type().unwrap());

    let data: Post = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    assert_eq!(id, data.id);
    assert_eq!(post.body, data.body);
    assert_eq!(post.title, data.title);
}

#[test]
#[ignore]
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
        let response = client.get(format!("/api/posts/{}", data)).dispatch();
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
#[ignore]
fn test_update_post() {
    let (id, _) = create_post();
    let token = create_user();
    let client = create_client();
    let body = r#"{"title":"Updated title", "body": "Updated body"}"#;
    let response = client
        .put(format!("/api/posts/{}", id))
        .body(body)
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(ContentType::JSON, response.content_type().unwrap());

    let mut updated_post_response = client.get(format!("/api/posts/{}", id)).dispatch();

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
#[ignore]
fn test_update_post_with_invalid_data() {
    let (id, _) = create_post();
    let client = create_client();
    let token = create_user();

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
            .put(format!("/api/posts/{}", id))
            .body(data)
            .header(Header::new("Authorization", format!("Bearer {}", token)))
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
#[ignore]
fn test_delete_post() {
    let (id, _) = create_post();
    let client = create_client();
    let token = create_user();
    let mut response = client
        .delete(format!("/api/posts/{}", id))
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(
        ContentType::JSON,
        response
            .content_type()
            .expect("Couldn't read content type header")
    );

    response = client.get(format!("/api/posts/{}", id)).dispatch();

    assert_eq!(Status::NotFound, response.status());
    assert_eq!(
        ContentType::JSON,
        response
            .content_type()
            .expect("Couldn't read content type header")
    );
}


// TODO: Rebuild all tests for using same context, and clear database after every test