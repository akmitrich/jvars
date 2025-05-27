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
}
