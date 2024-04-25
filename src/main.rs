use std::{env};

use serde::{Deserialize, Serialize};
use systemstat::{saturating_sub_bytes, Platform, System};

#[derive(Default, Serialize, Deserialize)]
struct Stat {
    memory: Memory,
    temp: f32,
    uptime: u64
}

#[derive(Default, Serialize, Deserialize)]
struct Memory {
    total: u64,
    free: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    assert_eq!(args.len(), 3, "Poirot takes exactly two arguments");

    let url = &args[1];
    let token = &args[2];
    
    let client = reqwest::Client::new();
    let resp = client
        .get(url.to_owned() + "/ping")
        .header("Authorization", "Bearer ".to_owned() + token)
        .send()
        .await?;
    
    assert_eq!(resp.status(), 200, "Unable to ping the Poirot app!");

    let sys = System::new();

    let memory = match sys.memory() {
        Ok(mem) => Memory {
            total: mem.total.as_u64(),
            free: mem.free.as_u64(),
        },
        Err(_e) => Memory::default(),
    };
    
    let temp = match sys.cpu_temp() {
        Ok(cpu_temp) => cpu_temp,
        Err(_e) => 0.0
    };
    
    let uptime = match sys.uptime() {
        Ok(uptime) => uptime.as_secs(),
        Err(_e) => 0
    };
    
    let stat = Stat{
        memory,
        temp,
        uptime
    };

    let resp = client
        .post(url)
        .json(&stat)
        .send()
        .await?;
    println!("{resp:#?}");

    Ok(())
}
