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

### For WebSocket urls the prefix is `/ws`

### WebSocket urls:

- [GET] authentication (before the WebSocket comunication): `/ws/auth`
- [GET] application: `/ws/app`


### instruction:

To start a comunication with WebSocket protocol you'll need a UUID that certify that you are actually authenticated, to do so, before make a request to the `authentication` url with the header: `Authorization: Bearer [YOUR TOKEN]`, you'll get a UUID into put as a query parameter inside the `application` url

### Application request:

```
[GET] http://localhost:8000/ws/app?uuid=[YOUR UUID FROM AUTH]
```

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