# Store

Store Component responsible for read operations over pizza stores.

## Methods

***

### List

| Method | Url | Description | Auth required |
| --- | --- | --- | --- |
| *GET* | `/api/v1/store/list` | Get list of available stores |  :heavy_check_mark: |

**Request body:** `None`

*Returned values:*

| Array |
| --- |
| *[]Store* |


*Store type:*

| Field | Type | Requirement |
| --- | --- | --- |
| `id` | *integer* | Id of store  |
| `name` | *string* | Store name |

*Success Response Status:* - `200 OK`

**Response body:**

*Successful:*
```json
[
    {
        "id": 1,
        "name": "Anton Store"
    },
    {
        "id": 2,
        "name": "Pizza Roma"
    }
]
```

*Failed:*
```json
{
    "success": false,
    "error": "Server error"
}
```
