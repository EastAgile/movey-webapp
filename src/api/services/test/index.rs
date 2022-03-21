use jelly::actix_web::{web, HttpRequest};
use jelly::prelude::*;
use jelly::Result;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct TestResponse {
    name: String,
    size: i32
}

pub async fn get_object(request: HttpRequest) -> Result<HttpResponse> {
    let json = TestResponse {
        name: "Test".into(),
        size: 5
    };

    request.json(200, &json)
}

pub async fn get_json(request: HttpRequest) -> Result<HttpResponse> {
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    request.json(200, &john)
}

pub async fn post_json(request: HttpRequest, res: web::Json<TestResponse>) -> Result<HttpResponse> {
    let json = TestResponse {
        name: res.name.clone(),
        size: res.size
    };

    request.json(200, &json)
}
