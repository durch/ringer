use chrono::NaiveDateTime;
use super::schema::checks;
use diesel;
use diesel::prelude::*;
use diesel::SaveChangesDsl;
use error::Result;

use utils::establish_connection;
use curl::easy::Easy;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SerdeCheck {
    pub id: i32,
    pub url: String,
    pub rate: i32,
    pub last_start: Option<String>,
    pub last_end: Option<String>,
    pub http_status: Option<i32>,
    pub state: Option<String>,
}


impl<'a> From<&'a Check> for SerdeCheck {
    fn from(c: &Check) -> Self {
        SerdeCheck {
            id: c.id,
            url: c.url.clone(),
            rate: c.rate,
            last_start: match c.last_start {
                Some(x) => Some(format!("{}", x)),
                None => None,
            },
            last_end: match c.last_end {
                Some(x) => Some(format!("{}", x)),
                None => None,
            },
            http_status: c.http_status,
            state: c.state.clone(),
        }
    }
}

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
    pub fn exists(url: &str) -> Result<Self> {
        Ok(checks::table
               .filter(checks::url.eq(url))
               .get_result(&establish_connection())?)
    }

    pub fn get_all(limit: Option<i64>) -> Result<Vec<Self>> {
        match limit {
            Some(l) => Ok(checks::table.limit(l).load(&establish_connection())?),
            None => Ok(checks::table.load(&establish_connection())?),
        }
    }

    pub fn all_for_serde(limit: Option<i64>) -> Result<Vec<SerdeCheck>> {
        Ok(Check::get_all(limit)?.iter().map(|x| x.into()).collect())
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

    pub fn conditional_perform(&mut self) -> Result<bool> {
        // println!("{}", self.duration_since_last_end());
        if self.rate <= self.duration_since_last_end() {
            self.perform()?;
            Ok(true)
        } else {
            Ok(false)
        }
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
        self.u_last_end(UTC::now().naive_utc())?;
        self.u_state(String::from_utf8(dst)?)?;
        self.u_http_status(easy.response_code()?)?;
        Ok(())
    }

    pub fn valid(&self) -> bool {
        self.last_start < self.last_end
    }

    pub fn duration_since_last_end(&self) -> i32 {
        match self.last_end {
            Some(x) => {
                UTC::now()
                    .naive_utc()
                    .signed_duration_since(x)
                    .num_seconds() as i32
            }
            None => ::std::i32::MAX,
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

#[derive(Insertable, Deserialize)]
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

    pub fn insert_if_url_not_exists(&self) -> Check {
        match Check::exists(&self.url) {
            Ok(check) => check,
            Err(_) => self.insert(),
        }
    }
}
