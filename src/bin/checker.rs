extern crate ringer;
extern crate chrono;

use std::{thread, time};
use ringer::models::{Check};
use ringer::error::Result;

use chrono::UTC;

fn min_rate(checks: &[Check]) -> i32 {
    checks
        .iter()
        .fold(60, |s, x| if s < x.rate { s } else { x.rate })
}

fn run() -> Result<()> {
    let mut checks;
    let mut idle_time;
    loop {
        let mut n = 0;
        checks = Check::get_all(None)?;
        let l = checks.len();
        idle_time = min_rate(&checks[..]);
        for mut check in checks {
            n += 1;
            let _ = check.conditional_perform();
        }
        println!("{} - Performed {}/{} checks", UTC::now(), n, l);
        thread::sleep(time::Duration::from_secs(idle_time as u64))
    }
}

fn main() {
    let _ = run();
}