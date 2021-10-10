use chrono::prelude::Utc;
use crypto::{digest::Digest, sha2::Sha256};
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Default, FromForm, Deserialize, Serialize)]
pub struct ScanPoints {
    pub vendor_address: String,
    pub timestamp: String,
    pub message: Option<String>,
}

#[derive(Clone, Default, FromForm, Deserialize, Serialize)]
pub struct BlockData {
    pub to_address: String,
    pub from_address: String,
    pub supply_name: String,
    pub supply_amount: u32,
    pub vendor_pks: Vec<String>,
    pub scan_points: Option<Vec<ScanPoints>>,
}

#[derive(Clone, FromForm, Deserialize, Serialize)]
pub struct Block {
    #[serde(skip_deserializing)]
    pub hash_address: String,
    #[serde(skip_deserializing)]
    pub timestamp: String,
    pub data: BlockData,
    pub last_block_hash: String,
}

impl Block {
    pub fn new(data: BlockData, last_block_hash: String) -> Self {
        let hash_address = Self::generate_hash(&data.supply_name);
        let timestamp = Self::get_now_timestamp();

        Self {
            hash_address,
            timestamp,
            data,
            last_block_hash,
        }
    }

    pub fn get_hash_address(&self) -> &String {
        &self.hash_address
    }

    fn get_now_timestamp() -> String {
        Utc::now().to_string()
    }

    fn generate_hash(block_supply: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.input(block_supply.to_string().as_bytes());
        hasher.result_str()
    }
}
