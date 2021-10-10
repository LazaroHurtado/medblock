use super::block::Block;
use super::blockchain::Blockchain;
use rocket::fairing::AdHoc;
use rocket::serde::json;
use rocket::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct Validation {
    is_valid: bool,
}

#[post("/create", format = "json", data = "<block>")]
pub fn create_block(block: json::Json<Block>, blockchain: &State<Blockchain>) -> json::Json<Block> {
    let last_block_hash = blockchain
        .get_most_recent_block()
        .unwrap()
        .get_hash_address()
        .to_string();
    let curr_block = Block::new(block.data.clone(), last_block_hash);
    blockchain.append_block(curr_block);
    json::Json(blockchain.get_most_recent_block().unwrap())
}

#[get("/last_block")]
pub fn get_last_block(blockchain: &State<Blockchain>) -> json::Json<Block> {
    let last_block_hash: Block = blockchain.get_most_recent_block().unwrap();
    json::Json(last_block_hash)
}

#[get("/validate/<company>/<vaccine>")]
pub fn validate_vaccine(
    company: String,
    vaccine: String,
    blockchain: &State<Blockchain>,
) -> json::Json<Validation> {
    let is_valid = blockchain.check_validity(company, vaccine);
    json::Json(Validation { is_valid })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Request Local State", |rocket| async {
        rocket
            .mount(
                "/block",
                routes![create_block, get_last_block, validate_vaccine],
            )
            .manage(Blockchain::new())
    })
}
