# Pizza

Pizza Component responsible for CRUD operations over pizza.

## Methods

***

### List

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *GET* | `/api/v1/pizza/list` | Get list of non-accepted pizzas |  :heavy_check_mark: |

**Request body:** `None`

*Url params:*

| Field | Type | Requirement |
| --- | --- | --- |
| `offset` | *uint64* | Number of records to skip. Default is 0 |
| `limit` | *uint64* | Number of records to take. Default is 100 |


*Returned values:*

| Field | Type | Requirement |
| --- | --- | --- |
| `offset` | *uint64* | Number of skipped records  |
| `limit` | *uint64* | Number of taken records |
| `count` | *uint64* | Total count of records |
| `results` | *[]pizza* | Actual result of query |


*pizza type:*

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

*Success Response Status:* - `200 OK`

**Response body:**

*Successful:*
```json
{
    "offset": 0,
    "limit": 100,
    "count": 1,
    "results": [
        {
            "uuid": "c8a3f984-bd39-4c03-bece-4629c9bcc2cd",
            "name": "Suppa pizza!",
            "store_id": 1,
            "user_uuid": "d160fe6c-20a1-41d1-a331-2383d6a185ce",
            "size": 30,
            "accepted": false,
            "price": 8.7,
            "description": "Some description",
            "img_url": "static/upload/c8a3f984-bd39-4c03-bece-4629c9bcc2cd_pizza.png",
            "created_date": "2018-03-22T18:56:35.176577Z",
            "time_prepared": "2018-03-22T19:02:50Z"
        }
    ]
}
```

*Failed:*
```json
{
    "success": false,
    "error": "Wrong authorization data"
}
```

***

### Create pizza

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *POST* | `/api/v1/pizza/create` | Create new pizza |  :heavy_check_mark: |


**Content-Type: multipart/formdata**

**Request body - Validation:**

| Key | Value | Requirement | 
| --- | --- | --- |
| `name` | *field* | Min length 3, max - 24 |
| `description` | *field* | _Optional_ |
| `size` | *field* | Should be either 30, 45 or 60 |
| `ingredients` | *field* | JSON serialized array of integers - ingredient IDs. Non-empty. |
| `tags` | *field* | JSON serialized array of integers - tag IDs. Can be empty array. |
| `image` | *file* | Up to 3MB `image/png` file |


**Request body - Example:**

| Key | Value |
| --- | --- |
| `name` | Suppa pizza! |
| `description` | Some description |
| `size` | 30 |
| `ingredients` | [1,5,3,12] |
| `tags` | [] |
| `image` | File(`pizza_img.png`) |


*Success Response Status:* - `201 Created`

**Response body:**

*Successful:*
```json
{
    "success": true,
    "time_prepared": "2018-03-22T19:02:50Z"
}
```

*Failed:*
```json
{
    "success": false,
    "error": "Validation failed",
    "validations": [
        "Pizza name is not valid. Min length is 3, max - is 24",
        "Wrong file MIME type - expected: 'image/png'",
        "Pizza size can be either 30, 45 or 60",
        "Ingredients cannot be empty",
        "Tags with ids [21, 16] are not exist"
    ]
}
```
