extern crate ringer;
extern crate chrono;
extern crate curl;
extern crate futures;
extern crate futures_cpupool;

use futures::future;
use futures_cpupool::CpuPool;

use std::{thread, time};
use ringer::models::Check;
use ringer::error::{Result, Error};
use ringer::utils::alert_on_error_code;

use chrono::UTC;


fn min_rate(checks: &[Check]) -> i32 {
    checks
        .iter()
        .fold(60, |s, x| if s < x.rate { s } else { x.rate })
}

fn async_check<F>(check: &mut Check, funs: &[F]) -> future::FutureResult<(), Error> 
    where F: Fn(&mut Check) -> Result<()> {
    if check.conditional_perform().unwrap() {
        for fun in funs {
            match fun(check) {
                Ok(_) => {}
                Err(e) => return future::err(e)
            }
        }
    };
    future::ok(())
}

fn run<F>(funs: &[F]) -> Result<()> 
    where F: Fn(&mut Check) -> Result<()> {
    let cpu_pool = CpuPool::new(10);
    let mut checks;
    let mut idle_time;
    loop {
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
        println!("{} - Performing {}/{} checks", UTC::now(), futures.len(), l);
        thread::sleep(time::Duration::from_secs(idle_time as u64));
    }
}

fn main() {
    run(&[alert_on_error_code]).unwrap();
}