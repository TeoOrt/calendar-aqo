use crate::models::NewCalendar;
use crate::models::{self, Calendar};
use crate::schema;
use crate::schema::calendar;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL needs to be set in .env");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Erroc connecting to {}", db_url))
}

// creating post request

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    date: String,
    time: String,
    description: String,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/test")]
pub fn hello_world() -> &'static str {
    "Hello World!"
}

// help me create
#[post("/calendar", format = "json", data = "<post>")]
pub fn create_post(post: Json<NewPost>) -> Result<Created<Json<NewPost>>> {
    // use self::schema::calendar::dsl::*;

    use models::NewCalendar;

    let connecting = &mut establish_connection();

    let new_post = NewCalendar {
        date: post.date.to_string(),
        time: post.time.to_string(),
        description: post.description.to_string(),
    };

    // diesel::insert_into(self::schema::calendar::dsl::calendar)
    //     .values(&new_post)
    //     .execute(connecting)
    //     .expect("Error saving new post");

    schedule_event(new_post, connecting).expect("Error saving new post");
    Ok(Created::new("/").body(post))
}

fn schedule_event(
    new_event: NewCalendar,
    conn: &mut PgConnection,
) -> Result<Calendar, &'static str> {
    use schema::calendar::dsl::*;

    let num_events = calendar
        .filter(date.eq(&new_event.date))
        .count()
        .get_result::<i64>(conn)
        .expect("Error Counting events");

    if num_events < 3 {
        diesel::insert_into(calendar)
            .values(&new_event)
            .get_result(conn)
            .map_err(|_| " Error inserting event")
    } else {
        Err("Cannot Schedule More than three events the same day")
    }
}
