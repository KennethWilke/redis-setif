#[macro_use]
extern crate redis_module;

use redis_module::{Context, KeyType, RedisError, RedisResult, RedisString, REDIS_OK};

fn setif(context: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 4 {
        return Err(RedisError::WrongArity)
    }

    let key = args.get(1).unwrap();
    let expected_value = args.get(2).unwrap().to_string();
    let new_value = args.get(3).unwrap().to_string();

    let element = context.open_key_writable(key);
    
    match element.key_type() {
        KeyType::String => {
            // I suspect there's a better way to do this, but this should always work
            let current_value = element.read().unwrap().unwrap();

            if current_value == expected_value {
                element.write(new_value.as_str())?;
                REDIS_OK
            } else {
                let err = format!("Current value of \"{}\" is not \"{}\"", key, expected_value);
                Err(RedisError::String(err))
            }
        }
        _ => Err(RedisError::WrongType)
    }
}

fn hsetif(context: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() != 5 {
        return Err(RedisError::WrongArity)
    }

    let key = args.get(1).unwrap();
    let field = args.get(2).unwrap().to_string();
    let expected_value = args.get(3).unwrap().to_string();
    let new_value = args.get(4).unwrap().to_owned();

    let element = context.open_key_writable(key);
    
    match element.key_type() {
        KeyType::Hash => {
            let current_value =  match element.hash_get(field.as_str()).unwrap() {
                Some(current_value) => current_value.to_string(),
                None => {
                    let err = format!("Could not read field \"{}\" of hash \"{}\"", field, key);
                    return Err(RedisError::String(err))
                }
            };

            if current_value == expected_value {
                element.hash_set(field.as_str(), new_value);
                REDIS_OK
            } else {
                let err = format!("Current value of \"{}\" field \"{}\" is not \"{}\"", key, field, expected_value);
                Err(RedisError::String(err))
            }
        },
        _ => Err(RedisError::WrongType)
    }
}

redis_module! {
    name: "setif",
    version: 1,
    data_types: [],
    commands: [
        ["SETIF", setif, "write", 0, 0, 0],
        ["HSETIF", hsetif, "write", 0, 0, 0]
    ],
}
