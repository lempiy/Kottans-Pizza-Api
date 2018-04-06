# WebSocket interface

WebSocket interface implements real-time notifications for WebSocket clients. 
It has its own one-off token for authentication based on general authentication
from [Users component](USERS.md#login). Authentication is being held upon 
client-server handshake.

## Methods

***

### Get ticket

***Protocol: HTTPS***

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *GET* | `/api/v1/ws/ticket` | Get one-off token for websocket session handshake. Ticket is valid for 1 min. |  :heavy_check_mark: |

**Request body:** `None`

*Returned values:*

| Field | Type | Requirement |
| --- | --- | --- |
| `success` | *bool* | Is ticket request was successful? |
| `info` | *string* | Descriptive message about request result. |
| `token` | *string* | Token one-off to be used for WS handshake. Valid for 1 min. |

*Success Response Status:* - `200 OK`

**Response body:**

*Successful:*
```json
{
    "success": true,
    "info": "Your ws connection token is valid for 60s",
    "token": "x8m1q6uLSJGYOQzXTESWc72UObD5xokh"
}
```

*Failed:*
```json
{
    "success": false,
    "error": "Wrong authorization data"
}
```

## Handshake

***Protocol: WSS***

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *GET* | `/ws` | WS Upgrade Request |  :heavy_multiplication_x: |

*Url params:*

| Field | Type | Requirement |
| --- | --- | --- |
| `key` | *string* | One-off token taken from [get ticket](#get-ticket) method |

*Example of url*

`wss://pizza-tele.ga/ws?key=x8m1q6uLSJGYOQzXTESWc72UObD5xokh"`

**Output:**

*Successful:*

Upgraded WS connection, with `101` Status code.

*Failed:*

Upgraded WS connection and immediate Close Event with `4001` status code.

## Events

### New Pizza Created

*Event identifier*: `CREATE_PIZZA`

*Returned values:*

| Field | Type | Requirement |
| --- | --- | --- |
| `event_name` | *string* | Constant event identifier  |
| `data` | *Pizza* | Number of taken records |

*Pizza type:*

| Field | Type | Requirement |
| --- | --- | --- |
| `uuid` | *string* | UUID of pizza  |
| `name` | *string* | Pizza name |
| `store_id` | *integer* | Id of store where pizza was created  |
| `user_uuid` | *string* | UUID of pizza manager - author of pizza order |
| `size` | *integer* | Size of pizza  |
| `accepted` | *bool* | Whether or not pizza was received by customer  |
| `price` | *float64* | Price of pizza |
| `description` | *string* | Description for pizza  |
| `img_url` | *string* | URL with pizza picture  |
| `created_date` | *string* | Datetime UTC when pizza was created |
| `time_prepared` | *string* | Datetime UTC when pizza will be prepared |
