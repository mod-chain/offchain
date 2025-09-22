mod module;
pub use module::*;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod chain {}