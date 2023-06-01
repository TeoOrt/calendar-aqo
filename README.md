# Calendar Scheduler in Rust

This is a backedn API built with the backend framework Rocket and with libraries such as Diesel for Connecting to our PostgreSQL

## Prerequisites

- Rust: Programming language know for speed and memory safety

- Diesel CLI: Diesel is used to handle database operations, like connecting, reading, and writing

- PostgreSQL: This application uses the popular db framework PostgreSQL for data operations

## Setup

1. Clone this repository

```
git clone git@github.com:TeoOrt/calendar-aqo.git

```

2. Setup your PostgreSQL database and take note of the credentials.

3. Navigate inside the project and create a .env file
   In this .env file add

```
DATABASE_URL=postgres://user:PasswordName@localhost/calendar
PGUSER=user
PGPASSWORD=Password
```

### Setup up Diesel

- Install Diesel

```
cargo install diesel_cli --no-default-features --features postgres
```

### SKip this part if you only want the project

- Run diesel setup to start the diesel service

```
diesel setup
```

- A new folder will appear called migrations, inside of go to the directory 2034-05_calendar and go to the up.sql and modify it to make it your own custom table
- run the database

```
diesel migration run
```

- This will create a new schema.rs file that will have the schema you want in your query

## Run and test

- To run this project

```
cargo run
```

- This will start a rocket server in port 8000

## API endpoints

<b> All of our endpoints and services will be stored in the mod.rs in the services folder<b>

- /calendar POST REQUEST
- - This endpoint will create a input to our db for our customers deciding a time

- /get_current_date GET
- - This endpoint will get what times are available for customers to schedule an event.
