### Notes
From here going forward, all references to "RS" refer to `relay-struct`.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and
"OPTIONAL" in this document are to be interpreted as described in
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

This document follows the format defined in [this gist](https://gist.github.com/azagniotov/a4b16faf0febd12efbc6c3d7370383a6).

# RelayStruct API

## Index
- [Node Core](api/CORE.md)
- [Post](api/POST.md)
- [Relay](api/RELAY.md)
- [Host](api/HOST.md)
- [User Identification](api/USERID.md)

## Description
All RelayStruct APIs follow the `/_rs/<api_name>/v<api_version>` format.
This is for clarity, compartmentalization, and ease of implementation.
Additionally, all APIs MUST support JSON requests and responses.

## Error Handling
All errors follow the following format:
| name        | optional | type     | description            |
|-------------|----------|----------|------------------------|
| error       | required | `string` | The error.             |
| error_type  | required | `string` | The error type.        |
| description | required | `string` | The error description. |
Any references to this type are referred to as `Error`.
