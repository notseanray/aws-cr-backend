use crate::{CreateAccountEvent, PaymentEvent, ResponseError};
use lambda_runtime::{Error as LambdaError, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod accounts;
mod payment;
mod response;
mod storage;

use accounts::*;
use payment::*;
use response::*;
use storage::*;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
pub(crate) struct BackendResponse {
    pub response: BackendResponseContents,
}

// hash first.password.creation timestamp

#[derive(Serialize)]
pub(crate) enum BackendResponseContents {
    ResponseCode(usize),
    ResponseMessage(String), // json struct response
    ErrorMessage(String),
}

#[derive(Deserialize)]
pub(crate) struct LoginEvent {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub(crate) enum EventTypes {
    CreateAccount(CreateAccountEvent),
    Payment(PaymentEvent),
    LoginRequest(LoginEvent),
}

pub async fn dispatch_event(e: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let (event, _context) = e.into_parts();
    let backend_event: EventTypes = match serde_json::from_value(event) {
        Ok(v) => v,
        Err(_) => return Err(Box::new(ResponseError::InvalidRequest)),
    };
    type R = EventTypes;
    Ok(json!(match backend_event {
        R::CreateAccount(v) => handle_create_account(v).await?,
        R::Payment(v) => handle_payment(v).await?,
        R::LoginRequest(v) => handle_login_request(v).await?,
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

pub(crate) async fn handle_payment(e: PaymentEvent) -> Result<BackendResponse, ResponseError> {
    Ok(BackendResponse {
        response: BackendResponseContents::ResponseCode(200),
    })
}

pub(crate) async fn handle_login_request(e: LoginEvent) -> Result<BackendResponse, ResponseError> {
    Ok(BackendResponse {
        response: BackendResponseContents::ResponseCode(200),
    })
}
