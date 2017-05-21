use ringer::error::Result;
use ringer::models::{Session, User, NewUser};
use ringer::utils::process_pass;
use pencil::{Request, Response, PencilResult};
use serde_json;

fn newuser_from_request(r: &mut Request) -> Result<NewUser> {
    Ok(if let Some(ref value) = *r.get_json() {
           if let Some(obj) = value.as_object() {
               NewUser {
                   email: if let Some(email) = obj.get("email") {
                       if let Some(x) = email.as_str() {
                           String::from(x)
                       } else {
                           bail!("invalid email")
                       }
                   } else {
                       bail!("email is mandatory!")
                   },
                   pass: if let Some(rate) = obj.get("rate") {
                       if let Some(x) = rate.as_str() {
                           process_pass(x)?
                       } else {
                           bail!("invalid password")
                       }
                   } else {
                       bail!("password is mandatory!")
                   },
               }
           } else {
               bail!("data must be wrapped in a JSON object")
           }
       } else {
           bail!("no json data found")
       })
}

pub fn login(r: &mut Request) -> PencilResult {
    let foff = Ok(Response::from("invalid credentials"));
    if let Some(email) = r.args().get::<String>("email") {
        match User::get_by_email(email) {
            Ok(user) => {
                if let Some(pass) = r.args().get::<String>("pass") {
                    if user.pass == process_pass(pass).unwrap_or_default() {
                        match Session::return_fresh_id() {
                            Ok(id) => Ok(Response::from(id)),
                            Err(e) => {
                                Ok(Response::from(serde_json::to_string(e.description()).unwrap()))
                            }
                        }
                    } else {
                        foff
                    }
                } else {
                    foff
                }
            }
            Err(_) => foff,
        }
    } else {
        foff
    }
}

pub fn register(r: &mut Request) -> PencilResult {
    match newuser_from_request(r) {
        Ok(newuser) => {
            if newuser.insert_if_email_not_exists() {
                Ok(Response::from(serde_json::to_string(&json!({"status": 200})).unwrap()))
            } else {
                Ok(Response::from(serde_json::to_string(&json!({"status": 400})).unwrap()))
            }
        }
        Err(e) => Ok(Response::from(serde_json::to_string(
            &json!({"status": 400, "error": e.description()})).unwrap())),
    }
}