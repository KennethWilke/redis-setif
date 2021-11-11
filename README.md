# Atomic State Redis Module

This module provides additional redis commands to atomically set strings and hash fields if the field matches. Originally intended to facilitate state machine transitions.

## Set String If Equals

Changes the value of a string key only if the string matches the expected current value

`SETIF <string_key> <expected> <new>`

Examples:

```
> SETIF example-string old new
(error) WRONGTYPE Operation against a key holding the wrong kind of value
> SET example-string old
OK
> SETIF example-string old new
OK
> SETIF example-string old new
(error) Current value of "example-string" is not "old"
> SETIF example-string new newer
OK

```

## Set Hash Field If Equals

Similar to the Set String State, this format sets a hash field value.

`HSETIF <hash_key> <hash_field> <expected> <new>`

Examples:

```
> HSET example-hash field value
(integer) 1
> HSETIF example-hash field old new
(error) Current value of "example-hash" field "field" is not "old"
> HSETIF example-hash field value new
OK

```

# Building and Running

```
cargo build --release
redis-server --loadmodule ./target/release/libatomicstate.so
```