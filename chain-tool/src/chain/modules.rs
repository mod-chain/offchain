use crate::{
    AccountIdOf,
    AppState,
    BalanceOf,
    Block,
    StorageReference,
    URLReference,
};
use super::{ chain, ChainConfig };
use serde::{ Serialize, Deserialize };
use subxt::{ OnlineClient };
use anyhow::Result;
use sp_arithmetic::Percent;
use subxt_signer::sr25519::Keypair;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleName(pub Vec<u8>);

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
    NotRegistered,
}

impl ModuleTier {
    pub fn all() -> Vec<Self> {
        vec![Self::Official, Self::Approved, Self::Unapproved, Self::Delisted]
    }
}

impl std::fmt::Display for ModuleTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleTier::Official => write!(f, "Official"),
            ModuleTier::Approved => write!(f, "Approved"),
            ModuleTier::Unapproved => write!(f, "Unapproved"),
            ModuleTier::Delisted => write!(f, "Delisted"),
            ModuleTier::NotRegistered => write!(f, "Not Registered"),
        }
    }
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

    pub async fn authorized_module(api: &OnlineClient<ChainConfig>) -> Result<u64> {
        let storage_query = chain::storage().module_payments().authorized_module();
        let result = api.storage().at_latest().await?.fetch(&storage_query).await?;

        match result {
            Some(authorized_module) => Ok(authorized_module),
            // None => Err(anyhow::anyhow!("Authorized Module failed to be queried."))
            None => Ok(0),
        }
    }

    pub async fn register(&self, api: &OnlineClient<ChainConfig>, state: &AppState) -> Result<()> {
        println!("Register: {:?}", &self);
        let wallets = state.wallets.clone();
        if !self.owner.is_empty() {
            let wallets = wallets.unwrap();
            let selected_wallet = wallets
                .iter()
                .find(|w| w.public_key == self.owner)
                .unwrap();

            let module_registration_tx = chain
                ::tx()
                .modules()
                .register_module(
                    chain::runtime_types::bounded_collections::bounded_vec::BoundedVec(
                        self.name.0.clone()
                    ),
                    match &self.data {
                        Some(data) =>
                            Some(
                                chain::runtime_types::bounded_collections::bounded_vec::BoundedVec(
                                    data.clone()
                                )
                            ),
                        None => None,
                    },
                    match &self.url {
                        Some(url) =>
                            Some(
                                chain::runtime_types::bounded_collections::bounded_vec::BoundedVec(
                                    url.clone()
                                )
                            ),
                        None => None,
                    },
                    // Some((self.take.deconstruct() as u32).into())
                    None
                );

            let from = Keypair::try_from(selected_wallet.clone()).unwrap();
            let events = api
                .tx()
                .sign_and_submit_then_watch_default(&module_registration_tx, &from).await?
                .wait_for_finalized_success().await?;

            let register_event = events.find_first::<chain::modules::events::ModuleRegistered>()?;
            if let Some(event) = register_event {
                println!("Register Success: {event:?}");
            }

            Ok(())
        } else {
            Err(anyhow::anyhow!("Register function expects owner to be set"))
        }
    }
}
