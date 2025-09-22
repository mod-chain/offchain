use super::chain;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    pub owner: String,
    pub id: u64,
    pub name: String,
    pub data: Option<String>,
    pub url: Option<String>,
    pub collateral: u128,
    pub take: u8,
    pub created_at: u64,
    pub last_updated: u64,
}

impl From<chain::runtime_types::pallet_modules::module::module::Module> for Module {
    fn from(value: chain::runtime_types::pallet_modules::module::module::Module) -> Self {
        Self {
            owner: value.owner.to_string(),
            id: value.id,
            name: String::from_utf8_lossy(&value.name.0).to_string(),
            data: match value.data {
                Some(data) => Some(String::from_utf8_lossy(&data.0).to_string()),
                None => None
            },
            url:  match value.url {
                Some(url) => Some(String::from_utf8_lossy(&url.0).to_string()),
                None => None
            },
            collateral: value.collateral,
            take: value.take.0,
            created_at: value.created_at,
            last_updated: value.last_updated,
        }
    }
}