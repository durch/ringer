extern crate sharp_pencil as pencil;
extern crate ringer;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate error_chain;
extern crate url;
extern crate curl;
extern crate dotenv;

use dotenv::dotenv;
use pencil::{Pencil, Request, Response, PencilResult};

use std::env;

mod check;
mod user;
mod session;

// Requires MASTER_KEY, ESPER_URL and DATABASE_URL
fn key_auth(r: &mut Request) -> Option<PencilResult> {
    dotenv().ok();

    let master = env::var("MASTER_KEY").expect("MASTER_KEY must be set");
    let unauth = Some(Ok(Response::from(serde_json::to_string(&json!({"code": 401, "status": "Unauthorized"})).unwrap())));
    if let Some(key) = r.args().get("key") {
        if key == master { None } else { unauth }
    } else {
        unauth
    }
}

fn main() {
    let mut app = Pencil::new("/");
    app.set_debug(true);
    app.get("/v0/check:list", "check:list", check::list);
    app.put("/v0/check:add", "check:add", check::add);
    app.get("/v0/check:run", "check:run", check::run);
    app.delete("/v0/check:delete", "check:delete", check::delete);
    app.get("/v0/check:publish", "check:publish", check::publish);
    app.get("/v0/session:validate", "session:validate", session::validate);
    app.get("/v0/user:login", "user:login", user::login);
    app.get("/v0/user:register", "user:register", user::register);
    // app.before_request(key_auth);
    app.run("0.0.0.0:5000");
}
