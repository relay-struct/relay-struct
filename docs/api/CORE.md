# Node Core
A common API for interacting with Nodes. This is a required API.

## Requests

### Query Capabilities

<details>
<summary><code>GET</code> <code>/_rs/core/v0/capabilities</code></summary>

#### Request
N/A

#### Response

##### `200 OK`
| name         | optional | type             | description                       |
|--------------|----------|------------------|-----------------------------------|
| capabilities | required | `array (string)` | The API names of each capability. |

```json5
{
	"capabilities": ["core", "post", "relay", "host"]
}
```

</details>

### Capability Support

<details>
<summary><code>GET</code> <code>/_rs/core/v0/capabilities/{api_name}</code></summary>

#### Request
N/A

#### Response

##### `200 OK`
| name        | optional | type     | description                        |
|-------------|----------|----------|------------------------------------|
| min_version | required | `number` | The minimum supported API version. |
| max_version | required | `number` | The maximum supported API version. |

```json5
{
	"min_version": 1,
	"max_version": 3
}
```

##### `404 Not Found`
The requested capability does not exist.

</details>
