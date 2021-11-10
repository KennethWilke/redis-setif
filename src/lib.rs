#[macro_use]
extern crate redis_module;

use redis_module::{Context, KeyType, RedisError, RedisResult, RedisString, RedisValue};

fn string_state(context: &Context, args: Vec<RedisString>) -> RedisResult {
    let key = args.get(1).unwrap();
    let expected_state = args.get(2).unwrap().to_string();
    let new_state = args.get(3).unwrap().to_string();

    let element = context.open_key_writable(key);
    
    match element.key_type() {
        KeyType::String => {
            let current_state = element.read().unwrap().unwrap();

            if current_state == expected_state {
                element.write(new_state.as_str())?;
                Ok(RedisValue::SimpleString("OK".to_string()))
            } else {
                let err = format!("Current state for {} is not {}", key, expected_state);
                Err(RedisError::String(err))
            }
        }
        _ => Err(RedisError::WrongType)
    }
}

fn hash_state(context: &Context, args: Vec<RedisString>) -> RedisResult {
    let key = args.get(1).unwrap();
    let field = args.get(2).unwrap().to_string();
    let expected_state = args.get(3).unwrap().to_string();
    let new_state = args.get(4).unwrap().to_owned();

    let element = context.open_key_writable(key);
    
    match element.key_type() {
        KeyType::Hash => {
            let current_state = element.hash_get(field.as_str()).unwrap().unwrap().to_string();

            if current_state == expected_state {
                element.hash_set(field.as_str(), new_state);
                Ok(RedisValue::SimpleString("OK".to_string()))
            } else {
                let err = format!("Current state for {} is not {}", key, expected_state);
                Err(RedisError::String(err))
            }
        },
        _ => Err(RedisError::WrongType)
    }
}

fn state(context: &Context, args: Vec<RedisString>) -> RedisResult {
    match args.len() {
        4 => string_state(context, args),
        5 => hash_state(context, args),
        _ => Err(RedisError::WrongArity)
    }
}

redis_module! {
    name: "atomic-state",
    version: 1,
    data_types: [],
    commands: [
        ["state", state, "write", 0, 0, 0],
    ],
}
