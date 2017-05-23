use ringer::models::Session;
use serde_json;
use pencil::{Request, Response, PencilResult, PencilError, HTTPError};

pub fn before_each_request(r: &mut Request) -> Option<PencilResult> {
    if let Some(session_id) = r.args().get::<String>("session_id") {
        match Session::get_by_ext_id(session_id) {
            Ok(session) => {
                if session.is_valid() {
                    None
                } else {
                    session.delete().unwrap();
                    Some(Err(PencilError::PenHTTPError(HTTPError::Forbidden)))
                }
            }
            Err(_) => Some(Err(PencilError::PenHTTPError(HTTPError::InternalServerError))),
        }

    } else {
        Some(Err(PencilError::PenHTTPError(HTTPError::BadRequest)))
    }
}

pub fn validate(r: &mut Request) -> PencilResult {
    if let Some(session_id) = r.args().get::<String>("session_id") {
        match Session::get_by_ext_id(session_id) {
            Ok(session) => {
                if session.is_valid() {
                    Ok(Response::from(serde_json::to_string(&json!({"session_id": session_id, "valid": true}))
                               .unwrap()))
                } else {
                    session.delete().unwrap();
                    Err(PencilError::PenHTTPError(HTTPError::Forbidden))
                }
            }
            Err(_) => Err(PencilError::PenHTTPError(HTTPError::Forbidden)),
        }
    } else {
        Err(PencilError::PenHTTPError(HTTPError::BadRequest))
    }
}