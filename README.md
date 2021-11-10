# Atomic State Redis Module

This module was designed to facilitate the use of redis strings and hashes for stateful data.

## Set String State

Changes the value of a string key only if the string matches the expected current value

`state <string_key> <expected_current_value> <new_value>`

Examples:

```
127.0.0.1:6379> keys *
(empty array)
127.0.0.1:6379> state test-key "" new
(error) WRONGTYPE Operation against a key holding the wrong kind of value
127.0.0.1:6379> set test-key new
OK
127.0.0.1:6379> state test-key new edited
OK
127.0.0.1:6379> state test-key new edited
(error) Current state for test-key is not new
127.0.0.1:6379> state test-key edited edited-again
OK
```

## Set Hash Field State

Similar to the Set String State, this format sets a hash field value.

`state <hash_key> <hash_field> <expected_current_value> <new_value>`

Examples:

```
127.0.0.1:6379> keys *
(empty array)
127.0.0.1:6379> hset test-key test-field new
(integer) 1
127.0.0.1:6379> state test-key test-field old new
(error) Current state for test-key is not old
127.0.0.1:6379> state test-key test-field new edited
OK
127.0.0.1:6379> state test-key test-field new edited
(error) Current state for test-key is not new
```

# Building and Running

```
cargo build --release
redis-server --loadmodule ./target/release/libatomicstate.so
```