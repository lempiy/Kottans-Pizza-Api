## Pizza API Documentation

### Content

* [Store](STORE.md)
    * [List](STORE.md#list)
* [Users and Auth](USERS.md)
	* [Create User](USERS.md#create-user)
	* [Login](USERS.md#login)
	* [My Info](USERS.md#my-info)
* [Pizza](PIZZA.md)
    * [Unaccepted List](PIZZA.md#list)
    * [Create Pizza](PIZZA.md#create-pizza)
* [Ingredient](INGREDIENT.md)
    * [List](INGREDIENT.md#list)
* [Tag](TAG.md)
    * [List](TAG.md#list)
* WebSocket Interface

### General Info

All API endpoints are being served on path `/api/v1/`.
Each component of API hash own namespace, for instance, `/api/v1/user`, 
or `/api/v1/pizza`. Most of the endpoints require Authorization token to be
sent in `Authorization: Bearer ...` header. Token can be received
using [login](USERS.md#login) method of API. Token is a simple JWT with `exp` param inside payload. So it may 
expire after N hours. Example of token, its header and payload:

```js
let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.ExODSJ9._mov-jeMzOwqEwBhyxrF3GZ3I8hKzw8pPZMwB-Do6d8"
```
*Header:*
```json
{
  "typ": "JWT",
  "alg": "HS256"
}
```
*Payload:*
```json
{
  "exp": 1520015590,
  "username": "lempiy",
  "uuid": "d160fe6c-20a1-41d1-a331-2383d6a185ce"
}
```

**Server supports HTTPS and CORS. Its still under development 
so all missing components may appear soon.**