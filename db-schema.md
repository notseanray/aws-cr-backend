All data will be encrypted with AES-256 via MagicCrypt

#### users table:
Composite Primary Key

Primary key: HASH

Sort Key: RANGE

| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| first_name          | String        |          	  | 
| last_name           | String        | 			  |
| display_name        | String        | 			  |
| username            | String        | attribute     |
| password            | String        | sort key      |
| graduation_year     | u16           |               |
| team  		      | Vec<String>   | attribute     |
| email               | String        | partition key |
| creation_timestamp  | u64           |	   			  |
| lastlogin_timestamp | u64           |				  |

