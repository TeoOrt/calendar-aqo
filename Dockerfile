FROM rustlang/rust:nightly

WORKDIR /app

COPY . /app

RUN cargo install diesel_cli --no-default-features --features postgres


RUN cargo build --release

EXPOSE 8000

CMD ["/app/target/release/calendar-aqo"]

# Need to add CMD, diesel setup, and then diesel migration run, to start psql services



