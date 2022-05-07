use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ResponseError {
    #[error("Invalid Request")]
    InvalidRequest,
    #[error("Invalid Team Provided")]
    InvalidTeam,
    #[error("Invalid Team Assignment")]
    InvalidTeamAssignment,
    #[error("Invalid Username Provided")]
    InvalidUsername,
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
    #[error("AWS Lambda Error: Failed to access database")]
    FailedToAccessDatabase,
}
