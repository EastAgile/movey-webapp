use anyhow::Result;
use httpmock::prelude::*;
use serde_json;
use tera::Context;
use tera::Tera;
use std::sync::{Arc, RwLock};
use log::debug;



#[cfg(test)]
mod send_via_sendgrid_should {
    use super::*;
    use jelly::email::sendgrid::Email;
    use test_log::test; // Automatically log tests

    #[test]
    fn send_expected_json() -> Result<()> {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let mut templates = Tera::default();
        templates.add_raw_template("t.html", "test {{ name }}")?;
        templates.add_raw_template("t.txt", "test {{ name }}")?;

        std::env::set_var("SENDGRID_API_KEY", "sapikey");
        std::env::set_var("POSTMARK_MESSAGE_STREAM", "default");
        std::env::set_var("EMAIL_DEFAULT_FROM", "owner@example.com");

        // Create a mock on the server.
        let server_mock = server.mock(|expect, resp_with| {
            expect
                .method(POST)
                .header("Authorization", "Bearer: sapikey")
                .path("/v3/mail/send")
                .json_body(serde_json::json!({
                "personalizations":[{"to":[{"email":"a@exemple.com,b@example.com"}]}],
                "from":{"email":"owner@example.com"},
                "subject":"subject line",
                "content":[
                    {"type":"text/plain","value":"test surname name"},
                    {"type":"text/html","value":"test surname name"}]
                }
                ));
            resp_with
                .status(200)
                .header("content-type", "text/html")
                .body("ok");
        });

        // Send an HTTP request to the mock server. This simulates your code.
        let mut context = Context::new();
        context.insert("name", "surname name");
        let email = Email::new(
            "t",
            &vec!["a@exemple.com".to_string(), "b@example.com".to_string()],
            "subject line",
            context,
            Arc::new(RwLock::new(templates)),
        )?;
        email.send_via_sendgrid(&server.url(""))?;

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        server_mock.assert();
        Ok(())
    }

    #[test]
    fn catch_api_error() -> Result<()> {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let mut templates = Tera::default();
        templates.add_raw_template("t.html", "test {{ name }}")?;
        templates.add_raw_template("t.txt", "test {{ name }}")?;

        std::env::set_var("SENDGRID_API_KEY", "sapikey");
        std::env::set_var("POSTMARK_MESSAGE_STREAM", "default");
        std::env::set_var("EMAIL_DEFAULT_FROM", "owner@example.com");

        // Create a mock on the server.
        let server_mock = server.mock(|expect, resp_with| {
            expect
                .method(POST)
                .header("Authorization", "Bearer: sapikey")
                .path("/v3/mail/send")
                .json_body(serde_json::json!({
                "personalizations":[{"to":[{"email":"a@exemple.com,b@example.com"}]}],
                "from":{"email":"owner@example.com"},
                "subject":"subject line",
                "content":[
                    {"type":"text/plain","value":"test surname name"},
                    {"type":"text/html","value":"test surname name"}]
                }
                ));
            resp_with
                .status(401)
                .header("content-type", "text/json")
                .body(r#"{"errors":[{"message":"Permission denied, wrong credentials","field":null,"help":null}]}"#);
        });

        // Send an HTTP request to the mock server. This simulates your code.
        let mut context = Context::new();
        context.insert("name", "surname name");
        let email = Email::new(
            "t",
            &vec!["a@exemple.com".to_string(), "b@example.com".to_string()],
            "subject line",
            context,
            Arc::new(RwLock::new(templates)),
        )?;
        let res = email.send_via_sendgrid(&server.url(""));

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        server_mock.assert();
        assert!(res.is_err());
        let resstr = format!("{:?}", res);
        debug!("{}", resstr);
        assert!(resstr.contains("Sending mail to a@exemple.com,b@example.com via sendgrid failed. API call returns code 401 : Unauthorized"));
        assert!(resstr.contains("Permission denied, wrong credentials"));
        Ok(())
    }
}

#[cfg(test)]
mod send_via_postmark_should {
    use super::*;
    use jelly::email::postmark::Email;

    #[test]
    fn send_expected_json() -> Result<()> {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let mut templates = Tera::default();
        templates.add_raw_template("t.html", "test {{ name }}")?;
        templates.add_raw_template("t.txt", "test {{ name }}")?;

        std::env::set_var("POSTMARK_API_KEY", "papikey");
        std::env::set_var("POSTMARK_MESSAGE_STREAM", "default");
        std::env::set_var("EMAIL_DEFAULT_FROM", "owner@example.com");

        // Create a mock on the server.
        let server_mock = server.mock(|expect, resp_with| {
            expect
                .method(POST)
                .header("X-Postmark-Server-Token", "papikey")
                .path("/email")
                .json_body(serde_json::json!({
                    "From":"owner@example.com",
                    "To":"a@exemple.com,b@example.com",
                    "Subject": "subject line",
                    "TextBody":"test surname name",
                    "HtmlBody":"test surname name",
                    "MessageStream":"default"} 
                ));
            resp_with
                .status(200)
                .header("content-type", "text/html")
                .body("ok");
        });

        // Send an HTTP request to the mock server. This simulates your code.
        let mut context = Context::new();
        context.insert("name", "surname name");
        let email = Email::new(
            "t",
            &vec!["a@exemple.com".to_string(), "b@example.com".to_string()],
            "subject line",
            context,
            Arc::new(RwLock::new(templates)),
        )?;
        email.send_via_postmark(&server.url(""))?;

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        server_mock.assert();
        Ok(())
    }
    #[test]
    fn catch_api_error() -> Result<()> {
        // Start a lightweight mock server.
        let server = MockServer::start();
        let mut templates = Tera::default();
        templates.add_raw_template("t.html", "test {{ name }}")?;
        templates.add_raw_template("t.txt", "test {{ name }}")?;

        std::env::set_var("POSTMARK_API_KEY", "papikey");
        std::env::set_var("POSTMARK_MESSAGE_STREAM", "default");
        std::env::set_var("EMAIL_DEFAULT_FROM", "owner@example.com");

        // Create a mock on the server.
        let server_mock = server.mock(|expect, resp_with| {
            expect
                .method(POST)
                .header("X-Postmark-Server-Token", "papikey")
                .path("/email")
                .json_body(serde_json::json!({
                    "From":"owner@example.com",
                    "To":"a@exemple.com,b@example.com",
                    "Subject": "subject line",
                    "TextBody":"test surname name",
                    "HtmlBody":"test surname name",
                    "MessageStream":"default"} 
                ));
            resp_with
                .status(401)
                .header("content-type", "text/json")
                .body(r#"{"errors":[{"message":"Permission denied, wrong credentials","field":null,"help":null}]}"#);
        });

        // Send an HTTP request to the mock server. This simulates your code.
        let mut context = Context::new();
        context.insert("name", "surname name");
        let email = Email::new(
            "t",
            &vec!["a@exemple.com".to_string(), "b@example.com".to_string()],
            "subject line",
            context,
            Arc::new(RwLock::new(templates)),
        )?;
        let res = email.send_via_postmark(&server.url(""));

        // Ensure the specified mock was called exactly one time (or fail with a detailed error description).
        server_mock.assert();
        assert!(res.is_err());
        let resstr = format!("{:?}", res);
        debug!("{}", resstr);
        assert!(resstr.contains("Sending mail to a@exemple.com,b@example.com via postmark failed. API call returns code 401 : Unauthorized"));
        assert!(resstr.contains("Permission denied, wrong credentials"));
        Ok(())
    }
}
