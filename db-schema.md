All data will be encrypted with AES-256 via MagicCrypt

#### userauth table:

| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| username            | String        | primary key   |
| display_name        | String        | attribute     |
| password            | String        | attribute     |
| graduation_year     | u16           | attribute     |
| team  		      | Vec<String>   | attribute     |
| email               | String        | attribute     |
| creation_timestamp  | u64           | attribute     |
| lastlogin           | u64           | attribute     |
| admin               | bool          | attribute     |
| staff               | bool          | attribute     |
| registered          | bool          | attribute     |

#### tokentable:
| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| token               | String        | primary key   |
| username            | String        | attribute     |

#### usertable:
| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| username            | String        | primary key   |
| token               | String        | attribute     |

#### eventstable:

BatchGetItem then sort client side
possibly use LSI

| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| type                | String        | primary key   |
| name                | String        | attribute     |
| date                | u64           | attribute     |
| description         | String        | attribute     |
| attendence          | Vec<String>   | attribute     |

#### logstable:

BatchGetItem then sort client side
possibly use LSI

| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| EventType           | String        | primary key   |
| username            | String        | attribute     |
| timestamp           | u64           | attribute     |


#### admintable:
| Key                 | Type          | Special  	  |
|---------------------|---------------|---------------|
| year                | u32           | primary key   |
| creation_code       | String        | attribute     |
| admin_key           | String        | attribute     |
| max_email_length    | u32           | attribute     |
| max_password_length | u32           | attribute     |
| min_password_length | u32           | attribute     |
| max_username_length | u32           | attribute     |
| encryption_key      | String        | attribute     |
| encryption_key_out  | String        | attribute     |
| db_encryption_key   | String        | attribute     |
| master_key          | String        | attribute     |
| last_update         | u64           | attribute     |
| last_update_by      | String        | attribute     |

### notes:
* use BatchGetItemInput for viewing events or anything, faster
