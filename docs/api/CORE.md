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
<summary><code>GET</code> <code>/_rs/core/v0/capabilities/&lt;api_name&gt;</code></summary>

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
| name       | optional | type     | description                     |
|------------|----------|----------|---------------------------------|
| capability | required | `string` | The API name of the capability. |
| error      | required | `Error`  | The error.                      |

```json5
{
	"capability": "relay",
	"error": /* ... */
}
```

</details>

### Host Redirect
Indicates if this Host redirects to another Host.

<details>
<summary><code>GET</code> <code>/_rs/core/v0/host/redirects</code></summary>

#### Request
N/A

#### Response
##### `200 OK`
| name      | optional | type             | description                                  |
|-----------|----------|------------------|----------------------------------------------|
| redirects | required | `boolean`        | If this Host redirects to another Host.      |
| links     | optional | `array (string)` | A list of Hosts that this Host redirects to. |

```json5
{
	"redirects": true,
	"links": ["https://rs-host.example.net", "https://example.com/rs_host"]
}
```

##### `501 Not Implemented`
This Node is not a Host.

| name  | optional | type    | description |
|-------|----------|---------|-------------|
| error | required | `Error` | The error.  |

</details>
