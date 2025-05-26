# JVars
Simple tools to deal with JSON values via data paths

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

Hope you see the idea of data path clearly.

### Set data