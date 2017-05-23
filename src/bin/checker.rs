extern crate ringer;
extern crate chrono;
extern crate curl;
extern crate futures;
extern crate futures_cpupool;
extern crate dotenv;
extern crate serde_json;
use std::env;

use futures::future;
use futures_cpupool::CpuPool;

use std::{thread, time};
use ringer::models::{Check, CheckMeta};
use ringer::error::{Result, Error};
use ringer::utils::alert_on_error_code;
use curl::easy::Easy;

use chrono::UTC;
use dotenv::dotenv;

// REQUIRES DATABASE_URL, MATTERMOST_URL, MATTERMOST_HOOK, APP_URL, API_VER, MASTER_KEY

fn min_rate(checks: &[Check]) -> u64 {
    checks
        .iter()
        .fold(60, |s, x| if s < x.rate { s } else { x.rate }) as u64
}

fn async_check(check: &mut Check) -> future::FutureResult<(), Error> {
    if check.conditional_perform().unwrap() {
        let meta: CheckMeta = serde_json::from_value(check.meta.to_owned().unwrap_or_default())
            .unwrap();
        for checker in meta.checkers {
            let fun = match checker.as_ref() {
                "alert_on_error_code" => alert_on_error_code,
                _ => unreachable!()
            };  
            match fun(check) {
                Ok(_) => {}
                Err(e) => return future::err(e),
            }
        }
    };
    future::ok(())
}

fn run() -> Result<()> {
    let cpu_pool = CpuPool::new(10);

    let mut checks;
    let mut idle_time: u64;
    let mut start;
    let mut duration: u64;
    let mut now;
    let mut ticks: u64;
    loop {
        let mut publish_interval = 10;
        start = UTC::now().naive_utc();
        checks = Check::get_all(None)?;
        let l = checks.len();
        idle_time = min_rate(&checks[..]);
        let mut futures = Vec::new();
        // Test if this is truly async :)
        for mut check in checks {
            let future = cpu_pool.spawn(async_check(&mut check));
            futures.push(future);
        }

        ticks = idle_time / publish_interval;
        now = UTC::now().naive_utc();
        println!("{} - Performing {}/{} checks", now, futures.len(), l);
        duration = now.signed_duration_since(start).num_seconds() as u64;
        if duration < publish_interval {
            publish_interval -= duration
        }
        for _ in 0..ticks {
            // println!("{}", UTC::now());
            trigger_sse()?;
            thread::sleep(time::Duration::from_secs(publish_interval));
        }

    }
}

fn trigger_sse() -> Result<u32> {
    dotenv().ok();

    let url = env::var("APP_URL").expect("APP_URL must be set");
    let api_ver = env::var("API_VER").expect("API_VER must be set");
    let key = env::var("MASTER_KEY").expect("MASTER_KEY must be set");

    let mut easy = Easy::new();
    let mut dst = Vec::new();
    let endpoint = format!("{}/{}/check:publish?key={}", url, api_ver, key);
    easy.url(&endpoint)?;
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                                dst.extend_from_slice(data);
                                Ok(data.len())
                            })?;
        transfer.perform()?;
    }
    Ok(easy.response_code()?)
}

fn main() {
    run().unwrap();
}