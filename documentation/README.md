# API DOCUMENTATION

### For api urls the prefix is `/api`

### api urls:

- [POST] sign-in: `/api/auth/signin`
- [GET] get connections: `/api`

## Json body:

### Sign-in:
```
    {
        "username":"admin",
        "password":"admin",
        "session_type": "web" or "native"
    }
    
```

## Responses body:
`/api`:
```
{
	"sockets": [
		{
			"ip": "172.18.0.2",
			"port": 2000
		}
	]
}
{
	"sockets": null
}

```
`/api/auth/signin`:
```
{
	"access_token": "eyJ0eXAiOiJKV1QiLCJhbOiJIUzI1NiJ9.eyJOYXRpdmUViIjoxLCJzZXNzaW9uX3R5cGUiOiJuYXRpdmUifX0.YQ1SE1b-97clmOFSdEyiPPuv1VHdeDmF-2w",
	"admin": true
}

```

### For WebSocket urls the prefix is `/ws`

### WebSocket urls:

- [GET] authentication (before the WebSocket comunication): `/ws/auth`
- [GET] application: `/ws/app`
- [POST] set socket: `/ws/socket?uuid=[YOUR UUID FROM AUTH]`


### instruction:

To start a comunication with WebSocket protocol you'll need a UUID that certify that you are actually authenticated, to do so, before make a request to the `authentication` url with the header: `Authorization: Bearer [YOUR TOKEN]`, you'll get a UUID into put as a query parameter inside the `application` url
## Json body:

### set socket:
```
{
	"socket":"matrix-simulator:2000"
}
    
```

## Responses body:
`/ws/auth`:
```
{
	"uuid": "80e1a00b-fb26-4e36-89d0-9df7e920b041"
}
```
`/ws/socket?uuid=[YOUR UUID FROM AUTH]`:
```
{
    "socket":"127.0.0.1:2000"
}
```
(if socket isn't set the application will connect automatically to the DEFAULT_SOCKET set in the .env file).

### Application request:

`[GET] http://localhost:8000/ws/app?uuid=[YOUR UUID FROM AUTH]`

## Commands:

### Send a set command:

To send a command used to set some value in the matrix you'll need the function code (what you want to do), 
the input/output flag, the channel where value is applied to, and the final value.
To do so you have to send a json formatted WebSocket message with this body:

### set command body:

```
{
    "section": "[POSSIBLE SECTIONS]",
    "io": "[POSSIBLE IO]", -> OPTIONAL                   
    "channel": String,  -> OPTIONAL
    "value" : String, 
}

```

#### POSSIBLE SECTIONS:
sections are specific keywords, here is a list of sections that the engine actually support:
```
    preset -> "preset";
    mute -> "mute";
    volume ->"volume";
```

### POSSIBLE IO:
input/output sources are specific keywords, here is a list of ios that the engine actually support:
```
   general audio -> "both";
   input ->"input";
   output -> "output";
```