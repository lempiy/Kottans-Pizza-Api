# Users And Auth

Users Component responsible for authorization inside the system 
and CRUD operations over pizza management users.

## Methods

***

### Create user

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *POST* | `/api/v1/user/create` | Create new user inside system |  :heavy_multiplication_x: |

**Request body:**

```json
{
    "username": "lempiy",
    "password": "secret42",
    "password_repeat": "secret42",
    "email": "lempiy@gmail.com",
    "store_id": 1,
    "store_password": "q1w2e3r4"
}
```

*Validation:*

| Field | Type | Requirement | 
| --- | --- | --- |
| `username` | *string* | Unique. Min length 2, max - 24 |
| `password` | *string* | Min length 8 |
| `password_repeat` | *string* | Should match with `password` |
| `email` | *string* | Should be valid email |
| `store_id` | *integer* | Should be existing store id |
| `store_password` | *string* | Should be valid store password. Min length 8 chars. |


*Success Response Status:* - `201 Created`

**Response body:**

*Successful:*
```json
{
    "success": true,
    "uuid": "2bf2ac6e-6f1f-4735-8ea6-0b8bdd48fc39"
}
```

*Failed:*
```json
{
    "success": false,
    "error": "Validation failed",
    "validations": [
        "Passwords do not match",
        "Email is not valid",
        "Password is not valid. Min length is 8",
        "User with such username already exist",
        "Wrong store credentials",
        "Store password is not valid. Min length is 8"
    ]
}
```

***

### Login

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *POST* | `/api/v1/user/login` | Get JWT token for user |  :heavy_multiplication_x: |

**Request body:**

```json
{
    "username": "lempiy",
    "password": "secret42",
}
```

*Validation:*

| Field | Type | Requirement | 
| --- | --- | --- |
| `username` | *string* | Min length 2, max - 24 |
| `password` | *string* | Min length 8 |


*Success Response Status:* - `200 OK`

**Response body:**

*Successful:*
```json
{
    "success": true,
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE1MjAwMjU3MzksInVzZXJuYW1lIjoibGVtcGl5IiwidXVpZCI6ImQxNjBmZTZjLTIwYTEtNDFkMS1hMzMxLTIzODNkNmExODVjZSJ9.svKfKWHzQ4radAoZrWRHTkHOzQ2qiuLM6dnqmnXxuhY"
}
```

*Failed:*
```json
{
    "success": false,
    "error": "Wrong username or password"
}
```

***

### My Info

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *POST* | `/api/v1/user/my_info` | Get personal data of currently authorized user |  :heavy_check_mark: |

**Request body:** `None`

*Success Response Status:* - `200 OK`

**Response body:**

*Successful:*
```json
{
    "username": "lempiy",
    "uuid": "d160fe6c-20a1-41d1-a331-2383d6a185ce",
    "email": "lempiy@gmail.com",
    "created_at": "2018-03-01T19:47:32.312036Z",
    "last_login": "2018-03-02T16:22:19.633329Z"
}
```

*Failed:*
```json
{
    "success": false,
    "error": "Wrong authorization data"
}
```
