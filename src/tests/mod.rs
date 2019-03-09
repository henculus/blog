use crate::create_app;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

pub fn create_post() {
    let rocket = create_app();
    let client = Client::new(rocket).expect("Valid rocket instance");
    let req = client
        .post("/posts")
        .header(ContentType::JSON)
        .body(r#"{"title": "Hello world", "body": "Test Body}"#);
    let response = req.dispatch();
}

#[test]
fn test_get_post() {
    create_post();
    let rocket = create_app();
    let client = Client::new(rocket).expect("Valid rocket instance");
    let req = client.get("/posts/1");
    let response = req.dispatch();
    assert_eq!(Status::Ok, response.status());
}