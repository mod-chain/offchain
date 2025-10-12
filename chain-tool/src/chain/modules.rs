use crate::{ AccountIdOf, BalanceOf, Block, StorageReference, URLReference };
use super::{ chain, ChainConfig };
use serde::{ Serialize, Deserialize };
use subxt::{ utils::AccountId32, OnlineClient };
use anyhow::Result;
use sp_arithmetic::Percent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleName(Vec<u8>);

impl std::fmt::Display for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0).to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleTier {
    Official,
    Approved,
    Unapproved,
    Delisted,
}

impl From<chain::runtime_types::pallet_modules::module::module::ModuleTier> for ModuleTier {
    fn from(value: chain::runtime_types::pallet_modules::module::module::ModuleTier) -> Self {
        match value {
            chain::runtime_types::pallet_modules::module::module::ModuleTier::Official =>
                ModuleTier::Official,
            chain::runtime_types::pallet_modules::module::module::ModuleTier::Approved =>
                ModuleTier::Approved,
            chain::runtime_types::pallet_modules::module::module::ModuleTier::Unapproved =>
                ModuleTier::Unapproved,
            chain::runtime_types::pallet_modules::module::module::ModuleTier::Delisted =>
                ModuleTier::Delisted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    pub owner: AccountIdOf,
    pub id: Option<u64>,
    pub name: ModuleName,
    pub data: StorageReference,
    pub url: URLReference,
    pub collateral: BalanceOf,
    pub take: Percent,
    pub tier: ModuleTier,
    pub created_at: Block,
    pub last_updated: Block,
}

impl From<chain::runtime_types::pallet_modules::module::module::Module> for Module {
    fn from(value: chain::runtime_types::pallet_modules::module::module::Module) -> Self {
        Self {
            owner: value.owner.to_string(),
            id: Some(value.id),
            name: ModuleName(value.name.0),
            data: match value.data {
                Some(d) => Some(d.0),
                None => None,
            },
            url: match value.url {
                Some(u) => Some(u.0),
                None => None,
            },
            collateral: value.collateral,
            take: Percent::from_parts(value.take.0),
            tier: value.tier.into(),
            created_at: value.created_at,
            last_updated: value.last_updated,
        }
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new()
    }
}

impl Module {
    pub fn new() -> Self {
        Self {
            owner: String::new(),
            id: None,
            name: ModuleName(Vec::new()),
            data: None,
            url: None,
            collateral: 0,
            take: Percent::zero(),
            tier: ModuleTier::Unapproved,
            created_at: 0,
            last_updated: 0,
        }
    }

    pub async fn iter(api: &OnlineClient<ChainConfig>) -> Result<Vec<Module>> {
        let mut modules = Vec::<Module>::new();
        let storage_query = chain::storage().modules().modules_iter();
        let mut results = api.storage().at_latest().await?.iter(storage_query).await?;

        while let Some(Ok(kv)) = results.next().await {
            let module: Module = kv.value.into();
            modules.push(module);
        }
        Ok(modules)
    }

    pub async fn get(api: &OnlineClient<ChainConfig>, id: u64) -> Result<Module> {
        let storage_query = chain::storage().modules().modules(id);
        let result = api.storage().at_latest().await?.fetch(&storage_query).await?;

        match result {
            Some(module) => Ok(module.into()),
            None => Err(anyhow::anyhow!("Module Not Found")),
        }
    }
}
