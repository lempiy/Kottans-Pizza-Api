# Ingredient

Ingredient Component responsible for CRUD operations over pizza ingredients.

## Methods

***

### List

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *GET* | `/api/v1/ingredient/list` | Get list of available ingredients |  :heavy_check_mark: |

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
| `results` | *Array<Ingredient>* | Actual result of query |


*Ingredient type:*

| Field | Type | Requirement |
| --- | --- | --- |
| `id` | *uint32* | Id of ingredient  |
| `name` | *string* | Ingredients name |
| `price` | *float64* | Price of ingredient |
| `created_date` | *string* | Datetime UTC when ingredient was added to DB |

*Success Response Status:* - `200 OK`

**Response body:**

*Successful:*
```json
{
    "offset": 0,
    "limit": 3,
    "count": 14,
    "results": [
        {
            "id": 1,
            "name": "pineapple",
            "description": "pineapple",
            "image_url": "static/images/ananas.png",
            "price": 0.8,
            "created_date": "2018-03-05T18:41:29.508613Z"
        },
        {
            "id": 2,
            "name": "eggplant",
            "description": "eggplant",
            "image_url": "static/images/baklazhan.png",
            "price": 0.9,
            "created_date": "2018-03-05T18:41:29.508613Z"
        },
        {
            "id": 3,
            "name": "bacon",
            "description": "bacon",
            "image_url": "static/images/becone.png",
            "price": 1,
            "created_date": "2018-03-05T18:41:29.508613Z"
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
