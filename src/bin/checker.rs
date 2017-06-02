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
use ringer::models::{Check, CheckMeta, User};
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
                _ => unreachable!(),
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
    let mut start;
    let mut duration: u64;
    let mut now;
    let mut ticks: u64;
    let idle_time = 60;
    loop {
        let users = User::get_all()?;
        let mut publish_interval = 10;
        start = UTC::now().naive_utc();
        for user in &users {
            checks = Check::get_all(None, Some(user.id))?;
            let l = checks.len();
            let mut futures = Vec::new();
            // Test if this is truly async :)
            for mut check in checks {
                let future = cpu_pool.spawn(async_check(&mut check));
                futures.push(future);
            }
            now = UTC::now().naive_utc();
            println!("{} - Performing {}/{} checks for user {}", now, futures.len(), l, user.id);
            trigger_sse(user.id)?;
        }
        ticks = idle_time / publish_interval;
        now = UTC::now().naive_utc();
        duration = now.signed_duration_since(start).num_seconds() as u64;
        if duration < publish_interval {
            publish_interval -= duration
        }
        for _ in 0..ticks {
            // println!("{}", UTC::now());
            // trigger_sse(users.as_slice())?;
            thread::sleep(time::Duration::from_secs(publish_interval));
        }
    }
}

fn trigger_sse(user_id: i32) -> Result<u32> {
    dotenv().ok();

    let url = env::var("APP_URL").expect("APP_URL must be set");
    let api_ver = env::var("API_VER").expect("API_VER must be set");
    let key = env::var("MASTER_KEY").expect("MASTER_KEY must be set");

    let mut easy = Easy::new();
    let mut dst = Vec::new();
    let endpoint = format!("{}/{}/check:publish?key={}?user_id={}", url, api_ver, key, user_id);
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