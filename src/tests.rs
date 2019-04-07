use rocket::http::{ContentType, Header};
use rocket::local::{Client, LocalRequest};

use super::*;

struct TestContext {
    client: Client,
    token: Option<String>,
}

impl TestContext {
    pub fn new() -> Self {
        let rocket = create_app();
        let client = Client::new(rocket).unwrap();

        Self { client, token: None }
    }

    fn create_user_and_get_token(&mut self) -> String {
        self.clear_database();
        use schema::users;
        use diesel::prelude::*;

        let conn = Database::get_one(self.client.rocket()).unwrap();

        let username = &"some_user".to_string();
        let password = &"some_password".to_string();
        let user_data = user::UserData { username: username.to_string(), password: password.to_string() };
        let user = user::User::from(user_data);

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&*conn)
            .unwrap_or_default();

        user.verify_password_and_generate_jwt(password.to_string()).unwrap()
    }

    pub fn authorized_request(&mut self, method: Method, url: String) -> LocalRequest {
        if self.token == None {
            self.token = Some(self.create_user_and_get_token())
        }
        let token = self.token.as_ref().unwrap().to_string();
        self.client
            .req(method, url)
            .header(Header::new("Authorization", format!("Bearer {}", token)))
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
    let mut context = TestContext::new();

    let response = context
        .authorized_request(Method::Get, "/api/posts".to_string())
        .dispatch();

    println!("{:?}", response);
    assert_eq!(200, response.status().code)
}

#[test]
fn test_create_post() {
    let mut context = TestContext::new();

    let post_serialized = r#"{"title": "test", "body": "test"}"#;
    let response = context
        .authorized_request(Method::Post, "/api/posts".to_string())
        .body(post_serialized)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(200, response.status().code)
}