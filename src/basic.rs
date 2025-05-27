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
    let mut valid_path = path;
    let mut current = get_mut(json, path);
    while current.is_none() {
        let mut next = valid_path;
        for (i, c) in valid_path.char_indices().rev() {
            next = &valid_path[..i];
            if c == '.' {
                break;
            }
        }
        valid_path = next;
        if valid_path.is_empty() {
            // Brave new data path in json
            current = Some(json);
        } else {
            current = get_mut(json, valid_path);
        }
    }
    let Some(diff) = path.strip_prefix(valid_path) else {
        return Err(crate::Error::ShitHappens(path.to_string()));
    };
    let fin = if diff.is_empty() {
        current
    } else {
        current.and_then(|start| {
            diff.strip_prefix('.')
                .unwrap_or(diff)
                .split('.')
                .try_fold(start, |a, b| match a {
                    Value::Array(arr) => {
                        let i = b.parse().ok()?;
                        while arr.len() <= i {
                            arr.push(Value::Null);
                        }
                        arr.get_mut(i)
                    }
                    Value::Object(map) => {
                        map.insert(b.to_string(), Value::Null);
                        map.get_mut(b)
                    }
                    _ => match b.parse::<usize>() {
                        Ok(i) => {
                            *a = Value::Array(vec![Value::Null; i + 1]);
                            a.get_mut(i)
                        }
                        Err(_) => {
                            *a = json!({});
                            a[b] = Value::Null;
                            a.get_mut(b)
                        }
                    },
                })
        })
    };
    if let Some(res) = fin {
        *res = value;
        Ok(())
    } else {
        Err(crate::Error::InvalidDataPath(path.to_string()))
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
