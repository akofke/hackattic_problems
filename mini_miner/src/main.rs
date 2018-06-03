#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate sha2;
extern crate rand;
extern crate byteorder;

use std::error::Error;
use sha2::{Digest};
use rand::prelude::*;
use byteorder::{ByteOrder};

#[derive(Deserialize, Serialize, Debug)]
struct Problem {
    difficulty: u32,
    block: Block
}

#[derive(Deserialize, Serialize, Debug)]
struct Block {
    data: serde_json::Value,
    nonce: Option<u32>
}

fn main() -> Result<(), reqwest::Error> {
    let url = format!("https://hackattic.com/challenges/mini_miner/problem?access_token={}", env!("HA_API_KEY"));
    let resp = reqwest::get(&url)?;
    let problem: Problem = serde_json::from_reader(resp).unwrap();
    let difficulty = problem.difficulty;
    println!("{:?}", problem);

    let mut rng = thread_rng();

    let nonce = loop {
        let nonce: u32 = rng.gen();
        let j = serde_json::to_string(&Block {data: problem.block.data.clone(), nonce: Some(nonce)}).unwrap();
        let mut hasher = sha2::Sha256::new();
        hasher.input(j.as_bytes());
        let bytes = hasher.result();
        let zero_bits = leading_zero_bits(&bytes);
        if zero_bits == difficulty {
            break nonce;
        }
    };

    println!("{}", nonce);
    post_solution(json!({"nonce": nonce}));
    Ok(())
}

fn leading_zero_bits(bytes: &[u8]) -> u32 {
    let i = byteorder::BigEndian::read_u32(bytes);
    return i.leading_zeros();
}

fn post_solution(solution: serde_json::Value) {
    let client = reqwest::Client::new();
    let mut resp = client.post(&format!("https://hackattic.com/challenges/mini_miner/solve?access_token={}", env!("HA_API_KEY")))
        .json(&solution)
        .send().unwrap();
    println!("{:?}", resp.text().unwrap());
}
