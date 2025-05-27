use serde_json::Value;

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
}
