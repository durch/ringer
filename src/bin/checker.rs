extern crate ringer;
extern crate chrono;
extern crate curl;
extern crate futures;
extern crate futures_cpupool;
extern crate dotenv;
use std::env;

use futures::future;
use futures_cpupool::CpuPool;

use std::{thread, time};
use ringer::models::Check;
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

fn async_check<F>(check: &mut Check, funs: &[F]) -> future::FutureResult<(), Error>
    where F: Fn(&mut Check) -> Result<()>
{
    if check.conditional_perform().unwrap() {
        for fun in funs {
            match fun(check) {
                Ok(_) => {}
                Err(e) => return future::err(e),
            }
        }
    };
    future::ok(())
}

fn run<F>(funs: &[F]) -> Result<()>
    where F: Fn(&mut Check) -> Result<()>
{
    let cpu_pool = CpuPool::new(10);
    let publish_interval = 10;
    let mut checks;
    let mut idle_time: u64;
    let mut start;
    let mut duration: u64;
    let mut now;
    let mut interval: u64;
    loop {
        start = UTC::now().naive_utc();
        checks = Check::get_all(None)?;
        let l = checks.len();
        idle_time = min_rate(&checks[..]);
        let mut futures = Vec::new();
        // Test if this is truly async :)
        for mut check in checks {
            let future = cpu_pool.spawn(async_check(&mut check, funs));
            futures.push(future);
        }
        // for future in futures {
        //     future.wait()?;
        //     n += 1;
        // }
        interval = idle_time / publish_interval;
        now = UTC::now().naive_utc();
        println!("{} - Performing {}/{} checks", now, futures.len(), l);
        duration = now.signed_duration_since(start).num_seconds() as u64;
        if duration < interval {
            interval -= duration
        }
        for _ in 0..interval {
            println!("{}", UTC::now());
            trigger_sse()?;
            thread::sleep(time::Duration::from_secs(publish_interval));    
        }
        // trigger_sse()?;
        // thread::sleep(time::Duration::from_secs(idle_time as u64));
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
    run(&[alert_on_error_code]).unwrap();
}