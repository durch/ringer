use std::io::Read;
use curl::easy::Easy;
use dotenv::dotenv;
use ringer::error::Result;
use ringer::models::{Check, NewCheck};
use pencil::{Request, Response, PencilResult, PencilError, HTTPError};
use serde_json;
use std::env;

use std::error::Error;

use url::Url;

fn format_sse(payload: &str) -> String {
    format!("event: message\ndata: {}\n", payload)
}

pub fn list(_: &mut Request) -> PencilResult {
    match Check::get_all(Some(20)) {
        Ok(checks) => {
            match Check::for_serde(checks) {
                Ok(ref serde_checks) => {
                    Ok(Response::from(serde_json::to_string(serde_checks).unwrap()))
                }
                Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
            }
        }
        Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),

    }
}

fn _publish(data: &str) -> Result<u32> {
    dotenv().ok();

    let esper_url = env::var("ESPER_URL").expect("ESPER_URL must be set");
    let mut easy = Easy::new();
    let mut data = data.as_bytes();
    easy.url(&esper_url)?;
    easy.post(true)?;
    easy.post_field_size(data.len() as u64)?;
    {
        let mut transfer = easy.transfer();
        transfer
            .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
            .unwrap();
        transfer.perform().unwrap();
    }
    Ok(easy.response_code()?)
}

pub fn publish(_: &mut Request) -> PencilResult {
    match Check::get_all(Some(20)) {
        Ok(checks) => {
            match Check::for_serde(checks) {
                Ok(ref serde_checks) => {
                    let payload = serde_json::to_string(serde_checks).unwrap();
                    match _publish(&format_sse(&payload)) {
                        Ok(code) => Ok(Response::from(serde_json::to_string(&json!({"code": code, "status": "published"}))
                                   .unwrap())),
                        Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
                    }
                }
                Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
            }
        }
        Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
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

pub fn add(r: &mut Request) -> PencilResult {
    match newcheck_from_request(r) {
        Ok(newcheck) => {
            match newcheck.insert_if_url_not_exists() {
                Ok(mut check) => {
                    check.perform().unwrap();
                    Ok(Response::from(serde_json::to_string(&json!({"id": check.id, "status": 200}))
                                  .unwrap()))
                }
                Err(_) => Err(PencilError::PenHTTPError(HTTPError::BadRequest)),
            }
        }
        Err(_) => Err(PencilError::PenHTTPError(HTTPError::BadRequest)),
    }
}

pub fn delete(r: &mut Request) -> PencilResult {
    if let Some(id) = r.args().get("id") {
        let id: &str = id;
        match Check::get(id.parse().unwrap_or(-1)) {
            Ok(check) => {
                match check.delete() {
                    Ok(_) => Ok(Response::from("Ok")),
                    Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
                }
            }
            Err(_) => Err(PencilError::PenHTTPError(HTTPError::BadRequest))
        }
    } else {
        Err(PencilError::PenHTTPError(HTTPError::BadRequest))
    }
}

pub fn find(r: &mut Request) -> PencilResult {
    if let Some(query) = r.args().get("query") {
        let query: &str = query;
        match Check::get_ilike(Some(20), String::from(query)) {
               Ok(checks) => {
                   match Check::for_serde(checks) {
                       Ok(ref serde_checks) => {
                           Ok(Response::from(serde_json::to_string(serde_checks).unwrap()))
                       }
                       Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
                   }
               }
               Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),

           }
    } else {
        Err(PencilError::PenHTTPError(HTTPError::BadRequest))
    }
}

pub fn run(r: &mut Request) -> PencilResult {
    if let Some(id) = r.args().get("id") {
        let id: &str = id;
        match Check::get(id.parse().unwrap_or(-1)) {
            Ok(mut check) => {
                match check.perform() {
                    Ok(_) => Ok(Response::from("Ok")),
                    Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
                }
            }
            Err(_) => Err(PencilError::PenHTTPError(HTTPError::InternalServerError)),
        }
    } else {
       Err(PencilError::PenHTTPError(HTTPError::BadRequest))
    }
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