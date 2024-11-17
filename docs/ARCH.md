### Notes
From here going forward, all references to "RS" refer to `relay-struct`.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and
"OPTIONAL" in this document are to be interpreted as described in
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

# Architecture

## Fediverse
https://wikipedia.org/wiki/Fediverse

## Nodes
Each server running an RS implementation is called a Node.
The Node typically provides the frontend as well as the backend API; however, a Node may provide one or neither if desired.
A Node MUST provide a method of communication with other Nodes via `relay-struct`.
A Node MAY block specific Nodes from communication (a blocklist).
A Node MAY also block all but specific Nodes from communication (an allowlist).

## Domains
Domains refer to domain names [as defined by the IETF](https://www.rfc-editor.org/rfc/rfc1035).
Each Node MUST have at least one Domain. Nodes MUST NOT be referred to by their IP address in Handles.

## Posts & Content
A Post is a container for data sent by a user.

Content refers to one of three things:
- File Attachments
- Messages
- Miscellaneous Data that a user attaches to a Post.

A Post contains Content or metadata that describes an interaction with another Post or with the platform.
A Post SHOULD refer to a social media post with attached Content.
A Post MAY also refer to:
- A reaction.
- A status update.

### Direct Messages (DMs)
Direct Messages, also referred to as DMs, are defined as Posts shared between users directly. Nodes MAY moderate DMs as they see fit. However, End-to-End Encrypted DMs MUST only be moderated when prompted by a user using a report feature. Nodes SHOULD provide End-to-End Encryption for users in DMs, but Nodes MUST NOT claim to provide End-to-End Encryption when other involved Nodes do not provide such a feature or when Nodes can access either end decrypted without user involvement, and Nodes SHOULD notify users when they enter a chat with another user of a Node that does not support End-to-End Encryption.

## Relays
Each Node optionally acts as a Relay.
A Relay scrapes and relays information from other Relays.
Nodes communicate with other Relays to propagate their Posts across the Fediverse.

## Hosts & CDNs
For Content storage, Nodes MAY host their own Content or use a large CDN.
A Node hosting its own Content is referred to as a Host.
Hosts provide Content for their Node(s) including a message and media database.
Hosts MAY additionally selectively provide Content based on size or type and offload the rest of the Content to an external Host or CDN. This requires the Host to provide links to the Content hosted on the external Host or CDN.
Nodes may also cache messages and media relayed from other Hosts or CDNs in their own Host or CDN.
It is RECOMMENDED that caches last no longer than one month per Post.
Keeping caches for too long may lead to storage and privacy concerns.

## Handles
A Handle is a user's username, UUID, or other identifier for other Nodes to look up.
There are two common Handle formats a Node MUST recognize:
- Canonical Handles
- Common Handles

### Canonical Handles
Canonical Handles are defined as `domain:user` where `domain` refers to a Node's Domain and `user` refers to a username.

### Common Handles
Common Handles are defined as `@user@domain`. Common handles are to be used by users.

### Additional Formats
Nodes MAY implement additional formats including but not limited to:
- Matrix Handles (`@user:domain`)
- E-Mail Handles (`user@domain`)
- Discriminated Handles (`user#discrim`) where `discrim` refers to a four-digit base-ten string.

However, Nodes MUST implement an API as defined by `relay-struct` to convert these additional formats into a Canonical Handle.

## User Identification
User Identification entails a Handle and a public and private key for Content integrity.
User Identification is stored in users' respective Nodes.
Each user MUST have a public and private key used to determine Content integrity and prevent tampering.
*The algorithms used are currently undefined! This is subject to change in the near future.*
