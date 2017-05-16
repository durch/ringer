#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate curl;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate sharp_pencil as pencil;
extern crate futures;
extern crate chrono_humanize;
extern crate url;

pub mod models;
pub mod schema;
pub mod utils;

pub mod error {
    error_chain! {
        foreign_links {
            Diesel(::diesel::result::Error);
            Curl(::curl::Error);
            FromUtf8(::std::string::FromUtf8Error);
            Json(::serde_json::Error);
        }
    }
}

#[cfg(test)]
mod tests {
    use models::{Check, NewCheck};

    #[test]
    fn test_mattermost() {
        use utils::{Attachment, Message, mattermost};

        let attachment = Attachment {
            fallback: "test",
            color: "#0073CF",
            pretext: "",
            text: "Test message text",
            title: "Test message title",
            title_link: "",
            fields: Vec::new(),
            image_url: "",
        };
        let message = Message { attachments: vec![attachment] };
        let status_code = mattermost(&message).unwrap_or(400);
        assert!(status_code == 200);
    }

    #[test]
    fn test_perform_check() {
        let new_check = NewCheck {
            url: String::from("https://www.rust-lang.org/"),
            rate: 60,
        };

        let mut check = new_check.insert();

        let _ = check.perform();
        let _ = check.delete();
    }

    #[test]
    fn test_dsl() {
        let check = NewCheck {
            url: String::from("google.com"),
            rate: 60,
        };

        let inserted = check.insert();

        let mut selected = Check::get(inserted.id).unwrap();

        assert_eq!(inserted, selected);

        let updated = selected.u_state(String::from("updated")).unwrap();

        let updated_in_db = Check::get(inserted.id).unwrap();

        assert_ne!(inserted, updated);
        assert_eq!(updated, updated_in_db);
        assert_eq!(updated.state, Some(String::from("updated")));

        let affected = updated.delete();
        assert_eq!(affected.unwrap(), 1);

        match Check::get(inserted.id) {
            Ok(_) => unreachable!(),
            Err(_) => assert!(true),
        };
    }
}
