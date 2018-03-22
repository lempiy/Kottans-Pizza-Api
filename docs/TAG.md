# Tag

Tag Component responsible for CRUD operations over pizza tags.

## Methods

***

### List

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *GET* | `/api/v1/tag/list` | Get list of available tags |  :heavy_check_mark: |

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
| `results` | *[]Tag* | Actual result of query |


*Tag type:*

| Field | Type | Requirement |
| --- | --- | --- |
| `id` | *uint32* | Id of tag  |
| `name` | *string* | Tags name |
| `description` | *string* | Description of tag |

*Success Response Status:* - `200 OK`

**Response body:**

*Successful:*
```json
{
    "offset": 0,
    "limit": 100,
    "count": 5,
    "results": [
        {
            "id": 1,
            "name": "no-salt",
            "description": "Pizza with salt excluded."
        },
        {
            "id": 2,
            "name": "no-crust",
            "description": "Pizza without crust."
        },
        {
            "id": 3,
            "name": "hot",
            "description": "Hot pizza."
        },
        {
            "id": 4,
            "name": "hard-baked",
            "description": "Hard baked crust."
        },
        {
            "id": 5,
            "name": "rye-crust",
            "description": "Pizza with rye crust."
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
