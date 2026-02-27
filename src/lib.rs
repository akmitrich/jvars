//! # JVars
//! Simple tools to deal with JSON values via data paths.

mod basic;
mod error;
mod ext;
mod path_separator;

pub use basic::{delete, get, get_mut, update_or_create};
pub use error::{Error, Result};
pub use ext::DataPathExt;
pub use path_separator::PATH_SEPARATOR;

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
        assert_eq!(&data, basic::get(&data, "").unwrap());
        let id = basic::get(&data, "friends.0.id").unwrap();
        assert_eq!(0, id.as_i64().unwrap());
        let name = basic::get_mut(&mut data, "friends.1.name").unwrap();
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
        basic::update_or_create(&mut data, "friends.2.name", "Мама".into()).unwrap();
        assert_eq!("Мама", data["friends"][2]["name"]);
        basic::update_or_create(&mut data, "friends.3.id", 42.into()).unwrap();
        basic::update_or_create(&mut data, "friends.3.name", "Юлия".into()).unwrap();
        assert_eq!(
            json!({
                "id": 42,
                "name": "Юлия"
            }),
            data["friends"][3]
        );
        basic::update_or_create(&mut data, "", Value::Bool(true)).unwrap();
        assert!(data.as_bool().unwrap());
    }

    #[test]
    fn insert_new_objects() {
        let mut data = json!({});
        basic::update_or_create(&mut data, "a.b.c.d.e.f", 543.into()).unwrap();
        assert_eq!(543, data["a"]["b"]["c"]["d"]["e"]["f"]);
    }

    #[test]
    fn add_enough_elements() {
        let mut data = json!({});
        data.update_or_create(["array1"], json!([])).unwrap();
        assert!(data["array1"].is_array());
        let n = 501;
        basic::update_or_create(&mut data, format!("array1.{}", n), true.into()).unwrap();
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
        basic::update_or_create(&mut data, "", 42.into()).unwrap();
        assert_eq!(42, data);
        basic::update_or_create(&mut data, "abc", true.into()).unwrap();
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
        println!("path separator: {}", PATH_SEPARATOR.get());
        assert!(basic::delete(&mut data, "some.non-existent.path").is_none());
        assert!(basic::delete(&mut data, "35").is_none());
        assert_eq!("apple", basic::delete(&mut data, "favoriteFruit").unwrap());
        for i in 0..3 {
            let old = basic::delete(&mut data, "friends.0").unwrap();
            assert_eq!(&i, basic::get(&old, "id").unwrap());
        }
        assert!(basic::delete(&mut data, "friends.0").is_none());
        assert!(basic::delete(&mut data, "friends.abc").is_none());
        assert!(basic::delete(&mut data, "friends.-75").is_none());
        assert_eq!("officia", basic::delete(&mut data, "tags.4").unwrap());
        assert_eq!("cillum", basic::delete(&mut data, "tags.5").unwrap());
        assert!(basic::delete(&mut data, "tags.5").is_none());
        assert!(basic::delete(&mut data, "tags.").is_none());
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
            data
        );
    }
}
