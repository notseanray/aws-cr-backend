#### CR http/REST api backend

This is a REST api backend running on AWS Lambda with dynamodb for the CircuitRunners website.

Heavily a work in progress.


##### env config values

| Key name          | default value <type> | description |
|-------------------|----------------------|-------------|
| CREATION_CODE     | None <String>        | Mandatory code used to create an account |
| MAX_EMAIL_LENGTH | 30 <usize>           | Maximum email length in valid unicode characters accepted from frontend |
| MAX_PASSWORD_LENGTH | 30 <usize> | Maximum password length in valid unicode characters accepted from frontend |
| MIN_PASSWORD_LENGTH | 6 <usize>  | Minimum password length in valid unicode characters accepted from frontend |
| MAX_USERNAME_LENGTH | 30 <usize> | Maximum username length in valid unicode characters accepted from frontend |
