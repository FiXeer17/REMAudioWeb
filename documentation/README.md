# API DOCUMENTATION

### for api urls the prefix is `/api`

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