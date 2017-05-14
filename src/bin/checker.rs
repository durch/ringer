extern crate ringer;

use std::{thread, time};
use ringer::models::{Check, NewCheck};
use ringer::error::Result;

fn min_rate(checks: &[Check]) -> i32 {
    checks
        .iter()
        .fold(60, |s, x| if s < x.rate { s } else { x.rate })
}

fn run() -> Result<()> {
    let mut checks;
    let mut idle_time;
    loop {
        checks = Check::get_all(None)?;
        idle_time = min_rate(&checks[..]);
        for mut check in checks {
            let _ = check.conditional_perform();
        }
        thread::sleep(time::Duration::from_secs(idle_time as u64))
    }
}

fn main() {
    let _ = run();
}