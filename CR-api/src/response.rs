use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ResponseError {
    #[error("Invalid Request")]
    InvalidRequest,
    #[error("Malformed Request")]
    MalformedRequest,
    #[error("Invalid Decryption Result")]
    InvalidDecryptionResult,
    #[error("Invalid Team Provided")]
    InvalidTeam,
    #[error("Invalid Team Assignment")]
    InvalidTeamAssignment,
    #[error("Invalid Name Provided")]
    InvalidName,
    #[error("Invalid Username Provided")]
    InvalidUsername,
    #[error("Invalid Token Provided")]
    InvalidToken,
    #[error("Invalid Password Provided")]
    InvalidPassword,
    #[error("Invalid Creation Code")]
    InvalidCreationCode,
    #[error("Invalid Graduation Year")]
    InvalidGraduationYear,
    #[error("Invalid Email Provided")]
    InvalidEmail,
    #[error("Invalid Timestamp")]
    InvalidTimestamp,
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Invalid Permissions")]
    InvalidPermissions,
}
