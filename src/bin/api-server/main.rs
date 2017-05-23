extern crate sharp_pencil as pencil;
extern crate ringer;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate error_chain;
extern crate url;
extern crate curl;
extern crate dotenv;
extern crate chrono;

use pencil::Pencil;

mod check;
mod user;
mod session;

// Requires MASTER_KEY, ESPER_URL and DATABASE_URL

fn main() {
    let mut app = Pencil::new("/");
    app.set_debug(true);
    app.get("/v0/check:list", "check:list", check::list);
    app.put("/v0/check:add", "check:add", check::add);
    app.get("/v0/check:run", "check:run", check::run);
    app.delete("/v0/check:delete", "check:delete", check::delete);
    app.get("/v0/check:publish", "check:publish", check::publish);
    app.get("/v0/session:validate", "session:validate", session::validate);
    app.post("/v0/user:login", "user:login", user::login);
    app.post("/v0/user:register", "user:register", user::register);
    app.before_request(session::before_each_request);
    app.run("0.0.0.0:5000");
}
