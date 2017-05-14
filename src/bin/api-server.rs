extern crate sharp_pencil as pencil;
extern crate ringer;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate error_chain;

use ringer::error::Result;
use ringer::models::{Check, NewCheck};
use pencil::{Pencil, Request, Response, PencilResult};

fn check_list(_: &mut Request) -> PencilResult {

    Ok(match Check::all_for_serde(Some(20)) {
           Ok(ref serde_checks) => Response::from(serde_json::to_string(serde_checks).unwrap()),
           Err(ref e) => Response::from(serde_json::to_string(e.description()).unwrap()),
       })
}

fn newcheck_from_request(r: &mut Request) -> Result<NewCheck> {
    Ok(match *r.get_json() {
        Some(ref value) => {
            if let Some(obj) = value.as_object() {
                NewCheck {
                    url: match obj.get("url") {
                        Some(url) => {
                            if url.is_string() {
                                String::from(url.as_str().unwrap())
                            } else {
                                bail!("url needs to ne string!")
                            }
                        }
                        None => bail!("url is mandatory!"), 
                    },
                    rate: match obj.get("rate") {
                        Some(rate) => {
                            if rate.is_number() {
                                rate.as_i64().unwrap() as i32
                            } else {
                                bail!("rate needs to ne integer!")
                            }
                        }
                        None => bail!("rate rate is mandatory!"),
                    },
                }
            } else {
                bail!("data must be wrapped in a JSON object")
            }
        }
        None => bail!("no json data found"),
    })
}

fn check_add(r: &mut Request) -> PencilResult {
    match newcheck_from_request(r) {
        Ok(newcheck) => {
            let check = newcheck.insert_if_url_not_exists();
            Ok(Response::from(serde_json::to_string(&json!({"id": check.id, "status": 200}))
                                  .unwrap()))

        }
        Err(e) => Ok(Response::from(serde_json::to_string(
            &json!({"status": 400, "error": e.description()})).unwrap())),
    }

}


fn main() {
    let mut app = Pencil::new("/check:list");
    app.set_debug(true);
    app.get("/", "check:list", check_list);
    app.put("/check:add", "check:add", check_add);
    app.run("127.0.0.1:5000");
}
