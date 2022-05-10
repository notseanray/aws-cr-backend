use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemError, Put};

use crate::{accounts::NewAccount, LoginEvent, response::ResponseError};

pub(crate) async fn insert_new_account(v: NewAccount) -> Result<(), RusotoError<PutItemError>> {
    // maybe swap to env deciding region?
    let client = DynamoDbClient::new(Region::UsEast1);

    client.put_item(v.into()).await?;
    Ok(())
}

pub(crate) enum DatabaseQuery {
    Login(LoginEvent),
}
