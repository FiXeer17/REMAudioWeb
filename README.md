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
DEFAULT_ADMIN=[APPLICATION ADMIN USERNAME]
DEFAULT_ADMIN_PASSWORD=[APPLICATION ADMIN PASSWORD]
DEFAULT_USER=[APPLICATION USER USERNAME]
DEFAULT_USER_PASSWORD=[APPLICATION USER USERNAME]
CHANNEL_DEFAULT_PREFIX=[PREFIX FOR CHANNELS]
I_CHANNEL_NUMBER=[NUMBER OF INPUT CHANNELS]
O_CHANNEL_NUMBER=[NUMBER OF OUTPUT CHANNELS]
DEFAULT_VISIBILITY=[DEFAULT VALUE OF VISIBILITY]


```

configuration example: 

```
JWT_SECRET=30247432903281965233239807424244205335774652113094630819941797969296964439814
POSTGRES_USER=user
POSTGRES_PASSWORD=postgres
POSTGRES_DB=users
DEFAULT_ADMIN=admin
DEFAULT_ADMIN_PASSWORD=admin
DEFAULT_USER=user
DEFAULT_USER_PASSWORD=user
CHANNEL_DEFAULT_PREFIX=ch
I_CHANNEL_NUMBER=16
O_CHANNEL_NUMBER=16
DEFAULT_VISIBILITY=true
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

if you start the application like that you have default websocket and matrix comunication configurations, if you desire you can change them in the `.env` file, here is an example of all customizable settings:

```
### DATABASE SETTINGS ###

### DATABASE SETTINGS ###

JWT_SECRET=92368843446958883401368835169004584754983166754091679294489897723038422606962
POSTGRES_USER=belle
POSTGRES_PASSWORD=postgres 
POSTGRES_DB=user
DEFAULT_ADMIN=admin
DEFAULT_ADMIN_PASSWORD=admin

### OPTIONAL SETTINGS ###

### TCP COMUNICATION SETTINGS ###
COMMAND_DELAY=20                #(milliseconds) Command delay.
RECONNECT_DELAY=1500            #(milliseconds) Reconnect delay after a fail.
READ_TIMEOUT =500               #(milliseconds) Wait for a response from the machine.
CONNECTION_TIMEOUT=2000         #(milliseconds) Wait for the connection to be established.
INACTIVITY_TIMEOUT=5000         #(milliseconds) Client inactivity time (free the matrix to other users).
MAX_RETRIES=3                   #(positive integer) Retries ammount if the connection request fail. 

### PING SOCKET SETTINGS ###
PING_SOCKET_TIMEOUT=1000        #(milliseconds) Wait for the connection to be established.
PING_SOCKET_MAX_RETRIES=2       #(positive integer) Retries ammount if the ping request fail.


### WEBSOCKET SETTINGS ###
HEARTBEAT_INTERVAL=5000         #(milliseconds) Time for ping interval (detect dead sessions).
CLIENT_TIMEOUT=10000            #(milliseconds) Time to wait for a client response to the ping request.
```
