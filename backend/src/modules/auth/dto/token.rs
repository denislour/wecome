use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenClaim {
    pub user_id: String,
    pub is_admin: bool,
}
