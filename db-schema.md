All data will be encrypted with AES-256 via MagicCrypt

#### users table:
Primary key: HASH

| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| first_name          | String        | attribute     | 
| last_name           | String        | attribute     |
| display_name        | String        | attribute     |
| username            | String        | primary key   |
| password            | String        | attribute     |
| graduation_year     | u16           | attribute     |
| team  		      | Vec<String>   | attribute     |
| email               | String        | attribute     |
| creation_timestamp  | u64           | attribute     |
| lastlogin_timestamp | u64           | attribute     |

