use rocket::http::{ContentType, Header};
use rocket::local::{Client, LocalRequest};
use serde_json;

use super::*;

struct TestContext {
    token: String,
    client: Client,
}

impl TestContext {
    pub fn new() -> Self {
        let rocket = create_app();
        let client = Client::new(rocket).unwrap();
        let token = Self::create_user_and_login();

        Self { client, token }
    }

    fn create_user_and_login() -> String {
        let rocket = create_app();
        let client = Client::new(rocket).unwrap();
        let serialized_user_data = r#"{"username": "some_user","password": "some_password"}"#;

        let _ = client
            .post("/api/users")
            .header(ContentType::JSON)
            .body(serialized_user_data)
            .dispatch();

        let mut user_login_response = client
            .post("/api/users/login")
            .header(ContentType::JSON)
            .body(serialized_user_data)
            .dispatch();

        let user_login_response_body = user_login_response.body_string().unwrap();
        serde_json::from_str(&user_login_response_body).unwrap()
    }

    pub fn authorized_request(&self, method: Method, url: String) -> LocalRequest {
        self.client
            .req(method, url)
            .header(Header::new("Authorization", format!("Bearer {}", self.token)))
    }

    fn clear_database(&mut self) {
        use schema::posts::dsl::*;
        use schema::users::dsl::*;
        use diesel::prelude::*;

        let conn = Database::get_one(self.client.rocket()).unwrap();
        diesel::delete(users).execute(&*conn).unwrap();
        diesel::delete(posts).execute(&*conn).unwrap();
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        self.clear_database();
    }
}

#[test]
fn test_get_user_post() {
    let context = TestContext::new();

    let response = context
        .authorized_request(Method::Get, "/api/posts".to_string())
        .dispatch();

    println!("{:?}", response);
    assert_eq!(response.status().code, 200)
}