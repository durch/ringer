use ringer::error::Result;
use ringer::models::{Session, User, NewUser};
use ringer::utils::verify_pass;
use pencil::{Request, Response, PencilResult, PencilError, HTTPError};
use chrono::prelude::*;
use rand::{thread_rng, Rng};
use std::io::{self, Write};
use super::INVITE_CODES;


pub fn generate_invite_codes(n: usize) -> Result<()> {
    let mut codes = INVITE_CODES.write().unwrap();
    let mut rng = thread_rng();
    for _ in 0..n {
        let s = rng.gen_ascii_chars().take(10).collect::<String>();
        let _ = io::stdout().write(s.as_bytes());
        let _ = io::stdout().write(b"\n");
        codes.push(s);
    }
    Ok(())
}

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
                   pass: if let Some(rate) = obj.get("pass") {
                       if let Some(x) = rate.as_str() {
                           String::from(x)
                       } else {
                           bail!("invalid password")
                       }
                   } else {
                       bail!("password is mandatory!")
                   },
                   created: UTC::now().naive_utc(),
               }
           } else {
               bail!("data must be wrapped in a JSON object")
           }
       } else {
           bail!("no json data found")
       })
}

pub fn login(r: &mut Request) -> PencilResult {
    match newuser_from_request(r) {
        Ok(newuser) => {
            match User::exists(&newuser.email) {
                Ok(user) => {
                    if verify_pass(&newuser.pass, &user.pass) {
                        match Session::return_fresh_id() {
                            Ok(id) => Ok(Response::from(id)),
                            Err(_) => Err(PencilError::PenHTTPError(HTTPError::Forbidden)),
                        }
                    } else {
                        Err(PencilError::PenHTTPError(HTTPError::Forbidden))
                    }
                }
                Err(_) => Err(PencilError::PenHTTPError(HTTPError::Unauthorized)),
            }
        }
        Err(_) => Err(PencilError::PenHTTPError(HTTPError::UnprocessableEntity)),
    }
}


pub fn register(r: &mut Request) -> PencilResult {
    match newuser_from_request(r) {
        Ok(mut newuser) => {
            match User::exists(&newuser.email) {
                // If a user exists, we return error, since this is register
                Ok(_) => Err(PencilError::PenHTTPError(HTTPError::Conflict)),
                Err(_) => {
                    if let Some(invite_code) = r.args().get::<String>("invite_code") {
                        let codes = INVITE_CODES.read().unwrap();
                        match codes.binary_search(invite_code) {
                            Ok(i) => {
                                let mut w_codes = INVITE_CODES.write().unwrap();
                                w_codes.swap_remove(i);
                            }
                            Err(_) => return Err(PencilError::PenHTTPError(HTTPError::Forbidden)),
                        }
                    }
                    newuser.insert().unwrap();
                    Ok(Response::from("Ok"))
                }
            }
        }
        Err(_) => Err(PencilError::PenHTTPError(HTTPError::UnprocessableEntity)),
    }
}