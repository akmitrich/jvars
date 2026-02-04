# JVars
Simple tools to deal with JSON values via data paths.

## Dependencies
The crate has `DataPathExt` trait implementation for 
- `serde_json::Value`.

Pull requests are welcomed.

## Data path
Let's say we have
```json
{
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
}
```

Special case of empty path is the whole JSON itself.

### Get data
Examples of paths:
- `registered` -> "2018-07-24T06:26:18 -03:00"
- `latitude` -> -1.198644
- `tags.3` -> "irure"
- `friends.1` -> 
```json
{
    "id": 1,
    "name": "Fuentes Carroll"
}
```
- `friends.1.id` -> 1
- `friends.1.name` -> "Fuentes Carroll"
- `tags` ->
```json
[
    "non",
    "aute",
    "amet",
    "irure",
    "officia",
    "ea",
    "cillum"
]
```
- `` ->
```json
{
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
}
```

Hope you see the idea of data path clearly.

### Set data
Update value in path `friends.2.name="Мама"`. `friends` array becomes
```json
[
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
    "name": "Мама"
  }
]
```

Create value in path `friends.3.name="Юлия"`. `friends` array becomes
```json
[
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
    "name": "Мама"
  },
  {
    "name": "Юлия"
  }
]
```

Creating value inside array you expect that enough `null`s will be added. Let's take
```json
{
  "array1": []
}
```
and create `5` in path `array1.5`. We end up with
```json
{
  "array1": [ null, null, null, null, null, 5 ]
}
```

I hope you get the idea behind `update_or_create` function.

## Some code snippets
```rust
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
```

```rust
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
```

```rust
let mut data = json!({});
data.update_or_create("a.b.c.d.e.f", 543.into()).unwrap();
println!("{}", serde_json::to_string_pretty(&data).unwrap());
```
Output for above snippet:
```json
{
  "a": {
    "b": {
      "c": {
        "d": {
          "e": {
            "f": 543
          }
        }
      }
    }
  }
}
```

```rust
let mut data = json!({});
data.update_or_create("array1", json!([])).unwrap();
assert!(data["array1"].is_array());
let n = 5;
data.update_or_create(format!("array1.{}", n), true.into())
    .unwrap();
println!("{}", serde_json::to_string_pretty(&data).unwrap());
```

Output is
```json
{
  "array1": [
    null,
    null,
    null,
    null,
    null,
    true
  ]
}
```