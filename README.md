# Setup install PostgreSQL

```
sudo apt update
sudo apt install postgresql
```

## Start Service

```
sudo service postgresql start
```

- The service will always be runninng on port 5432 on localhost
- If running service from outside host try different approach

## Creating User an AUTH for db

There is a need for AUTH in a db to be able to modify your localpostgress, this is also needed in case of using a docker container

```
SELECT * FROM pg_user;
CREATE USER user WITH PASSWORD 'your_password';
```

- After creating user grant them privilages

```
GRANT ALL PRIVILAGES ON DATABASE your_database TO user;
```

## Restart Service

```
sudo service postgresql restart
```
