use crate::DataPathExt;
use serde_json::Value;

/// Get shared reference to the value in dot separated `path` inside `json`
pub fn get(json: &Value, path: impl AsRef<str>) -> Option<&Value> {
    if path.as_ref().is_empty() {
        return Some(json);
    }
    json.path(path.as_ref().split('.'))
}

/// Get exclusive reference to the value in dot separated `path` inside `json`
pub fn get_mut(json: &mut Value, path: impl AsRef<str>) -> Option<&mut Value> {
    if path.as_ref().is_empty() {
        return Some(json);
    }
    json.path_mut(path.as_ref().split('.'))
}

/// Update the `path` inside `json` with `value` or create the dot separated `path` if it does not exist and place `value` in it
pub fn update_or_create(
    json: &mut Value,
    path: impl AsRef<str>,
    value: Value,
) -> crate::Result<()> {
    if path.as_ref().is_empty() {
        *json = value;
        return Ok(());
    }
    json.update_or_create(path.as_ref().split('.'), value)
}

/// Delete the value in the `path` and return it; returns None if there is no value in the `path`
pub fn delete(json: &mut Value, path: impl AsRef<str>) -> Option<Value> {
    if path.as_ref().is_empty() {
        return Some(json.take());
    }
    let last_dot = path.as_ref().rfind('.').unwrap_or(0);
    let (path, tail) = path.as_ref().split_at(last_dot);
    let index = tail.strip_prefix(".").unwrap_or(tail);
    let target = get_mut(json, path)?;
    match target {
        Value::Object(map) => map.remove(index),
        Value::Array(arr) => index.parse::<usize>().ok().and_then(|n| {
            if n < arr.len() {
                Some(arr.remove(n))
            } else {
                None
            }
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_works() {
        let mut data = json!({
            "friends": [
              {
                "id": 0,
                "name": "Holt Stewart"
              },
              {
                "id": 1,
                "name": "Fuentes Carroll"
              },
              {
                "id": 2,
                "name": "Greta Kane"
              }
            ]
        });
        let id = get(&data, "friends.0.id").unwrap();
        assert_eq!(0, id.as_i64().unwrap());
        let name = get_mut(&mut data, "friends.1.name").unwrap();
        *name = json!(42);
        assert_eq!(42, data["friends"][1]["name"]);
    }

    #[test]
    fn unit_test_update_and_create() {
        let mut data = json!({
            "friends": [
              {
                "id": 0,
                "name": "Holt Stewart"
              },
              {
                "id": 1,
                "name": "Fuentes Carroll"
              },
              {
                "id": 2,
                "name": "Greta Kane"
              }
            ]
        });
        update_or_create(&mut data, "friends.2.name", json!("Мама")).unwrap();
        assert_eq!("Мама", data["friends"][2]["name"]);
        update_or_create(&mut data, "friends.3.id", 42.into()).unwrap();
        update_or_create(&mut data, "friends.3.name", "Юлия".into()).unwrap();
        assert_eq!(
            json!({
                "id": 42,
                "name": "Юлия"
            }),
            data["friends"][3]
        );
        update_or_create(&mut data, "", Value::Bool(true)).unwrap();
        assert!(data.as_bool().unwrap());
    }

    #[test]
    fn delete_works() {
        let mut data = json!({
            "фис": 25,
            "a": {
                "b": {
                    "c": true
                }
            }
        });
        let num = delete(&mut data, "фис");
        assert_eq!(25, num.unwrap());
        let flag = delete(&mut data, "a.b.c");
        assert!(flag.unwrap().as_bool().unwrap());
        assert_eq!(json!({"a":{"b":{}}}), data);
        let json = delete(&mut data, "");
        assert!(data.is_null());
        assert_eq!(json!({"a":{"b":{}}}), json.unwrap());
    }
}
