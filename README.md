# Backend instructions

## Setup:

first you have to write the `DATABASE_URL` needed by the docker-compose.yml file so:

```
cd backend
nano .env
```

now you need to write this line:

```
DATABASE_URL= postgresql://[user]:[password]@db:5432/users

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