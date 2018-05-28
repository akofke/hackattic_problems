#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate sha2;

use std::error::Error;
use sha2::{Sha256, Digest};

#[derive(Deserialize, Serialize, Debug)]
struct Problem {
    difficulty: u32,
    block: Block
}

#[derive(Deserialize, Serialize, Debug)]
struct Block {
    data: serde_json::Value,
    nonce: Option<i32>
}

fn main() -> Result<(), reqwest::Error> {
    let url = format!("https://hackattic.com/challenges/mini_miner/problem?access_token={}", env!("HA_API_KEY"));
    let resp = reqwest::get(&url)?;
    let problem: Problem = serde_json::from_reader(resp).unwrap();
    let difficulty = problem.difficulty;
    let j = serde_json::to_string(&Block {data: json!([]), nonce: Some(45)}).unwrap();
    println!("{:?}", problem);
    println!("{:?}", j);
    let mut hasher = sha2::Sha256::new();
    hasher.input(j.as_bytes());
    println!("{:x?}", hasher.result());
    Ok(())
}
