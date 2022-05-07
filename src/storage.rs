use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};

use crate::{accounts::NewAccount, LoginEvent};

pub(crate) async fn insert_new_account(v: NewAccount) {
    //let shared_config = aws_config::load_from_env().await;

    let client = DynamoDbClient::new(Region::UsEast1);
}

pub(crate) enum DatabaseQuery {
    Login(LoginEvent),
}
