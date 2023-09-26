use actix_web::web;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct BadPayload {
    error: String,
    field: String,
}

/*
    converts JSON to a string, so that it can be further converted to a hashmap
    for purposes of validating the json keys

    e.g. convert_json_to_string({ "username": "hello" }) -> String({"username": "hello"})
*/
fn convert_json_to_string(json: &web::Bytes) -> Result<String, Vec<BadPayload>> {
    match String::from_utf8(json.to_vec()).map_err(|_| {
        vec![BadPayload {
            field: "payload".to_string(),
            error: "invalid payload".to_string(),
        }]
    }) {
        Ok(result) => Ok(result),
        Err(error) => Err(error),
    }
}

/*
    converts JSON string to hashmap so that the keys can be verified when a user
    submits JSON to any endpoint
*/
fn convert_jsonstring_to_hashmap(
    jsonstring: &String,
) -> Result<HashMap<String, serde_json::Value>, Vec<BadPayload>> {
    match serde_json::from_str::<HashMap<String, serde_json::Value>>(jsonstring).map_err(|_| {
        vec![BadPayload {
            field: "payload".to_string(),
            error: "invalid payload".to_string(),
        }]
    }) {
        Ok(result) => Ok(result),
        Err(error) => Err(error),
    }
}

/*
    validate user JSON at runtime to ensure they're passing what we want.

    e.g. validate_json(...jsonFromActix, vec!["username", "password"])
*/
pub fn validate_json<'a>(
    json: &web::Bytes,
    keys: &'a Vec<&str>,
) -> Result<HashMap<&'a str, serde_json::Value>, Vec<BadPayload>> {
    /* converts JSON to string */
    let json_as_string = &match convert_json_to_string(json) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    /* create hashmap from JSON string */
    let lookup = &match convert_jsonstring_to_hashmap(json_as_string) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    // another hashmap for only the key/value pairs we choose to accept from the user

    /*
        create another hashmap that will contain the key/value pairs we choose
        to accept from the user
    */
    let mut map = HashMap::new();

    let mut errors = vec![];

    /*
       iterate over all the allowed keys and add errors for any ones that
       are missing
    */
    for key in keys {
        /* if we can find it in the json, copy it to the hashmap */
        match lookup.get(*key) {
            Some(val) => {
                map.insert(*key, val.clone());
            }
            /* if not, add it to the error vector */
            None => {
                errors.push(BadPayload {
                    error: format!("key {} is missing.", *key),
                    field: key.to_string(),
                });
            }
        };
    }

    match errors.is_empty() {
        true => Ok(map),
        false => Err(errors.clone()),
    }
}
