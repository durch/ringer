#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate error_chain;

pub mod models;
pub mod schema;
pub mod utils;

mod fdw_error {
    error_chain! {
        foreign_links {
            Diesel(::diesel::result::Error);
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use models::{Check, NewCheck};
    use utils::establish_connection;

    #[test]
    fn test_dsl() {
        let connection = establish_connection();
        let check = NewCheck { url: "google.com", rate: 60 };
        let inserted = check.insert(&connection);

        let mut selected = Check::get(inserted.id, &connection).unwrap();

        assert_eq!(inserted, selected);

        let updated = selected.update_state(String::from("updated"), &connection).unwrap();

        let updated_in_db = Check::get(inserted.id, &connection).unwrap();

        assert_ne!(inserted, updated);
        assert_eq!(updated, updated_in_db);
        assert_eq!(updated.state, Some(String::from("updated")));

        let affected = updated.delete(&connection);
        assert_eq!(affected.unwrap(), 1);

        match Check::get(inserted.id, &connection) {
            Ok(_) => unreachable!(),
            Err(_) => assert!(true)
        };
    }
}

