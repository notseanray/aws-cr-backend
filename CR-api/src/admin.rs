pub(crate) struct UpdateEventEvent {
    pub token: String,
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub date: Option<u64>,
    pub description: Option<String>,
    pub attendence: Option<Vec<String>>
}

impl UpdateEventEvent {
    pub(crate) async fn create_event() {}
    pub(crate) async fn remove_event() {}
    pub(crate) async fn update_event() {}
}

pub(crate) struct UpdateAdminsEvent {
    pub token: String,
    pub master_key: String,
    pub username: String
}

impl UpdateAdminsEvent {
    pub(crate) async fn add_admin() {}
    pub(crate) async fn remove_admin() {}
}

pub(crate) struct UpdateAdminDetailsEvent {
    pub token: String,
    pub master_key: Option<String>,
    pub creation_code: Option<String>,
    pub admin_key: Option<String>,
    pub max_email_length: Option<String>,
    pub max_password_length: Option<String>,
    pub min_password_length: Option<String>,
    pub max_username_length: Option<String>,
    pub encryption_key: Option<String>,
    pub encryption_key_out: Option<String>,
    pub db_encryption_key: Option<String>
}

impl UpdateAdminDetailsEvent {
    // verify, storage call
    pub(crate) async fn update_admin_details() {}
}
