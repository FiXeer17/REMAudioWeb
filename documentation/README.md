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