use jvars::basic;
use serde_json::{Value, json};

#[test]
fn use_arrow_separator() {
    jvars::PATH_SEPARATOR.change("->");
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
    let id = basic::get(&data, "friends->0->id").unwrap();
    assert_eq!(0, id.as_i64().unwrap());
    let name = basic::get_mut(&mut data, "friends->1->name").unwrap();
    *name = json!(42);
    assert_eq!(42, data["friends"][1]["name"]);
    basic::update_or_create(&mut data, "friends->3->name", "Марго".into()).unwrap();
    assert_eq!(Value::Null, data["friends"][2]);
    assert!(basic::get(&data, "friends->3->id").is_none());
    assert_eq!(Some(Value::Null), basic::delete(&mut data, "friends->2"));
    assert_eq!("Марго", basic::get(&data, "friends->2->name").unwrap());
}
