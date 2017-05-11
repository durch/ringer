use chrono::NaiveDateTime;
use super::schema::checks;
use diesel;
use diesel::prelude::*;
use diesel::SaveChangesDsl;
use fdw_error::Result;

use utils::establish_connection;
use curl::easy::Easy;
use chrono::prelude::*;

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq, AsChangeset)]
pub struct Check {
    pub id: i32,
    pub url: String,
    pub rate: i32,
    pub last_start: Option<NaiveDateTime>,
    pub last_end: Option<NaiveDateTime>,
    pub http_status: Option<i32>,
    pub state: Option<String>,
}


impl Check {
    pub fn get_all() -> Result<Vec<Self>> {
        Ok(checks::table.load(&establish_connection())?)
    }

    pub fn u_state(&mut self, new_state: String) -> Result<Self> {
        self.state = Some(new_state);
        Ok(self.save_changes::<Self>(&establish_connection())?)
    }

    pub fn u_http_status(&mut self, status: u32) -> Result<Self> {
        self.http_status = Some(status as i32);
        Ok(self.save_changes::<Self>(&establish_connection())?)
    }

    pub fn u_last_start(&mut self, time: NaiveDateTime) -> Result<Self> {
        self.last_start = Some(time);
        Ok(self.save_changes::<Self>(&establish_connection())?)
    }

    pub fn u_last_end(&mut self, time: NaiveDateTime) -> Result<Self> {
        self.last_end = Some(time);
        Ok(self.save_changes::<Self>(&establish_connection())?)
    }

    //    Returns number of rows affected
    pub fn delete(&self) -> Result<usize> {
        Ok(diesel::delete(self).execute(&establish_connection())?)
    }

    pub fn get(id: i32) -> Result<Self> {
        Ok(checks::table
               .filter(checks::id.eq(id))
               .first::<Self>(&establish_connection())?)
    }

    pub fn conditional_perform(&mut self) -> Result<()> {
        if self.rate <= self.duration_since_last_end() {
            println!("{} - Running check : {}", UTC::now(), self.url);
            self.perform()?
        }
        Ok(())
    }

    pub fn perform(&mut self) -> Result<()> {
        let mut easy = Easy::new();
        let mut dst = Vec::new();
        easy.url(&self.url)?;
        let _ = self.u_last_start(UTC::now().naive_utc());
        {
            let mut transfer = easy.transfer();
            transfer
                .write_function(|data| {
                                    dst.extend_from_slice(data);
                                    Ok(data.len())
                                })?;
            transfer.perform()?;
        }
        let _ = self.u_last_end(UTC::now().naive_utc());
        let _ = self.u_state(String::from_utf8(dst)?);
        let _ = self.u_http_status(easy.response_code()?);
        return Ok(());
    }

    pub fn valid(&self) -> bool {
        self.last_start < self.last_end
    }

    pub fn duration_since_last_end(&self) -> i32 {
        match self.last_end {
            Some(x) => UTC::now().naive_utc().signed_duration_since(x).num_seconds() as i32,
            None => ::std::i32::MAX
        }
        
    }
}

impl From<NewCheck> for Check {
    fn from(newcheck: NewCheck) -> Self {
        Check {
            id: 0,
            url: newcheck.url,
            rate: newcheck.rate,
            state: None,
            http_status: None,
            last_start: None,
            last_end: None,
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

        diesel::insert(self)
            .into(checks::table)
            .get_result(&establish_connection())
            .expect("Error saving new check")
    }
}
