# instructions

## Setup:


### Backend env variables
enter the backend folder and create a .env file where you have to store the database user password and name variables and the jwt_secret variable used to generate the jwt_token so:

```
cd backend
nano .env
```

now you need to write this line:

```
POSTGRES_USER=[your username] 
POSTGRES_PASSWORD=[your password]  
POSTGRES_DB=[your db name]
JWT_SECRET=[STRONG SECRET PASSWORD]


```

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
