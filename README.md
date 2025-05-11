# instructions

## Setup:


### Backend env variables
enter the root folder and create a .env file where you have to store all principal settings that the application will use (database options, connectivity options, default users ...)

```
nano .env
```

now you need to write this line:

```
JWT_SECRET=[STRONG KEY!!!]
POSTGRES_USER=[DATABASE USER]
POSTGRES_PASSWORD=[DATABASE USER PASSWORD] 
POSTGRES_DB=[DATABASE NAME]
DEFAULT_ADMIN=[APPLICATION ADMIN USERNAME]
DEFAULT_ADMIN_PASSWORD=[APPLICATION ADMIN PASSWORD]
DEFAULT_USER=[APPLICATION USER USERNAME]
DEFAULT_USER_PASSWORD=[APPLICATION USER USERNAME]
SETTINGS_PATH=[PATH OF THE SETTINGS FILE, DEFAULT ./settings.json]
```

### .env example:

```

### DATABASE SETTINGS ###

JWT_SECRET=92368843446958883401368835169004584754983166754091679294489897723038422606962
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres 
POSTGRES_DB=user
DEFAULT_ADMIN=admin
DEFAULT_ADMIN_PASSWORD=admin
DEFAULT_USER=user
DEFAULT_USER_PASSWORD=user
SETTINGS_PATH=./settings.json
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


## Extra customizations:

You can start the application if you previously set the .env file with all the variables, but if you want
to change some application behavior you can do it in the `settings.json` file in the `backend/` folder, here is a description of what those variables do in the application:



| variable |value type| Description |
| --- | --- |---|
| channel_label_prefix | String | set the default channel prefix e.g. ch -> (ch1,ch2, .., chN) |
| i_channels_number | positive integer | default ammount of matrix input channels |
| o_channels_number | positive integer | default ammount of matrix output channels |
| default_visibility | boolean | default visibility of matrix channels |
| command_delay | milliseconds (positive integer) | default delay for matrix commands |
| reconnect_delay | milliseconds (positive integer) | retry time to wait after a failed connection attempt |
| read_timeout | milliseconds (positive integer) | max time to wait for a response to a command |
| connection_timeout | milliseconds (positive integer) | max time to wait for a matrix connection |
| inactivity_timeout | milliseconds (positive integer) | max user inactivity time before freeing the matrix |
| max_connection_retries | positive integer | max connection retries ammount after a failed attempt |
| ping_socket_timeout | milliseconds (positive integer) | time to wait for a matrix response to the ping while adding a matrix socket |
| ping_socket_max_retries | positive integer | max ping retries after a failed ping attempt. |
| heartbeat_interval | milliseconds (positive integer) | time to wait before sending a ping to the WebSocket client |
| client_timeout | milliseconds (positive integer) | time to wait for a response to the sent WebSocket ping |


# USERS-CLI USAGE

The users-cli executable was maid to edit users inside the database, you can list, edit, create or remove users inside the running database (the database container has to be already started).

to execute it, simply type: `./users-cli [COMMAND KEYWORD]`

| command | keyword |
| ------- | ------- |
| list users inside the database | ls |
| create new user inside the database | new | 
| edit a user inside the database | edit |
| remove a user inside the database | rm |