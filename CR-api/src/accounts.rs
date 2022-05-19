use crate::ResponseError;
use crate::{env_or_default, env_or_exit};
use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use rusoto_dynamodb::{AttributeValue, PutItemInput};
use serde::Deserialize;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sha3::{Digest, Sha3_256, Sha3_512};
use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};

lazy_static! {
    /// Key used to sign up an admin account
    pub static ref ADMIN_KEY: String = env_or_exit!("ADMIN_KEY");

    /// Key used to encrypt encoding of token timestamp
    pub static ref TOKEN_KEY: String = env_or_exit!("TOKEN_KEY");

    /// Max length of email address (inclusive)
    pub static ref MAX_EMAIL_LENGTH: usize = env_or_default!("MAX_EMAIL_LENGTH", 30);

    /// Max length of raw password (inclusive)
    pub static ref MAX_PASSWORD_LENGTH: usize = env_or_default!("MAX_PASSWORD_LENGTH", 30);

    /// Minimum length of raw password (inclusive)
    pub static ref MIN_PASSWORD_LENGTH: usize = env_or_default!("MIX_PASSWORD_LENGTH", 6);

    /// Max length of raw username, must be at least 8 characters long
    pub static ref MAX_USERNAME_LENGTH: usize = {

        // We need this to be at least 8 long
        let length = env_or_default!("MAX_USERNAME_LENGTH", 30);
        match length {
            8.. => length,
            _ => 30
        }
    };

    /// Supplied code used for signup
    pub static ref CREATION_CODE: String = env_or_exit!("CREATION_CODE");
}

/// Constant time comparison, when we perform a lookup of passwords we want to prevent a timeout
/// from occuring.
#[macro_export]
macro_rules! constant_time_compare {
    ($val1:expr, $val2:expr) => {
        match $val1.len() == $val2.len() {
            true => {
                let mut result = 0;
                for (x, y) in $val1.chars().zip($val2.chars()) {
                    result |= x as u32 ^ y as u32;
                }
                result == 0
            }
            false => false,
        }
    };
}

/// UnixTimestamp since epoch in seconds
#[macro_export]
macro_rules! timestamp {
    () => {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    };
}

pub(crate) fn match_team(t: Vec<String>) -> Result<Vec<String>, ResponseError> {
    let mut teams = Vec::with_capacity(1);

    // ensure that we only get valid teams
    for e in t {
        teams.push(match e.as_str() {
            "FTC1002" | "FTC11347" | "FRC1002" | "BEST" => e,
            _ => return Err(ResponseError::InvalidTeam),
        });
    }

    // janky as hell, but verify that someone isn't on both FTC teams and that they aren't on an
    // FTC and best team
    if teams.is_empty()
        || teams.len() > 2
        || teams.contains(&String::from("FTC1002")) && teams.contains(&String::from("FTC11347"))
        || (teams.contains(&String::from("FTC1002")) || teams.contains(&String::from("FTC11347")))
            && teams.contains(&String::from("BEST"))
    {
        return Err(ResponseError::InvalidTeamAssignment);
    }

    Ok(teams)
}

pub(crate) struct NewAccount {
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub graduation_year: u16,
    pub team: Vec<String>,
    pub email: String,
    pub creation_timestamp: u64,
    pub token: String,
    pub last_login: u64,
    pub admin: bool,
    pub registered: bool,
}

#[macro_export]
macro_rules! insert_string {
    ($map:expr, $key:expr, $val:expr) => {
        $map.insert(
            String::from($key),
            AttributeValue {
                s: Some(String::from($val)),
                ..AttributeValue::default()
            },
        );
    };
}

#[macro_export]
macro_rules! insert_number {
    ($map:expr, $key:expr, $val:expr) => {
        $map.insert(
            String::from($key),
            AttributeValue {
                n: Some($val.to_string()),
                ..AttributeValue::default()
            },
        );
    };
}

#[macro_export]
macro_rules! insert_bool {
    ($map:expr, $key:expr, $val:expr) => {
        $map.insert(
            String::from($key),
            AttributeValue {
                r#bool: Some($val),
                ..AttributeValue::default()
            },
        );
    };
}

impl From<NewAccount> for PutItemInput {
    fn from(a: NewAccount) -> PutItemInput {
        let mut items = HashMap::with_capacity(10);
        insert_string!(items, "username", a.username);
        insert_string!(items, "password", a.password);
        insert_string!(items, "email", a.email);
        insert_string!(items, "display_name", a.display_name);
        insert_number!(items, "creation_timestamp", a.creation_timestamp);
        insert_number!(items, "graduation_year", a.graduation_year);
        insert_number!(items, "last_login", 0);
        insert_bool!(items, "admin", a.admin);
        // if they're an admin they are registered
        insert_bool!(items, "registered", a.admin);

        let mut teams = Vec::with_capacity(a.team.len());
        a.team.iter().for_each(|x| {
            teams.push(AttributeValue {
                s: Some(x.to_owned()),
                ..AttributeValue::default()
            })
        });

        items.insert(
            String::from("team"),
            AttributeValue {
                l: Some(teams),
                ..AttributeValue::default()
            },
        );

        PutItemInput {
            condition_expression: Some(String::from("attribute_not_exists")),
            item: items,
            table_name: String::from("userauth"), // TODO replace with env
            ..PutItemInput::default()
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct CreateAccountEvent {
    pub first_name: String,
    pub last_name: String,
    pub creation_code: String,
    pub password: String,
    pub graduation_year: u16,
    pub team: Vec<String>,
    pub email: String,
    pub admin_key: Option<String>,
}

pub(crate) fn token_gen(username: &str, time: u64) -> String {

    let mut hasher = Sha3_256::new();
    hasher.update(time.to_string().as_bytes());

    // sha3 256 is 128 characters long
    let mut timestamp_hash = String::with_capacity(128);

    // convert to hex string since it's easier to deal with
    for byte in hasher.finalize() {
        timestamp_hash.push_str(&format!("{:02X}", byte));
    }

    let mut hasher = Sha3_256::new();
    hasher.update(username.as_bytes());

    let mut username = String::with_capacity(128);

    for byte in hasher.finalize() {
        username.push_str(&format!("{:02X}", byte));
    }

    let encrypt = new_magic_crypt!(&*TOKEN_KEY, 128);
    format!(
        "{}.{}.{}", 
        timestamp_hash,
        encrypt.encrypt_str_to_base64((time + 864000).to_string()), // expires after 10 days
        username,
    )
}

impl CreateAccountEvent {
    pub(crate) fn validate_account(a: CreateAccountEvent) -> Result<NewAccount, ResponseError> {
        if !constant_time_compare!(a.creation_code, *CREATION_CODE) {
            return Err(ResponseError::InvalidCreationCode);
        }

        let admin = Self::is_admin(a.admin_key);

        let time = timestamp!();

        let username = Self::generate_username(&a.first_name, &a.last_name, a.graduation_year)?;

        // These are intentionally out of order to prevent extra cloning of strings and to ensure
        // that things are verified in order
        Ok(NewAccount {
            username,
            display_name: Self::validate_display_name(&a.first_name, &a.last_name)?,
            password: Self::validate_password(&a.password)?,
            graduation_year: Self::validate_graduation_year(a.graduation_year)?,
            team: match_team(a.team)?,
            email: Self::validate_email(&a.email)?,
            creation_timestamp: time,
            admin,
            last_login: time,
            registered: admin,
            token: token_gen(&a.first_name, time)
        })
    }

    // Subtract 7 from here since .last_initial-grad_year would always be 7 long
    fn validate_display_name(first: &str, last: &str) -> Result<String, ResponseError> {
        Ok(format!("{first} {last}"))
    }

    // The first/last name should be valid since we check the display name first, this function
    // assumes that the last name is not empty
    /// Generate username of user based on {first}.{last_initial}-{grad_year} format
    pub(crate) fn generate_username(
        first: &str,
        last: &str,
        grad_year: u16,
    ) -> Result<String, ResponseError> {
        if first.len() < 2
            || first.len() > *MAX_USERNAME_LENGTH - 7
            || last.len() < 2
            || last.len() > 15
        {
            return Err(ResponseError::InvalidName);
        }

        let lastname_initial = last.chars().collect::<Vec<char>>()[0];

        Ok(format!("{first}.{lastname_initial}-{grad_year}"))
    }

    /// Check the raw password against given length constraints and hash it
    fn validate_password(password: &str) -> Result<String, ResponseError> {
        if password.len() > *MAX_PASSWORD_LENGTH || password.len() < *MIN_PASSWORD_LENGTH {
            return Err(ResponseError::InvalidPassword);
        }

        let mut hasher = Sha3_512::new();
        hasher.update(password.as_bytes());

        // sha3 512 is 128 characters long
        let mut hex_string = String::with_capacity(128);

        // convert to hex string since it's easier to deal with
        for byte in hasher.finalize() {
            hex_string.push_str(&format!("{:02X}", byte));
        }

        Ok(hex_string)
    }

    /// Compare the year given to see if it's more than 4 years from now or more than 1 year ago
    #[inline(always)]
    pub(crate) fn validate_graduation_year(year: u16) -> Result<u16, ResponseError> {
        // If it's less than 1970 we could cause the year to wrap in release mode when subtracting
        // 1970, this would technically be fine since it would not pass the second set of
        // conditions but it's better to be safe
        if year < 1970 {
            return Err(ResponseError::InvalidGraduationYear);
        }

        // Unix epoch is 1970
        let mut offset = year as u64 - 1970;

        // seconds in a year
        offset *= 31557600;

        let date = timestamp!();

        // if the given year is more than 1 year ago or greater than 4 years from now we responde
        // with an error
        if offset < date - 31557600 || offset > date + 126230400 {
            return Err(ResponseError::InvalidGraduationYear);
        }

        Ok(year)
    }

    /// Veryify provided email against name@domain.com regex pattern, then trim the result
    fn validate_email(email: &str) -> Result<String, ResponseError> {
        // you know it's gonna be good when the regex is taken from some random stack overflow post
        // :D
        let email_regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).expect("Invalid Regex Provided");

        match email_regex.is_match(email) && email.len() < *MAX_EMAIL_LENGTH {
            true => Ok(email.trim().to_owned()),
            false => Err(ResponseError::InvalidEmail),
        }
    }

    fn is_admin(key: Option<String>) -> bool {
        match key {
            Some(v) => constant_time_compare!(v, *ADMIN_KEY),
            None => false,
        }
    }
}

// Cloud gets zero bitches
