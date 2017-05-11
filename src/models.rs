use chrono::NaiveDateTime;
use super::schema::checks;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::SaveChangesDsl;
use fdw_error::Result;

use utils::establish_connection;

use std::io::{stdout, Write};
use curl::easy::Easy;

use chrono::prelude::*;

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq, AsChangeset)]
pub struct Check {
    pub id: i32,
    pub url: String,
    pub rate: i32,
    pub last_checked: Option<NaiveDateTime>,
    pub state: Option<String>,
    pub check_finished: Option<NaiveDateTime>,
}

impl Check {
    pub fn update_state(&mut self, new_state: String) -> Result<Self> {
        let connection = establish_connection();
        self.state = Some(new_state);
        Ok(self.save_changes::<Self>(&connection)?)
    }

    pub fn update_last_checked(&mut self, time: NaiveDateTime) -> Result<Self> {
        let connection = establish_connection();
        self.last_checked = Some(time);
        Ok(self.save_changes::<Self>(&connection)?)
    }

    pub fn update_check_finished(&mut self, time: NaiveDateTime) -> Result<Self> {
        let connection = establish_connection();
        self.check_finished = Some(time);
        Ok(self.save_changes::<Self>(&connection)?)
    }

    //    Returns number of rows affected
    pub fn delete(&self) -> Result<usize> {
        let connection = establish_connection();
        Ok(diesel::delete(self).execute(&connection)?)
    }


    pub fn get(id: i32) -> Result<Self> {
        let connection = establish_connection();
        Ok(checks::table
               .filter(checks::id.eq(id))
               .first::<Self>(&connection)?)
    }

    pub fn perform(&mut self) -> Result<()> {
        let mut easy = Easy::new();
        let mut dst = Vec::new();

        easy.url(&self.url)?;
        let _ = self.update_last_checked(UTC::now().naive_utc());

        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                                    dst.extend_from_slice(data);
                                    Ok(data.len())
                                })?;
            transfer.perform()?;
        }

        let _ = self.update_check_finished(UTC::now().naive_utc());

        let _ = self.update_state(String::from_utf8(dst)?);

        return Ok(());
    }
}

impl From<NewCheck> for Check {
    fn from(newcheck: NewCheck) -> Self {
        Check {
            id: 0,
            url: newcheck.url,
            rate: newcheck.rate,
            state: None,
            last_checked: None,
            check_finished: None,
        }
    }
}

#[derive(Insertable)]
#[table_name = "checks"]
pub struct NewCheck {
    pub url: String,
    pub rate: i32,
}

impl NewCheck {
    pub fn insert(&self) -> Check {
        use schema::checks;

        let connection = establish_connection();

        diesel::insert(self)
            .into(checks::table)
            .get_result(&connection)
            .expect("Error saving new post")
    }
}
