use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

use serde_json;
use error::Result;

use curl::easy::{Easy, List};
use models::Check;

use std::io::Read;

#[derive(Debug, Serialize)]
pub struct Message<'a> {
    pub attachments: Vec<Attachment<'a>>,
}

#[derive(Debug, Serialize)]
pub struct Attachment<'a> {
    pub fallback: &'a str,
    pub color: &'a str,
    pub pretext: &'a str,
    pub text: &'a str,
    pub title: &'a str,
    pub title_link: &'a str,
    pub image_url: &'a str,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub short: bool,
    pub title: String,
    pub value: String,
}

pub fn mattermost(message: &Message) -> Result<u32> {
    dotenv().ok();

    let mattermost_url = env::var("MATTERMOST_URL").expect("MATTERMOST_URL must be set");
    let mattermost_hook = env::var("MATTERMOST_HOOK").expect("MATTERMOST_HOOK must be set");

    let endpoint = format!("{}/hooks/{}", mattermost_url, mattermost_hook);

    let msg_string = serde_json::to_string(message)?;
    let mut data = msg_string.as_bytes();

    let mut list = List::new();
    list.append("Content-type: application/json").unwrap();

    let mut easy = Easy::new();
    easy.url(&endpoint)?;
    easy.post(true)?;
    easy.post_field_size(data.len() as u64)?;
    easy.http_headers(list)?;

    {
        let mut transfer = easy.transfer();
        transfer
            .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
            .unwrap();
        transfer.perform().unwrap();
    }
    Ok(easy.response_code()?)
}

pub fn alert_on_error_code(check: &mut Check) -> Result<()> {
    if check.http_status.unwrap_or(418) > 400 {
        let attachment = Attachment {
            fallback: &format!("{}, {:?} - {}",
                              check.url,
                              check.last_end,
                              check.http_status.unwrap_or(418)),
            color: "#DC143C",
            pretext: "",
            text: &format!("{:?} - code: {}",
                          check.last_end,
                          check.http_status.unwrap_or(418)),
            title: &check.url,
            title_link: &check.url,
            fields: Vec::new(),
            image_url: "",
        };
        let message = Message { attachments: vec![attachment] };
        let status_code = mattermost(&message)?;
        print!("{}", status_code);
    }

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
