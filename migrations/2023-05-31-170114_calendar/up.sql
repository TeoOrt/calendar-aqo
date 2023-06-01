-- Your SQL goes here
CREATE TABLE Calendar(

    id SERIAL PRIMARY KEY,
    time TEXT NOT NULL,
    date TEXT NOT NULL,
    description TEXT NOT NULL
)