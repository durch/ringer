use chrono::NaiveDateTime;
use super::schema::checks;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::SaveChangesDsl;
use fdw_error::Result;

#[derive(Debug, Queryable, Identifiable, Associations, PartialEq, AsChangeset)]
pub struct Check {
    pub id: i32,
    pub url: String,
    pub rate: i32,
    pub last_checked: Option<NaiveDateTime>,
    pub state: Option<String>
}

impl Check {
    pub fn update_state(&mut self, new_state: String, conn: &PgConnection) -> Result<Self> {
        self.state = Some(new_state);
        Ok(self.save_changes::<Self>(conn)?)
    }

    pub fn update_last_checked(&mut self, time: NaiveDateTime, conn: &PgConnection) -> Result<Self> {
        self.last_checked = Some(time);
        Ok(self.save_changes::<Self>(conn)?)
    }

//    Returns number of rows affected
    pub fn delete(&self, conn: &PgConnection) -> Result<usize> {
        Ok(diesel::delete(self).execute(conn)?)
    }

    pub fn get(id: i32, conn: &PgConnection) -> Result<Self> {
        Ok(checks::table.filter(checks::id.eq(id)).first::<Self>(conn)?)
    }
}

#[derive(Insertable)]
#[table_name = "checks"]
pub struct NewCheck<'a> {
    pub url: &'a str,
    pub rate: i32
}

impl<'a> NewCheck<'a> {
    pub fn insert(&self, conn: &PgConnection) -> Check {
        use schema::checks;

        diesel::insert(self).into(checks::table)
            .get_result(conn)
            .expect("Error saving new post")
    }
}
