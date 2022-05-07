use crate::ResponseError;
use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use serde::Deserialize;
use sha3::{Digest, Sha3_512};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

macro_rules! env_or_default {
    ($var:expr, $default:expr) => {
        match env::var($var).ok() {
            Some(v) => v.parse().unwrap_or($default),
            None => $default,
        }
    };
}

lazy_static! {
    pub static ref MAX_EMAIL_LENGTH: usize = env_or_default!("MAX_EMAIL_LENGTH", 30);
    pub static ref MAX_PASSWORD_LENGTH: usize = env_or_default!("MAX_PASSWORD_LENGTH", 30);
    pub static ref MIN_PASSWORD_LENGTH: usize = env_or_default!("MIX_PASSWORD_LENGTH", 6);
    pub static ref MAX_USERNAME_LENGTH: usize = {
        // We need this to be at least 7 long
        let length = env_or_default!("MAX_USERNAME_LENGTH", 30);
        match length {
            8.. => length,
            _ => 30
        }
    };
    pub static ref CREATION_CODE: String = {
        match env::var("CREATION_CODE").ok() {
            Some(v) => v,
            None => {
                error!("fill out CREATION_CODE in env");
                std::process::exit(1);
            }
        }
    };
}

/// Constant time comparison, when we perform a lookup of passwords we want to prevent a timeout
/// from occuring.
///
/// Sha3 512 hash has a fixed length so we can use this comparison function to ensure that we have
/// no issues and it's secure
// TODO
// replace since we know length of sha3 512 hash
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
    for e in t {
        teams.push(match e.as_str() {
            "FTC1002" | "FTC11347" | "FRC1002" | "BEST" => e,
            _ => return Err(ResponseError::InvalidTeam),
        });
    }

    // janky as hell, but verify that someone isn't on both FTC teams
    if teams.is_empty()
        || teams.len() > 3
        || teams.contains(&String::from("FTC1002")) && teams.contains(&String::from("FTC11347"))
    {
        return Err(ResponseError::InvalidTeamAssignment);
    }
    Ok(teams)
}

pub(crate) struct NewAccount {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub graduation_year: u16,
    pub team: Vec<String>,
    pub email: String,
    pub creation_timestamp: u64,
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
}

impl CreateAccountEvent {
    pub(crate) fn validate_account(a: CreateAccountEvent) -> Result<NewAccount, ResponseError> {
        if a.creation_code == *CREATION_CODE {
            return Err(ResponseError::InvalidCreationCode);
        }

        // These are intentionally out of order to prevent extra cloning of strings and to ensure
        // that things are verified in order
        Ok(NewAccount {
            display_name: Self::validate_display_name(&a.first_name, &a.last_name)?,
            password: Self::validate_password(&a.password)?,
            graduation_year: Self::validate_graduation_year(a.graduation_year)?,
            username: Self::generate_username(&a.first_name, &a.last_name, a.graduation_year)?,
            team: match_team(a.team)?,
            email: Self::validate_email(&a.email)?,
            creation_timestamp: timestamp!(),
            first_name: a.first_name,
            last_name: a.last_name,
        })
    }

    // Subtract 7 from here since .last_initial-grad_year would always be 7 long
    fn validate_display_name(first: &str, last: &str) -> Result<String, ResponseError> {
        if first.len() < 2
            || first.len() > *MAX_USERNAME_LENGTH - 7
            || last.len() < 2
            || last.len() > 15
        {
            return Err(ResponseError::InvalidName);
        }
        Ok(format!("{first} {last}"))
    }

    // The first/last name should be valid since we check the display name first, this function
    // assumes that the last name is not empty
    //
    // Username format is first.last_initial-grad_year
    fn generate_username(first: &str, last: &str, grad_year: u16) -> Result<String, ResponseError> {
        let lastname_initial = last.chars().collect::<Vec<char>>()[0];

        Ok(format!("{first}.{lastname_initial}-{grad_year}"))
    }

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

    fn validate_graduation_year(year: u16) -> Result<u16, ResponseError> {
        unimplemented!();
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
}
