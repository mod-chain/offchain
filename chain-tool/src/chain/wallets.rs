use core::str::FromStr;
use std::path::{ Path, PathBuf };
use directories::ProjectDirs;
use serde::{ Deserialize, Serialize };
use subxt_signer::{ SecretUri, bip39::Mnemonic, sr25519::{ Keypair, dev } };
use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WalletDerivationMethod {
    #[default]
    None,
    Mnemonic(String),
    SecretURI(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Wallet {
    pub id: Option<String>,
    pub name: String,
    pub public_key: String,
    pub derivation: WalletDerivationMethod,
    pub timestamp: u64,
}

impl TryFrom<Wallet> for Keypair {
    type Error = String;

    fn try_from(wallet_store: Wallet) -> Result<Keypair, Self::Error> {
        match wallet_store.derivation {
            WalletDerivationMethod::Mnemonic(mnemonic) => {
                let mnemonic = Mnemonic::parse(&mnemonic).map_err(|e| e.to_string())?;
                Keypair::from_phrase(&mnemonic, None).map_err(|e| e.to_string())
            }
            WalletDerivationMethod::SecretURI(secret_uri) => {
                let secret_uri = SecretUri::from_str(&secret_uri).map_err(|e| e.to_string())?;
                Keypair::from_uri(&secret_uri).map_err(|e| e.to_string())
            }
            WalletDerivationMethod::None => Err("No derivation method specified".to_string()),
        }
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            id: None,
            name: "New Wallet".to_string(),
            public_key: "".to_string(),
            derivation: WalletDerivationMethod::None,
            timestamp: 0u64,
        }
    }

    pub fn get_dev_entries() -> [Wallet; 6] {
        let timestamp = std::time::SystemTime
            ::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let alice = Wallet {
            id: Some(Uuid::new_v4().to_string()),
            name: "Alice (dev)".to_string(),
            public_key: dev::alice().public_key().to_account_id().to_string(),
            derivation: WalletDerivationMethod::SecretURI("//Alice".to_string()),
            timestamp,
        };
        let bob = Wallet {
            id: Some(Uuid::new_v4().to_string()),
            name: "Bob (dev)".to_string(),
            public_key: dev::bob().public_key().to_account_id().to_string(),
            derivation: WalletDerivationMethod::SecretURI("//Bob".to_string()),
            timestamp,
        };
        let charlie = Wallet {
            id: Some(Uuid::new_v4().to_string()),
            name: "Charlie (dev)".to_string(),
            public_key: dev::charlie().public_key().to_account_id().to_string(),
            derivation: WalletDerivationMethod::SecretURI("//Charlie".to_string()),
            timestamp,
        };
        let dave = Wallet {
            id: Some(Uuid::new_v4().to_string()),
            name: "Dave (dev)".to_string(),
            public_key: dev::dave().public_key().to_account_id().to_string(),
            derivation: WalletDerivationMethod::SecretURI("//Dave".to_string()),
            timestamp,
        };
        let eve = Wallet {
            id: Some(Uuid::new_v4().to_string()),
            name: "Eve (dev)".to_string(),
            public_key: dev::eve().public_key().to_account_id().to_string(),
            derivation: WalletDerivationMethod::SecretURI("//Eve".to_string()),
            timestamp,
        };
        let ferdie = Wallet {
            id: Some(Uuid::new_v4().to_string()),
            name: "Ferdie (dev)".to_string(),
            public_key: dev::ferdie().public_key().to_account_id().to_string(),
            derivation: WalletDerivationMethod::SecretURI("//Ferdie".to_string()),
            timestamp,
        };

        [alice, bob, charlie, dave, eve, ferdie]
    }

    pub fn get_wallet_filepath(specified_path: Option<String>) -> PathBuf {
        let _wallet_filepath = if let Some(p) = specified_path {
            PathBuf::from(p)
        } else {
            let proj_dirs = ProjectDirs::from("com", "mod-net", "chain-tool").expect(
                "Couldn't get config_dir"
            );
            let config_path = proj_dirs.config_dir();
            config_path.join("wallets.json")
        };
        let wallet_filepath = Path::new(&_wallet_filepath);

        if !wallet_filepath.exists() {
            println!("Creating new wallets file at {}", wallet_filepath.to_str().unwrap());
            // Create parent directories if they don't exist
            std::fs::create_dir_all(&wallet_filepath.parent().unwrap()).unwrap();
            // Write default file contents
            let default_entries = Wallet::get_dev_entries();
            let value = serde_json::to_string_pretty(&default_entries).unwrap();
            std::fs::write(&wallet_filepath, value).unwrap();
        }

        wallet_filepath.to_path_buf()
    }

    pub fn load_wallets(specified_path: Option<String>) -> Option<Vec<Wallet>> {
        let wallet_filepath = Wallet::get_wallet_filepath(specified_path);
        println!("Loading wallets from file at {}", wallet_filepath.to_str().unwrap());
        match std::fs::read_to_string(&wallet_filepath) {
            Ok(json_data) =>
                match serde_json::from_str::<Vec<Wallet>>(&json_data) {
                    Ok(store) => Some(store),
                    Err(err) => {
                        println!("Failed to parse {}: {:?}", wallet_filepath.display(), err);
                        None
                    }
                }
            Err(err) => {
                println!("Failed to read {}: {:?}", wallet_filepath.to_str().unwrap(), err);
                None
            }
        }
    }
}
