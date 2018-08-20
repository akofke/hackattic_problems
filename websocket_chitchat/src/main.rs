#![feature(duration_as_u128)]
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate ws;
#[macro_use]
extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"congratulations!.+"(.+)""#).unwrap();
}

#[derive(Deserialize)]
struct Problem {
    token: String
}

#[derive(Serialize)]
struct Solution {
    secret: String
}

struct Client {
    out: ws::Sender,
    last_msg_time: std::time::Instant
}

impl ws::Handler for Client {
    
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.last_msg_time = std::time::Instant::now();
        println!("Open");
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {

        match msg.as_text().unwrap() {
            "ping!" => {
                let elapsed = self.last_msg_time.elapsed().as_millis() as i32;
                let interval = closest_time(elapsed);
                self.last_msg_time = std::time::Instant::now();
                println!("Measured {} -> {}", elapsed, interval);
                self.out.send(interval.to_string());
            },
            "good!" => println!("good!"),
            _ => { 
                println!("Got: {}", msg);
                if let Some(caps) = RE.captures(msg.as_text().unwrap()) {
                    let secret = caps.get(1).unwrap().as_str();
                    println!("{}", secret);
                    let solution = Solution { secret: secret.to_string() };
                    let url = format!("https://hackattic.com/challenges/websocket_chit_chat/solve?access_token={}", env!("HA_API_KEY"));
                    let mut resp = reqwest::Client::new().post(&url).json(&solution).send().unwrap();
                    println!("{:?}", resp.text().unwrap());
                }
            }
        }
        Ok(())
    }
}

const INTERVALS: [i32; 5] = [700, 1500, 2000, 2500, 3000];

fn closest_time(millis: i32) -> i32 {
    *INTERVALS.into_iter().find(|&&interval| { (millis - interval).abs() < 20 }).expect("not close enough!")
}

fn main() {
     let url = format!("https://hackattic.com/challenges/websocket_chit_chat/problem?access_token={}", env!("HA_API_KEY"));
     let problem: Problem = reqwest::get(&url).unwrap().json().unwrap();

     let ws_url = format!("wss://hackattic.com/_/ws/{}", problem.token);
     println!("{}", ws_url);
     ws::connect(ws_url, |out| { Client { out, last_msg_time: std::time::Instant::now() }}).unwrap()

//    ws::connect("wss://echo.websocket.org:443", |out| {
//        move |msg| {
//            println!("{}", msg);
//            Ok(())
//        }
//    }).unwrap();
}
