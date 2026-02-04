//! # JVars
//! Simple tools to deal with JSON values via data paths.

mod basic;
mod error;
mod ext;

pub use basic::{delete, get, get_mut, update_or_create};
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
        assert_eq!(&data, data.path("").unwrap());
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
        assert_eq!(&data.clone(), data.path_mut("").unwrap());
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
        assert!(
            data["array1"].as_array().unwrap()[..(n - 1)]
                .iter()
                .all(Value::is_null)
        );
        assert!(data["array1"][n].as_bool().unwrap());
    }

    #[test]
    fn start_from_null() {
        let mut data = Value::Null;
        data.update_or_create("", 42.into()).unwrap();
        assert_eq!(42, data);
        data.update_or_create("abc", true.into()).unwrap();
        assert_ne!(42, data);
        assert!(data["abc"].as_bool().unwrap());
    }

    #[test]
    fn some_data_is_deleted() {
        let mut data = json!({
            "registered": "2018-07-24T06:26:18 -03:00",
            "latitude": -1.198644,
            "longitude": 18.3947,
            "tags": [
              "non",
              "aute",
              "amet",
              "irure",
              "officia",
              "ea",
              "cillum"
            ],
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
            ],
            "greeting": "Hello, Sanchez Daniels! You have 1 unread messages.",
            "favoriteFruit": "apple"
        });
        assert!(data.delete("some.non-existent.path").is_none());
        assert!(data.delete("35").is_none());
        assert_eq!("apple", data.delete("favoriteFruit").unwrap());
        for i in 0..3 {
            assert_eq!(&i, data.delete("friends.0").unwrap().path("id").unwrap());
        }
        assert!(data.delete("friends.0").is_none());
        assert!(data.delete("friends.abc").is_none());
        assert!(data.delete("friends.-75").is_none());
        assert_eq!("officia", data.delete("tags.4").unwrap());
        assert_eq!("cillum", data.delete("tags.5").unwrap());
        assert!(data.delete("tags.5").is_none());
        assert!(data.delete("tags.").is_none());
        assert_eq!(
            json!({
                "registered": "2018-07-24T06:26:18 -03:00",
                "latitude": -1.198644,
                "longitude": 18.3947,
                "tags": [
                  "non",
                  "aute",
                  "amet",
                  "irure",
                  "ea",
                ],
                "friends": [],
                "greeting": "Hello, Sanchez Daniels! You have 1 unread messages.",
            }),
            dbg!(data)
        );
    }
}
