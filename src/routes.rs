use actix_files::NamedFile;
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_MESSAGE: &str = "This is a default message";

#[derive(Serialize, Deserialize)]
pub struct MessageInput {
    message: Option<String>,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct Message {
    #[validate(length(min = 1, max = 200))]
    message: String,
}

fn to_message(input: &MessageInput) -> Message {
    Message {
        message: String::from(
            input
                .message
                .as_ref()
                .unwrap_or(&String::from(DEFAULT_MESSAGE)),
        ),
    }
}

fn validate_message_input(input: &MessageInput) -> HttpResponse {
    let message = to_message(&input);

    message.validate().map_or(
        HttpResponse::BadRequest().json(Message {
            message: "Bad Request".into(),
        }),
        |_| HttpResponse::Ok().json(&message),
    )
}

#[get("/echo")]
pub async fn get_echo(query: web::Query<MessageInput>) -> HttpResponse {
    validate_message_input(&query.into_inner())
}

#[post("/echo")]
pub async fn post_echo(input: web::Json<MessageInput>) -> HttpResponse {
    validate_message_input(&input.into_inner())
}

#[get("/openapi")]
pub async fn openapi() -> std::io::Result<NamedFile> {
    Ok(NamedFile::open_async("data/openapi.yaml")
        .await?
        .use_etag(true))
}

#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::dev::Service;
    use actix_web::{http, test, App, Error};

    use super::*;

    #[actix_web::test]
    async fn to_message_handles_empty_string() -> Result<(), Error> {
        let input = MessageInput { message: None };
        let message = to_message(&input);
        assert_eq!(message.message, DEFAULT_MESSAGE);
        Ok(())
    }

    #[actix_web::test]
    async fn to_message_handles_string() -> Result<(), Error> {
        let input = MessageInput {
            message: Some(String::from("test message")),
        };
        let message = to_message(&input);
        assert_eq!(message.message, "test message");
        Ok(())
    }

    #[actix_web::test]
    async fn test_get_echo() -> Result<(), Error> {
        let app = App::new().service(get_echo);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/echo").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        let expected = serde_json::to_string(&Message {
            message: String::from(DEFAULT_MESSAGE),
        })?;

        assert_eq!(to_bytes(response_body).await?, expected);

        Ok(())
    }

    #[actix_web::test]
    async fn test_get_echo_with_message() -> Result<(), Error> {
        let app = App::new().service(get_echo);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get()
            .uri("/echo?message=test")
            .to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        let expected = serde_json::to_string(&Message {
            message: "test".into(),
        })?;

        assert_eq!(to_bytes(response_body).await?, expected);

        Ok(())
    }

    #[actix_web::test]
    async fn test_get_echo_with_super_long_message() -> Result<(), Error> {
        let app = App::new().service(get_echo);
        let app = test::init_service(app).await;

        let message: String = "test".repeat(100).into();
        let req = test::TestRequest::get()
            .uri(&format!("/echo?message={}", &message))
            .to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[actix_web::test]
    async fn test_post_echo_without_message() -> Result<(), Error> {
        let app = App::new().service(post_echo);
        let app = test::init_service(app).await;

        let payload = MessageInput { message: None };

        let req = test::TestRequest::post()
            .uri("/echo")
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(&payload)
            .to_request();

        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();

        let expected_payload_str = serde_json::to_string(&Message {
            message: String::from(DEFAULT_MESSAGE),
        })?;
        assert_eq!(to_bytes(response_body).await?, &expected_payload_str);

        Ok(())
    }

    #[actix_web::test]
    async fn test_post_echo_with_message() -> Result<(), Error> {
        let app = App::new().service(post_echo);
        let app = test::init_service(app).await;

        let payload = MessageInput {
            message: Some("test".into()),
        };

        let req = test::TestRequest::post()
            .uri("/echo")
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(&payload)
            .to_request();

        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        let payload_str = serde_json::to_string(&payload)?;

        assert_eq!(to_bytes(response_body).await?, &payload_str);

        Ok(())
    }

    #[actix_web::test]
    async fn test_post_echo_message_too_large() -> Result<(), Error> {
        let app = App::new().service(post_echo);
        let app = test::init_service(app).await;

        let payload = MessageInput {
            message: Some("test message".repeat(100).into()),
        };

        let req = test::TestRequest::post()
            .uri("/echo")
            .insert_header((http::header::CONTENT_TYPE, "application/json"))
            .set_json(&payload)
            .to_request();

        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);

        Ok(())
    }

    #[actix_web::test]
    async fn test_gets_openapi() -> Result<(), Error> {
        let app = App::new().service(openapi);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/openapi").to_request();

        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}
