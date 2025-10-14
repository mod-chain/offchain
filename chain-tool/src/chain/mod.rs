use subxt::SubstrateConfig;

#[subxt::subxt(runtime_metadata_path = "../metadata.scale")]
pub mod chain {}
pub type ChainConfig = SubstrateConfig;

mod modules;
pub use modules::*;
mod wallets;
pub use wallets::*;