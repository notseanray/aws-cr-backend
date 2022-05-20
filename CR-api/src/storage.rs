use std::collections::HashMap;

use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient, GetItemInput, PutItemError, GetItemError, AttributeValue, UpdateItemInput
};

use crate::{
    accounts::{NewAccount, MAX_USERNAME_LENGTH},
    response::ResponseError,
    timestamp, LoginEvent, insert_string,
};
use std::time::{SystemTime, UNIX_EPOCH};

macro_rules! find_region {
    () => {
        // TODO
        // need to test this
        // also regions under construction for complete-ness?
        // might have to convert to function and disable inlining if bin size is a issue
        // <https://awsregion.info/>
        match option_env!("AWS_REGION").unwrap_or_default() {
            "us-east-1" => Region::UsEast1,
            "us-east-2" => Region::UsEast2,
            "us-west-1" => Region::UsWest1,
            "us-west-2" => Region::UsWest2,
            "ca-central-1" => Region::CaCentral1,
            "eu-north-1" => Region::EuNorth1,
            "eu-west-3" => Region::EuWest3,
            "eu-west-2" => Region::EuWest2,
            "eu-west-1" => Region::EuWest1,
            "eu-central-1" => Region::EuCentral1,
            "ap-south-1" => Region::ApSouth1,
            "ap-northeast-1" => Region::ApNortheast1,
            "ap-northeast-2" => Region::ApNortheast2,
            "ap-northeast-3" => Region::ApNortheast3,
            "sa-east-1" => Region::SaEast1,
            "cn-northwest-1" => Region::CnNorthwest1,
            "us-gov-east-1" => Region::UsGovEast1,
            "us-gov-west-1" => Region::UsGovWest1,
            "me-south-1" => Region::MeSouth1,
            "af-south-1" => Region::AfSouth1,
            _ => Region::UsEast1
        }
    }
}

type DBGetErr<T> = Result<T, RusotoError<GetItemError>>;
type DBPutErr<T> = Result<T, RusotoError<PutItemError>>;

pub(crate) async fn insert_new_account(v: NewAccount) -> DBPutErr<()> {
    // maybe swap to env deciding region?
    let client = DynamoDbClient::new(find_region!());

    client.put_item(v.into()).await?;
    Ok(())
}

impl From<LoginEvent> for GetItemInput {
    fn from(l: LoginEvent) -> GetItemInput {
        let mut items = HashMap::new();
        // TODO
        GetItemInput {
            key: items,
            projection_expression: Some(String::from("token")),
            table_name: String::from("userauth"),
            ..GetItemInput::default()
        }
    }
}

pub(crate) async fn validate_login(l: LoginEvent) -> Result<(bool, Option<String>), ResponseError> {
    // current year + 4 (and ceil to account for creating some accounts early)
    let year_max = (timestamp!() as f32 / 31557600.0 + 4.0).ceil() as u32;
    if l.email.is_empty()
        || l.password.is_empty()
        || !(2002..year_max).contains(&(l.grad_year as u32))
    {
        return Err(ResponseError::InvalidCredentials);
    }

    let client = DynamoDbClient::new(find_region!());

    let items = match client.get_item(l.into()).await {
        Ok(v) => v.item,
        Err(_) => return Err(ResponseError::InvalidCredentials),
    };

    /*
     * generate new token and update db + send to client
    Ok(client.update_item(UpdateItemInput {

    }))
    */

    Ok(match items {
        Some(v) => {
            if !v.is_empty() && v.len() == 1 {
                (
                    true,
                    Some({
                        let token = match v.get("token") {
                            Some(v) => v,
                            None => return Err(ResponseError::InvalidCredentials),
                        };
                        token.s.clone().unwrap_or_default()
                    }),
                )
            } else {
                (false, None)
            }
        }
        None => (false, None),
    })
}

macro_rules! unwrap_db_result {
    ($val:expr) => {
        match $val {
            Ok(v) => match v.item {
                Some(v) => {
                    if !v.is_empty() && v.len() == 1 {
                        Some(v)
                    } else {
                        None
                    }
                }
                None => None,
            }
            Err(_) => None
        }
    }
}

pub(crate) async fn token_from_username(username: &str) -> Result<String, ResponseError> {
    if username.is_empty() || username.len() > *MAX_USERNAME_LENGTH {
        return Err(ResponseError::InvalidUsername)
    }
    let mut items = HashMap::with_capacity(1);
    insert_string!(items, "username", username);
    let query = GetItemInput {
        key: items,
        projection_expression: Some(String::from("token")),
        table_name: String::from("usertable"),
        ..GetItemInput::default()
    };
    let client = DynamoDbClient::new(find_region!());
    // this is really janky should fix soon
    let items = match unwrap_db_result!(client.get_item(query).await) {
        Some(v) => v,
        None => return Err(ResponseError::InvalidUsername)
    };
    let token = match items.get("token") {
        Some(v) => v,
        None => return Err(ResponseError::InvalidCredentials)
    };
    match &token.s {
        Some(v) => Ok(v.clone()),
        None => Err(ResponseError::InvalidCredentials)
    } 
} 

pub(crate) async fn username_from_token(token: &str) -> Result<String, ResponseError> {
    let mut items = HashMap::with_capacity(1);
    insert_string!(items, "token", token);
    let query = GetItemInput {
        key: items,
        projection_expression: Some(String::from("username")),
        table_name: String::from("tokentable"),
        ..GetItemInput::default()
    };
    let client = DynamoDbClient::new(find_region!());
    // this is really janky should fix soon
    let items = match unwrap_db_result!(client.get_item(query).await) {
        Some(v) => v,
        None => return Err(ResponseError::InvalidToken)
    };
    let username = match items.get("username") {
        Some(v) => v,
        None => return Err(ResponseError::InvalidCredentials)
    };
    match &username.s {
        Some(v) => Ok(v.clone()),
        None => Err(ResponseError::InvalidCredentials)
    } 
} 


macro_rules! get_string_or_return {
    ($map:expr, $val:expr) => {
        match $map.get($val) {
            Some(v) => v.s.clone().unwrap_or_default(),
            None => return Err(ResponseError::InvalidCredentials)
        }
    };
}

macro_rules! get_or_return {
    ($map:expr, $val:expr) => {
        match $map.get($val) {
            Some(v) => v,
            None => return Err(ResponseError::InvalidCredentials)
        }
    };
}

pub(crate) async fn is_admin(token: &str) -> bool {
    let username = match username_from_token(token).await {
        Ok(v) => v,
        Err(_) => return false
    };
    let user = match userdata_from_username(&username).await {
        Ok(v) => v,
        Err(_) => return false
    };
    user.admin
}

pub(crate) async fn userdata_from_username(username: &str) -> Result<NewAccount, ResponseError> {
    let mut items = HashMap::with_capacity(1);
    insert_string!(items, "username", username);
    let query = GetItemInput {
        key: items,
        projection_expression: Some(String::from("display_name, username, password, graduation_year, team, email, creation_timestamp, last_login, admin, staff, registered")),
        table_name: String::from("userauth"),
        ..GetItemInput::default()
    };
    let client = DynamoDbClient::new(find_region!());
    let items = match unwrap_db_result!(client.get_item(query).await) {
        Some(v) => v,
        None => return Err(ResponseError::InvalidToken)
    };

    // this is awful, but the dynamo api makes it difficult to deal with
    Ok(NewAccount {
        display_name: get_string_or_return!(items, "display_name"),
        username: get_string_or_return!(items, "username"),
        password: get_string_or_return!(items, "password"),
        graduation_year: get_or_return!(items, "graduation_year").n.clone().unwrap_or_default().parse().unwrap_or_default(),
        team: convert_attribute_vec_to_vec(get_or_return!(items, "team").l.clone().unwrap_or_default()),
        email: get_string_or_return!(items, "email"),
        creation_timestamp: get_or_return!(items, "creation_timestamp").n.clone().unwrap_or_default().parse().unwrap_or_default(),
        last_login: get_or_return!(items, "last_login").n.clone().unwrap_or_default().parse().unwrap_or_default(),
        admin: get_or_return!(items, "admin").bool.unwrap_or_default(),
        staff: get_or_return!(items, "staff").bool.unwrap_or_default(),
        registered: get_or_return!(items, "registered").bool.unwrap_or_default()
    })
}

#[inline(always)]
fn convert_attribute_vec_to_vec(a: Vec<AttributeValue>) -> Vec<String> {
    let mut values = Vec::with_capacity(a.len());
    a.iter().for_each(|x| values.push(x.s.clone().unwrap_or_default()));
    values
} 

pub(crate) async fn update_admin_table() {}

pub(crate) async fn load_admin_table() {
    let mut items = HashMap::with_capacity(1);
    insert_string!(items, "username", username);
    let query = GetItemInput {
        key: items,
        projection_expression: Some(String::from("display_name, username, password, graduation_year, team, email, creation_timestamp, last_login, admin, staff, registered")),
        table_name: String::from("userauth"),
        ..GetItemInput::default()
    };
    let client = DynamoDbClient::new(find_region!());
    let items = match unwrap_db_result!(client.get_item(query).await) {
        Some(v) => v,
        None => return Err(ResponseError::InvalidToken)
    };

    // this is awful, but the dynamo api makes it difficult to deal with
    Ok(NewAccount {
        display_name: get_string_or_return!(items, "display_name"),
        username: get_string_or_return!(items, "username"),
        password: get_string_or_return!(items, "password"),
        graduation_year: get_or_return!(items, "graduation_year").n.clone().unwrap_or_default().parse().unwrap_or_default(),
        team: convert_attribute_vec_to_vec(get_or_return!(items, "team").l.clone().unwrap_or_default()),
        email: get_string_or_return!(items, "email"),
        creation_timestamp: get_or_return!(items, "creation_timestamp").n.clone().unwrap_or_default().parse().unwrap_or_default(),
        last_login: get_or_return!(items, "last_login").n.clone().unwrap_or_default().parse().unwrap_or_default(),
        admin: get_or_return!(items, "admin").bool.unwrap_or_default(),
        staff: get_or_return!(items, "staff").bool.unwrap_or_default(),
        registered: get_or_return!(items, "registered").bool.unwrap_or_default()
    })

}
