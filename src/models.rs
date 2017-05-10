use chrono::NaiveDateTime;
use serde_json;
use super::schema::checks;


#[derive(Debug, Queryable, Identifiable, Associations)]
pub struct Check {
    pub id: i32,
    pub url: String,
    pub rate: i32,
    pub last_checked: Option<NaiveDateTime>,
    pub state: Option<String>
}

#[derive(Insertable)]
#[table_name = "checks"]
pub struct NewCheck<'a> {
    pub url: &'a str,
    pub rate: i32
}
