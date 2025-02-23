# Backend instructions

## Setup:


### Database settings
choose settings to use for the database: username,password,database name and put them in a .env file:

```
nano .env
```
write:

```
POSTGRES_USER=[your username] 
POSTGRES_PASSWORD=[your password]  
POSTGRES_DB=[your db name]

```

### Backend env variables
first you have to write the `DATABASE_URL` and the `JWT_SECRET` variable needed by the `env_dns.rs` file variable so:

```
cd backend
nano .env
```

now you need to write this line:

```
DATABASE_URL= postgresql://[POSTGRES_USER]:[POSTGRES_PASSWORD]@db:5432/[POSTGRES_DB]

JWT_SECRET = [STRONG SECRET PASSWORD]


```

(username and password have to be set).

## Run:

go inside the folder where the `docker-compose.yml` file is located and then type:

```
docker compose up
```

to run a single service (e.g. only database) type:

```
docker compose up [service_name]

example:

docker compose up db
        or
docker compose up backend
```