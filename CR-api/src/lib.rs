mod accounts;
mod payment;
mod response;
mod storage;
mod admin;

use accounts::*;
use payment::*;
use response::*;
use storage::*;
use admin::*;

#[cfg(test)]
mod tests;

use crate::{CreateAccountEvent, PaymentEvent, ResponseError};
use lambda_runtime::{Error as LambdaError, LambdaEvent};
use lazy_static::lazy_static;
use log::error;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

lazy_static! {
    static ref ENCRYPTION_KEY: String = env_or_exit!("ENCRYPTION_KEY");
}

/// Load the specified environment variable or substitute it for a default if it's not present
#[macro_export]
macro_rules! env_or_default {
    ($var:expr, $default:expr) => {
        match env::var($var).ok() {
            Some(v) => v.parse().unwrap_or($default),
            None => $default,
        }
    };
}

/// Load the specified environment variable or exit if it is not present
#[macro_export]
macro_rules! env_or_exit {
    ($var:expr) => {
        match env::var($var).ok() {
            Some(v) => v,
            None => {
                error!("please fill out {} in env", $var);
                std::process::exit(1);
            }
        }
    };
}

#[derive(Serialize)]
pub(crate) struct BackendResponse {
    pub response: BackendResponseContents,
}

#[derive(Serialize)]
#[serde(untagged)]
pub(crate) enum BackendResponseContents {
    ResponseCode(usize),
    ResponseMessage(String),
}

#[derive(Serialize)]
pub(crate) struct UpdateUserRequest {
    pub token: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>
}

#[derive(Deserialize)]
pub(crate) struct LoginEvent {
    pub email: String,
    pub password: String,
    pub grad_year: u16,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Ping {
    pub ping: u64,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum EventTypes {
    CreateAccount(CreateAccountEvent),
    Payment(PaymentEvent),
    LoginRequest(LoginEvent),
    PingRequest(Ping),
}

#[derive(Deserialize)]
pub(crate) struct Data {
    pub payload: String,
}

pub async fn dispatch_event(e: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let (event, _context) = e.into_parts();

    let request: Data = match serde_json::from_value(event) {
        Ok(v) => v,
        Err(_) => return Err(Box::new(ResponseError::MalformedRequest)),
    };

    let decrypt = new_magic_crypt!(&*ENCRYPTION_KEY, 256);

    let data = match decrypt.decrypt_base64_to_string(request.payload) {
        Ok(v) => v,
        Err(_) => return Err(Box::new(ResponseError::InvalidDecryptionResult)),
    };

    let backend_event: EventTypes = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => return Err(Box::new(ResponseError::InvalidRequest)),
    };

    type R = EventTypes;
    Ok(json!(match backend_event {
        R::CreateAccount(v) => handle_create_account(v).await?,
        R::Payment(v) => handle_payment(v).await?,
        R::LoginRequest(v) => handle_login_request(v).await?,
        R::PingRequest(v) => handle_ping(v).await?,
    }))
}

pub(crate) async fn handle_create_account(
    e: CreateAccountEvent,
) -> Result<BackendResponse, ResponseError> {
    CreateAccountEvent::validate_account(e)?;
    Ok(BackendResponse {
        response: BackendResponseContents::ResponseCode(200),
    })
}

pub(crate) async fn handle_ping(e: Ping) -> Result<BackendResponse, ResponseError> {
    Ok(BackendResponse {
        response: BackendResponseContents::ResponseMessage(
            serde_json::to_string_pretty(&e).unwrap(),
        ),
    })
}
// TODO
// Paypal webtook
pub(crate) async fn handle_payment(e: PaymentEvent) -> Result<BackendResponse, ResponseError> {
    Ok(BackendResponse {
        response: BackendResponseContents::ResponseCode(200),
    })
}

pub(crate) async fn handle_login_request(e: LoginEvent) -> Result<BackendResponse, ResponseError> {
    Ok(BackendResponse {
        response: match validate_login(e).await {
            Ok((true, Some(v))) => BackendResponseContents::ResponseMessage(v),
            Err(e) => BackendResponseContents::ResponseMessage(e.to_string()),
            _ => BackendResponseContents::ResponseCode(400),
        },
    })
}
