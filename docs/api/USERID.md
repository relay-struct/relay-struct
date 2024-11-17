# User Identification

## Requests

### Query User ID

<details>
<summary><code>GET</code> <code>/_rs/uid/v0/query</code></summary>

### Request
| name   | optional | type     | description        |
|--------|----------|----------|--------------------|
| handle | required | `string` | The user's Handle. |

```json5
{
	"handle": "@user@domain"
}
```

### Response

#### `200 OK`
| name   | optional | type     | description                  |
|--------|----------|----------|------------------------------|
| handle | required | `string` | The user's Canonical Handle. |
| pubkey | required | `string` | The user's Public Key.       |

```json5
{
	"handle": "domain:user",
	"pubkey": "Public Key"
}
```

#### `400 Bad Request`
The requested Handle is unable to be parsed.
| name   | optional | type     | description           |
|--------|----------|----------|-----------------------|
| handle | required | `string` | The requested Handle. |

#### `404 Not Found`
The requested Handle does not exist.
| name   | optional | type     | description           |
|--------|----------|----------|-----------------------|
| handle | required | `string` | The requested Handle. |

</details>
