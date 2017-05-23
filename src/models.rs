use chrono::NaiveDateTime;
use super::schema::*;
use diesel;
use diesel::prelude::*;
use diesel::SaveChangesDsl;
use error::Result;
use utils::{hash, process_pass};
use time::Duration;
use serde_json;

use utils::establish_connection;
use curl::easy::Easy;
use chrono::prelude::*;
use chrono_humanize::HumanTime;
use dotenv::dotenv;
use std::env;


#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub pass: String,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>,
}

impl User {
    pub fn get_by_email(email: &str) -> Result<Self> {
        Ok(users::table
               .filter(users::email.eq(email))
               .first(&establish_connection())?)
    }

    pub fn exists(email: &str) -> Result<Self> {
        Ok(users::table
               .filter(users::email.eq(email))
               .get_result(&establish_connection())?)
    }
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub pass: String,
    pub created: NaiveDateTime,
}

impl NewUser {
    pub fn insert(&mut self) -> Result<User> {
        self.pass = process_pass(&self.pass)?;
        Ok(diesel::insert(self)
               .into(users::table)
               .get_result(&establish_connection())
               .expect("Error saving new check"))
    }

    pub fn insert_if_email_not_exists(&mut self) -> Result<User> {
        match User::exists(&self.email) {
            Ok(user) => Ok(user),
            Err(_) => self.insert(),
        }
    }
}


#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct Session {
    pub id: i32,
    pub ext_id: String,
    pub valid_until: NaiveDateTime,
}

impl Session {
    pub fn get_by_ext_id(ext_id: &str) -> Result<Self> {
        Ok(sessions::table
               .filter(sessions::ext_id.eq(ext_id))
               .first(&establish_connection())?)
    }

    pub fn is_valid(&self) -> bool {
        UTC::now().naive_utc() < self.valid_until
    }

    pub fn delete(&self) -> Result<usize> {
        Ok(diesel::delete(self).execute(&establish_connection())?)
    }

    pub fn return_fresh_id() -> Result<String> {
        Ok(NewSession::new()?.insert()?.ext_id)
    }
}

#[derive(Debug, Insertable)]
#[table_name="sessions"]
pub struct NewSession {
    pub ext_id: String,
    pub valid_until: NaiveDateTime,
}

impl NewSession {
    pub fn new() -> Result<Self> {
        dotenv().ok();
        let pepper = env::var("PEPPER").expect("PEPPER must be set");
        let timestamp = UTC::now().naive_utc().timestamp();
        Ok(NewSession {
               ext_id: format!("x{}", hash(&format!("{}{}", timestamp, pepper))),
               valid_until: UTC::now().naive_utc() + Duration::hours(1),
           })
    }

    pub fn insert(&self) -> Result<Session> {
        Ok(diesel::insert(self)
               .into(sessions::table)
               .get_result(&establish_connection())
               .expect("Error saving new check"))
    }
}

#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct CheckRun {
    pub id: i32,
    pub check_id: i32,
    pub starttime: NaiveDateTime,
    pub endtime: NaiveDateTime,
    pub http_status: i32,
    pub latency: i32,
}

impl CheckRun {
    pub fn get_from_check_id(limit: Option<i64>, check_id: i32) -> Result<Vec<Self>> {
        match limit {
            Some(l) => {
                Ok(check_runs::table
                       .filter(check_runs::check_id.eq(check_id))
                       .limit(l)
                       .load(&establish_connection())?)
            }
            None => {
                Ok(check_runs::table
                       .filter(check_runs::check_id.eq(check_id))
                       .load(&establish_connection())?)
            }
        }
    }
}

#[derive(Insertable)]
#[table_name = "check_runs"]
pub struct NewCheckRun {
    pub check_id: i32,
    pub starttime: NaiveDateTime,
    pub endtime: NaiveDateTime,
    pub http_status: i32,
    pub latency: i32,
}


impl NewCheckRun {
    pub fn insert(&self) -> Result<CheckRun> {
        Ok(diesel::insert(self)
               .into(check_runs::table)
               .get_result(&establish_connection())
               .expect("Error saving new check"))
    }
}

impl<'a> From<&'a mut Check> for NewCheckRun {
    fn from(check: &'a mut Check) -> Self {
        let starttime = match check.last_start {
            Some(x) => x,
            None => panic!("last_start is not set, this should impossible"),
        };
        let endtime = match check.last_end {
            Some(x) => x,
            None => panic!("last_end is not set, this should impossible"),
        };

        NewCheckRun {
            check_id: check.id,
            starttime: starttime,
            endtime: endtime,
            http_status: match check.http_status {
                Some(x) => x,
                None => panic!("http_status is not set, this should impossible"),
            },
            latency: endtime.signed_duration_since(starttime).num_milliseconds() as i32,
        }
    }
}

HasMany! {
    (check_runs, foreign_key = check_id)
    #[table_name(checks)]
    struct Check {
        id: i32,
    }
}

HasMany! {
    (checks, foreign_key = user_id)
    #[table_name(users)]
    struct User {
        id: i32,
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerdeCheck {
    pub id: i32,
    pub url: String,
    pub rate: i32,
    pub last_start: Option<String>,
    pub last_end: Option<String>,
    pub http_status: Option<i32>,
    pub humanized_end: Option<String>,
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
            humanized_end: match c.last_end {
                Some(x) => {
                    Some(format!("{}",
                                 HumanTime::from(x.signed_duration_since(UTC::now().naive_utc()))))
                }
                None => None,
            },
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
    pub meta: Option<serde_json::Value>,
    pub user_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct CheckMeta {
    pub checkers: Vec<String>,
    state: Option<String>,
}

impl Default for CheckMeta {
    fn default() -> Self {
        CheckMeta {
            checkers: vec![String::from("alert_on_error_code")],
            state: None,
        }
    }
}

impl Check {
    pub fn exists(url: &str) -> Result<Self> {
        Ok(checks::table
               .filter(checks::url.eq(url))
               .get_result(&establish_connection())?)
    }

    pub fn get_ilike(limit: Option<i64>, query: String) -> Result<Vec<Self>> {
        match limit {
            Some(l) => {
                Ok(checks::table
                       .filter(checks::url.like(query.to_lowercase()))
                       .order(checks::http_status.desc())
                       .limit(l)
                       .load(&establish_connection())?)
            }
            None => {
                Ok(checks::table
                       .filter(checks::url.like(query.to_lowercase()))
                       .order(checks::http_status.desc())
                       .load(&establish_connection())?)
            }
        }
    }

    pub fn get_all(limit: Option<i64>) -> Result<Vec<Self>> {
        match limit {
            Some(l) => {
                Ok(checks::table
                       .order(checks::http_status.desc())
                       .order(checks::id)
                       .limit(l)
                       .load(&establish_connection())?)
            }
            None => {
                Ok(checks::table
                       .order(checks::http_status.desc())
                       .order(checks::id)
                       .load(&establish_connection())?)
            }
        }
    }


    pub fn for_serde(checks: Vec<Check>) -> Result<Vec<SerdeCheck>> {
        Ok(checks.iter().map(|x| x.into()).collect())
    }

    pub fn u_meta(&mut self, new_meta: CheckMeta) -> Result<Self> {
        self.meta = Some(serde_json::to_value(new_meta)?);
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
        let mut meta: CheckMeta = serde_json::from_value(self.meta.to_owned().unwrap_or_default())?;
        self.u_last_end(UTC::now().naive_utc())?;
        meta.state = Some(String::from_utf8(dst)?);
        self.u_meta(meta)?;
        self.u_http_status(easy.response_code()?)?;
        let checkrun = NewCheckRun::from(self);
        checkrun.insert()?;
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
            meta: Some(serde_json::to_value(CheckMeta::default()).unwrap()),
            http_status: None,
            last_start: None,
            last_end: None,
            user_id: newcheck.user_id
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "checks"]
pub struct NewCheck {
    pub url: String,
    pub rate: i32,
    pub user_id: i32
}

impl NewCheck {
    pub fn insert(&self) -> Result<Check> {
        use schema::checks;
        Ok(diesel::insert(self)
               .into(checks::table)
               .get_result(&establish_connection())
               .expect("Error saving new check"))
    }

    pub fn insert_if_url_not_exists(&self) -> Result<Check> {
        match Check::exists(&self.url) {
            Ok(check) => Ok(check),
            Err(_) => self.insert(),
        }
    }
}
