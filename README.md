# instructions

## Setup:


### Backend env variables
enter the backend folder and create a .env file where you have to store all principal settings that the application will use (database options, connectivity options, default users ...)

```
cd backend
nano .env
```

now you need to write this line:

```
JWT_SECRET=[STRONG KEY!!!]
POSTGRES_USER=[DATABASE USER]
POSTGRES_PASSWORD=[DATABASE USER PASSWORD] 
POSTGRES_DB=[DATABASE NAME]
DEFAULT_SOCKET=[TO THE MATRIX] for dev simulation use -> matrix-simulator:2000
DEFAULT_ADMIN=[APPLICATION ADMIN USERNAME]
DEFAULT_ADMIN_PASSWORD=[APPLICATION ADMIN PASSWORD]


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
