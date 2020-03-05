use std::collections::HashMap;

use rocket::http::{ContentType, Header, StatusClass};
use rocket::local::{Client, LocalRequest};

use crate::user::User;

use super::*;

struct TestContext {
    client: Client,
    token: Option<String>,
    user: Option<User>,
}

impl TestContext {
    pub fn new() -> Self {
        let rocket = create_app();
        let client = Client::new(rocket).unwrap();

        Self {
            client,
            token: None,
            user: None,
        }
    }

    fn create_user_and_get_token(&mut self) -> String {
        use diesel::prelude::*;
        use schema::users;

        let conn = Database::get_one(self.client.rocket()).unwrap();

        // costile: i don't actually now how to test with database, so, as for integration testing
        // we're going to contain single test user in database, and not delete it after tear down
        // test context, and believe that no one changed it's password
        // (and how to test password changing then?)
        let username = &"NeVeR_T0_Uze_User_Nаme_Really_Long_And_HARD".to_string();
        let password = &"some_password".to_string();
        let user_data = user::UserData {
            username: username.to_string(),
            password: password.to_string(),
        };

        let user: User = users::table
            .find(&user_data.username)
            .first::<User>(&*conn)
            .unwrap_or(user::User::from(user_data));

        diesel::insert_into(users::table)
            .values(&user)
            .execute(&*conn)
            .unwrap_or_default();
        let token = user
            .verify_password_and_generate_jwt(password.to_string())
            .unwrap();
        self.user = Some(user);
        token
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
}

impl Drop for TestContext {
    fn drop(&mut self) {
        use schema::posts::dsl::*;
        //        use schema::users::dsl::*;
        use diesel::prelude::*;

        let conn = Database::get_one(self.client.rocket()).unwrap();

        let user = self.user.as_mut().unwrap();

        diesel::delete(posts)
            .filter(author.eq(user.username()))
            .execute(&*conn)
            .unwrap();
        //        diesel::delete(users).filter(username.eq(user.username()))
        //            .execute(&*conn)
        //            .unwrap();
    }
}

#[test]
fn test_get_user_post() {
    let mut context = TestContext::new();

    let response = context
        .authorized_request(Method::Get, "/posts".to_string())
        .dispatch();

    println!("{:?}", response);
    assert_eq!(200, response.status().code)
}

#[test]
fn test_create_post() {
    let mut context = TestContext::new();

    let post_serialized = r#"{"title": "test", "body": "test"}"#;
    let response = context
        .authorized_request(Method::Post, "/posts".to_string())
        .body(post_serialized)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(200, response.status().code)
}

#[test]
fn test_create_post_with_invalid_values() {
    let mut context = TestContext::new();
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
        let response = context
            .authorized_request(Method::Post, "/posts".into())
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
fn text_update_post() {}
