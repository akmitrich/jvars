//! # JVars
//! Simple tools to deal with JSON values via data paths.

mod basic;
mod error;
mod ext;

pub use basic::{get, get_mut, update_or_create};
pub use error::{Error, Result};
pub use ext::DataPathExt;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[test]
    fn access_via_path() {
        let mut data = json!({
            "friends": [
              {
                "id": 0,
                "name": "Holt Stewart"
              },
              {
                "id": 1,
                "name": "Fuentes Carroll"
              }
            ]
        });
        let id = data.path("friends.0.id").unwrap();
        assert_eq!(0, id.as_i64().unwrap());
        let name = data.path_mut("friends.1.name").unwrap();
        *name = json!(42);
        assert_eq!(42, data["friends"][1]["name"]);
    }

    #[test]
    fn extend_json() {
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
        data.update_or_create("friends.2.name", json!("Мама"))
            .unwrap();
        assert_eq!("Мама", data["friends"][2]["name"]);
        data.update_or_create("friends.3.id", 42.into()).unwrap();
        data.update_or_create("friends.3.name", "Юлия".into())
            .unwrap();
        assert_eq!(
            json!({
                "id": 42,
                "name": "Юлия"
            }),
            data["friends"][3]
        );
        data.update_or_create("", Value::Bool(true)).unwrap();
        assert!(data.as_bool().unwrap());
    }

    #[test]
    fn insert_new_objects() {
        let mut data = json!({});
        data.update_or_create("a.b.c.d.e.f", 543.into()).unwrap();
        assert_eq!(543, data["a"]["b"]["c"]["d"]["e"]["f"]);
    }

    #[test]
    fn add_enough_elements() {
        let mut data = json!({});
        data.update_or_create("array1", json!([])).unwrap();
        assert!(data["array1"].is_array());
        let n = 501;
        data.update_or_create(format!("array1.{}", n), true.into())
            .unwrap();
        for i in 0..(n - 1) {
            assert!(data["array1"][i].is_null());
        }
        assert!(data["array1"][n].as_bool().unwrap());
    }
}
