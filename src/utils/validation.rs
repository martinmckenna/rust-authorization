use actix_web::web;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct BadPayload {
    pub error: String,
    pub field: String,
}

/*
    converts JSON to a hashmap, so that the JSON keys can be validated

    e.g. convert_json_to_string({ "username": "hello" }) -> String({"username": "hello"})
*/
fn convert_json_to_hashmap(
    json: &web::Bytes,
) -> Result<HashMap<String, serde_json::Value>, Vec<BadPayload>> {
    match serde_json::from_slice(json).map_err(|error| {
        vec![BadPayload {
            field: "payload".to_string(),
            error: "Invalid payload".to_string(),
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
    /* converts JSON to hashmap */
    let json_as_hashmap = &match convert_json_to_hashmap(json) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

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
        match json_as_hashmap.get(*key) {
            Some(val) => {
                map.insert(*key, val.clone());
            }
            /* if not, add it to the error vector */
            None => {
                errors.push(BadPayload {
                    error: format!("Key {} is missing.", *key),
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
