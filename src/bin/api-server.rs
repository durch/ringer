extern crate sharp_pencil as pencil;
extern crate ringer;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate error_chain;
extern crate url;

use ringer::error::Result;
use ringer::models::{Check, NewCheck};
use pencil::{Pencil, Request, Response, PencilResult};

use url::Url;

fn check_list(_: &mut Request) -> PencilResult {
    Ok(match Check::get_all(Some(20)) {
               Ok(checks) => {
                   match Check::for_serde(checks) {
                       Ok(ref serde_checks) => {
                           Response::from(serde_json::to_string(serde_checks).unwrap())
                       }
                       Err(ref e) => {
                           Response::from(serde_json::to_string(e.description()).unwrap())
                       }
                   }
               }
               Err(ref e) => Response::from(serde_json::to_string(e.description()).unwrap()),

           })
}

fn validate_rate(rate: i64) -> Result<i32> {
    if rate >= 60 {
        Ok(rate as i32)
    } else {
        bail!("rate must be greater than 60 seconds")
    }
}

fn validate_url(url: &str) -> Result<String> {
    match Url::parse(url) {
        Ok(_) => Ok(String::from(url)),
        Err(_) => bail!("invalid url"),
    }
}


fn newcheck_from_request(r: &mut Request) -> Result<NewCheck> {
    Ok(if let Some(ref value) = *r.get_json() {
           if let Some(obj) = value.as_object() {
               NewCheck {
                   url: if let Some(url) = obj.get("url") {
                       if let Some(x) = url.as_str() {
                           validate_url(x)?
                       } else {
                           bail!("url needs to be a JSON string!")
                       }
                   } else {
                       bail!("url is mandatory!")
                   },
                   rate: if let Some(rate) = obj.get("rate") {
                       if let Some(x) = rate.as_str() {
                           validate_rate(x.parse()?)?
                       } else {
                           bail!("rate needs to be an integer!")
                       }
                   } else {
                       bail!("rate is mandatory!")
                   },
               }
           } else {
               bail!("data must be wrapped in a JSON object")
           }
       } else {
           bail!("no json data found")
       })
}

fn check_add(r: &mut Request) -> PencilResult {
    match newcheck_from_request(r) {
        Ok(newcheck) => {
            match newcheck.insert_if_url_not_exists() {
                Ok(check) => Ok(Response::from(serde_json::to_string(&json!({"id": check.id, "status": 200}))
                                  .unwrap())),
                Err(e) => Ok(Response::from(serde_json::to_string(
            &json!({"status": 400, "error": e.description()})).unwrap())),
            }
        }
        Err(e) => Ok(Response::from(serde_json::to_string(
            &json!({"status": 400, "error": e.description()})).unwrap())),
    }
}

fn check_delete(r: &mut Request) -> PencilResult {
    if let Some(id) = r.args().get("id") {
        let id: &str = id;
        match Check::get(id.parse().unwrap_or(-1)) {
            Ok(check) => {
                match check.delete() {
                    Ok(_) => Ok(Response::from("Ok")),
                    Err(e) => Ok(Response::from(e.description())),
                }
            }
            Err(e) => Ok(Response::from(e.description())),
        }
    } else {
        Ok(Response::from("id cannot be empty!"))
    }
}

fn check_find(r: &mut Request) -> PencilResult {
    if let Some(query) = r.args().get("query") {
        let query: &str = query;
        Ok(match Check::get_ilike(Some(20), String::from(query)) {
               Ok(checks) => {
                   match Check::for_serde(checks) {
                       Ok(ref serde_checks) => {
                           Response::from(serde_json::to_string(serde_checks).unwrap())
                       }
                       Err(ref e) => {
                           Response::from(serde_json::to_string(e.description()).unwrap())
                       }
                   }
               }
               Err(ref e) => Response::from(serde_json::to_string(e.description()).unwrap()),

           })
    } else {
        Ok(Response::from("query cannot be empty!"))
    }
}

fn check_run(r: &mut Request) -> PencilResult {
    if let Some(id) = r.args().get("id") {
        let id: &str = id;
        match Check::get(id.parse().unwrap_or(-1)) {
            Ok(mut check) => {
                match check.perform() {
                    Ok(_) => Ok(Response::from("Ok")),
                    Err(e) => Ok(Response::from(e.description())),
                }
            }
            Err(e) => Ok(Response::from(e.description())),
        }
    } else {
        Ok(Response::from("id cannot be empty!"))
    }
}

fn main() {
    let mut app = Pencil::new("/check:list");
    app.set_debug(true);
    app.get("/v0/check:list", "check:list", check_list);
    app.put("/v0/check:add", "check:add", check_add);
    app.get("/v0/check:run", "check:run", check_run);
    app.delete("/v0/check:delete", "check:delete", check_delete);
    app.run("0.0.0.0:5000");
}
