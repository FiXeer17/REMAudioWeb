# API DOCUMENTATION

### For api urls the prefix is `/api`

### api urls:

- [POST] register: `/auth/register`
- [POST] sign-in: `/auth/signin`

## Json body:

### Register:
```
    {
        "username":"foo",
        "email":"foobar@mail.com",
        "password":"fobar125",
        "session_type": "web" or "native"
    }
```
### Sign-in:
```
    {
        "email":"foobar@mail.com",
        "password":"fobar125",
        "session_type": "web" or "native"
    }
```

### For WebSocket urls the prefix is `/ws`

### WebSocket urls:

- [GET] authentication (before the WebSocket comunication): `/auth`
- [GET] application: `/app`


### instruction:

To start a comunication with WebSocket protocol you'll need a UUID that certify that you are actually authenticated, to do so, before make a request to the `authentication` url with the header: `Authorization: Bearer [YOUR TOKEN]`, you'll get a UUID into put as a query parameter inside the `application` url

### Application request:

```
[GET] http://localhost:8000/ws/app?uuid=[YOUR UUID FROM AUTH]
```