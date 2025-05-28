use serde_json::{Value, json};

pub fn get(json: &Value, path: impl AsRef<str>) -> Option<&Value> {
    path.as_ref()
        .split('.')
        .try_fold(json, |sub_value, next| match sub_value {
            Value::Array(a) => a.get(next.parse::<usize>().ok()?),
            _ => sub_value.get(next),
        })
}

pub fn get_mut(json: &mut Value, path: impl AsRef<str>) -> Option<&mut Value> {
    path.as_ref()
        .split('.')
        .try_fold(json, |sub_value, next| match sub_value {
            Value::Array(a) => a.get_mut(next.parse::<usize>().ok()?),
            _ => sub_value.get_mut(next),
        })
}

pub fn update_or_create(
    json: &mut Value,
    path: impl AsRef<str>,
    value: Value,
) -> crate::Result<()> {
    let path = path.as_ref();
    let mut current_path = path;
    let mut current_value = get_mut(json, path);
    while current_value.is_none() {
        let mut next = current_path;
        for (i, c) in current_path.char_indices().rev() {
            next = &current_path[..i];
            if c == '.' {
                break;
            }
        }
        current_path = next;
        if current_path.is_empty() {
            // Brave new data path in json
            current_value = Some(json);
        } else {
            current_value = get_mut(json, current_path);
        }
    }
    let Some(diff) = path.strip_prefix(current_path) else {
        return Err(crate::Error::Impossible(path.to_string()));
    };
    if let Some(dst) =
        create_destination_if_needed(current_value, diff.strip_prefix('.').unwrap_or(diff))
    {
        *dst = value;
        Ok(())
    } else {
        Err(crate::Error::InvalidDataPath(path.to_string()))
    }
}

fn create_destination_if_needed<'a>(
    valid: Option<&'a mut Value>,
    rest_path: &str,
) -> Option<&'a mut Value> {
    if rest_path.is_empty() {
        valid
    } else {
        valid.and_then(|start| {
            rest_path.split('.').try_fold(start, |a, b| match a {
                Value::Array(arr) => {
                    // in array index must be `usize`
                    let i = b.parse().ok()?;
                    while arr.len() <= i {
                        // make `i` to be a valid index inside the array
                        arr.push(Value::Null);
                    }
                    arr.get_mut(i)
                }
                Value::Object(map) => {
                    map.insert(b.to_string(), Value::Null);
                    map.get_mut(b)
                }
                _ => {
                    if let Ok(i) = b.parse::<usize>() {
                        // index is `usize` then create an array and enough nulls inside it
                        *a = Value::Array(vec![Value::Null; i + 1]);
                        a.get_mut(i)
                    } else {
                        // else create an object and put null for key `b`
                        *a = json!({});
                        a[b] = Value::Null;
                        a.get_mut(b)
                    }
                }
            })
        })
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
    fn update_and_create() {
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
}
