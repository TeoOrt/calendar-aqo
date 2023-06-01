use super::schema::calendar;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = calendar)]
pub struct Calendar {
    pub id: i32,
    pub date: String,
    pub time: String,
    pub description: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = calendar)]
pub struct NewCalendar {
    pub date: String,
    pub time: String,
    pub description: String,
}
