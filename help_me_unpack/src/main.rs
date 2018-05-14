extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate base64;
extern crate byteorder;

#[macro_use]
extern crate serde_derive;

use std::env;
use base64::{decode};
use byteorder::{ByteOrder, BigEndian, LittleEndian};

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    bytes: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Solution {
    int: i32,
    uint: u32,
    short: i16,
    float: f64, // needs to be f64 so it's formatted correctly
    double: f64,
    big_endian_double: f64
}

fn main() -> Result<(), reqwest::Error> {
    let url = format!("https://hackattic.com/challenges/help_me_unpack/problem?access_token={}", env!("HA_API_KEY"));
    let resp = reqwest::get(&url)?;
    let v: Response = serde_json::from_reader(resp).unwrap();
    let bytes = decode(v.bytes.as_str()).unwrap();
    println!("{:?}, {}", bytes, bytes.len());
    println!("{:?}, {:?}", &bytes[..4], &bytes[26..]);
    let solution = Solution {
        int: LittleEndian::read_i32(&bytes[..4]),
        uint: LittleEndian::read_u32(&bytes[4..8]),
        short: LittleEndian::read_i16(&bytes[8..10]),
        float: LittleEndian::read_f32(&bytes[12..16]) as f64,
        double: LittleEndian::read_f64(&bytes[16..24]),
        big_endian_double: BigEndian::read_f64(&bytes[24..32])
    };

    println!("{}", serde_json::to_string_pretty(&solution).unwrap());
    post_solution(solution);
    return Ok(())
}

fn post_solution(solution: Solution) {
    let client = reqwest::Client::new();
    let mut resp = client.post(&format!("https://hackattic.com/challenges/help_me_unpack/solve?access_token={}", env!("HA_API_KEY")))
        .json(&solution)
        .send().unwrap();
    println!("{:?}", resp.text().unwrap());
}
