use ringer::models::Session;
use serde_json;
use pencil::{Request, Response, PencilResult};

pub fn validate(r: &mut Request) -> PencilResult {
    if let Some(session_id) = r.args().get::<String>("session_id") {
        Ok(match Session::get_by_ext_id(session_id) {
               Ok(session) => {
                   if session.is_valid() {
            Response::from(serde_json::to_string(&json!({"session_id": session_id, "valid": true}))
                               .unwrap())
        } else {
            session.delete().unwrap();
            Response::from(serde_json::to_string(&json!({"session_id": session_id, "valid": false})).unwrap())
        }
               }
               Err(e) => Response::from(serde_json::to_string(e.description()).unwrap()),
           })

    } else {
        Ok(Response::from("session_id is required"))
    }
}