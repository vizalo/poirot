use std::{env, thread};

use serde::{Deserialize, Serialize};
use systemstat::{saturating_sub_bytes, ByteSize, Duration, Platform, System};

#[derive(Default, Serialize, Deserialize, Debug)]
struct Stat {
    cpu: Cpu,
    memory: Memory,
    temp: f32,
    uptime: u64,
    filsystem: Filesystem,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Cpu {
    total: f32,
    sys: f32,
    user: f32,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Memory {
    total: u64,
    free: u64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Filesystem {
    total: u64,
    free: u64,
    available: u64,
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

    let cpu = match sys.cpu_load() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = &cpu.done().unwrap()[0];
            let cpu_result = Cpu {
                total: (cpu.user * 100.0) + (cpu.system * 100.0),
                sys: cpu.system * 100.0,
                user: cpu.user * 100.0,
            };
            cpu_result
        }
        Err(_e) => Cpu::default(),
    };

    println!("{cpu:#?}");

    let memory = match sys.memory() {
        Ok(mem) => Memory {
            total: mem.total.as_u64(),
            free: mem.free.as_u64(),
        },
        Err(_e) => Memory::default(),
    };

    let temp = match sys.cpu_temp() {
        Ok(cpu_temp) => cpu_temp,
        Err(_e) => 0.0,
    };

    let uptime = match sys.uptime() {
        Ok(uptime) => uptime.as_secs(),
        Err(_e) => 0,
    };

    let fs = match sys.mount_at("/") {
        Ok(mount) => Filesystem {
            total: ByteSize::b(mount.total.as_u64()).as_u64(),
            free: ByteSize::b(mount.free.as_u64()).as_u64(),
            available: ByteSize::b(mount.avail.as_u64()).as_u64(),
        },
        Err(_e) => Filesystem::default(),
    };

    let stat = Stat {
        cpu,
        memory,
        temp,
        uptime,
        filsystem: fs,
    };

    println!("{stat:#?}");
    let resp = client.post(url).json(&stat).send().await?;

    Ok(())
}
