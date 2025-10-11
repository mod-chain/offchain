use super::chain;
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient, SubstrateConfig};
use anyhow::Result;

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
                None => None,
            },
            url: match value.url {
                Some(url) => Some(String::from_utf8_lossy(&url.0).to_string()),
                None => None,
            },
            collateral: value.collateral,
            take: value.take.0,
            created_at: value.created_at,
            last_updated: value.last_updated,
        }
    }
}

impl Module {
    pub async fn iter(api: &OnlineClient<SubstrateConfig>) -> Result<Vec<Module>> {
        let mut modules: Vec<Module> = Vec::new();
        let storage_query = chain::storage().modules().modules_iter();
        let mut results = api.storage().at_latest().await?.iter(storage_query).await?;

        while let Some(Ok(kv)) = results.next().await {
            modules.push(kv.value.into());
        }
        Ok(modules)
    }

    pub async fn get(api: &OnlineClient<SubstrateConfig>, id: u64) -> Result<Module> {
      let storage_query = chain::storage().modules().modules(id);
      let result = api.storage().at_latest().await?.fetch(&storage_query).await?;

      match result {
        Some(module) => Ok(module.into()),
        None => Err(anyhow::anyhow!("Module Not Found")),
      }
    }
}
