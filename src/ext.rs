use serde_json::Value;

/// Wheelhorse of the crate.
/// Implemented for `serde_json::Value`.
/// So you just place it in your scope and use the provided methods.
pub trait DataPathExt {
    type Value;

    /// Get shared reference to the value in `path` inside `self`
    fn path<A: AsRef<str>>(&self, path: impl IntoIterator<Item = A>) -> Option<&Self::Value>;
    /// Get exclusive reference to the value in `path` inside `self`
    fn path_mut<A: AsRef<str>>(
        &mut self,
        path: impl IntoIterator<Item = A>,
    ) -> Option<&mut Self::Value>;
    /// Update the `path` inside `self` with `value` or create the `path` if it does not exist and place `value` in it
    fn update_or_create<A: AsRef<str>>(
        &mut self,
        path: impl IntoIterator<Item = A>,
        value: Self::Value,
    ) -> crate::Result<()>;
    /// Delete the value in the `path` and return it; returns None if there is no value in the `path`
    fn delete<A: AsRef<str>>(&mut self, path: impl IntoIterator<Item = A>) -> Option<Self::Value>;
}

impl DataPathExt for Value {
    type Value = Self;

    fn path<A: AsRef<str>>(&self, path: impl IntoIterator<Item = A>) -> Option<&Self::Value> {
        path.into_iter().try_fold(self, |sub_value, next| {
            let next = next.as_ref();
            match sub_value {
                Value::Array(a) => a.get(next.parse::<usize>().ok()?),
                _ => sub_value.get(next),
            }
        })
    }

    fn path_mut<A: AsRef<str>>(
        &mut self,
        path: impl IntoIterator<Item = A>,
    ) -> Option<&mut Self::Value> {
        path.into_iter().try_fold(self, |sub_value, next| {
            let next = next.as_ref();
            match sub_value {
                Value::Array(a) => a.get_mut(next.parse::<usize>().ok()?),
                sub_value => sub_value.get_mut(next),
            }
        })
    }

    fn update_or_create<A: AsRef<str>>(
        &mut self,
        path: impl IntoIterator<Item = A>,
        value: Self::Value,
    ) -> crate::Result<()> {
        let path_buf = path.into_iter().collect::<Vec<_>>();
        let mut current_count = path_buf.len();
        let mut current_value = self.path_mut(&path_buf);
        while current_value.is_none() {
            current_count -= 1;
            current_value = self.path_mut(&path_buf[..current_count]);
        }
        let tail = &path_buf[current_count..];
        if let Some(dst) = create_destination_if_needed(current_value, tail) {
            *dst = value;
            Ok(())
        } else {
            Err(crate::Error::InvalidDataPath(
                path_buf
                    .into_iter()
                    .map(|a| a.as_ref().to_string())
                    .collect::<Vec<_>>()
                    .join(crate::PATH_SEPARATOR.get()),
            ))
        }
    }

    fn delete<A: AsRef<str>>(&mut self, path: impl IntoIterator<Item = A>) -> Option<Self::Value> {
        let path_buf = path.into_iter().collect::<Vec<_>>();
        if path_buf.is_empty() {
            return Some(self.take());
        }
        let (head, tail) = path_buf.split_at(path_buf.len() - 1);
        let target = self.path_mut(head)?;
        let index = tail.last()?.as_ref();
        match target {
            Value::Object(map) => map.remove(index),
            Value::Array(arr) => index.parse::<usize>().ok().and_then(|n| {
                if n < arr.len() {
                    Some(arr.remove(n))
                } else {
                    None
                }
            }),
            _ => None,
        }
    }
}

fn create_destination_if_needed<'a, A: AsRef<str>>(
    valid: Option<&'a mut Value>,
    rest_path: &[A],
) -> Option<&'a mut Value> {
    valid.and_then(|start| {
        rest_path.iter().try_fold(start, |current, next_index| {
            let next = next_index.as_ref();
            match current {
                Value::Array(arr) => {
                    // in array index must be `usize`
                    let i = next.parse().ok()?;
                    while arr.len() <= i {
                        // make `i` to be a valid index inside the array
                        arr.push(Value::Null);
                    }
                    arr.get_mut(i)
                }
                Value::Object(map) => {
                    map.insert(next.to_string(), Value::Null);
                    map.get_mut(next)
                }
                _ => {
                    if let Ok(i) = next.parse::<usize>() {
                        // index is `usize` then create an array and enough nulls inside it
                        *current = Value::Array(vec![Value::Null; i + 1]);
                        current.get_mut(i)
                    } else {
                        // else create an object and put null for key `next`
                        *current = Value::Object(serde_json::Map::new());
                        current[next] = Value::Null;
                        current.get_mut(next)
                    }
                }
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn update_value() {
        let mut data = json!({"abc": {"x": 0, "y": 1}});
        data.update_or_create(["abc", "x", "polar", "0"], 42.into())
            .unwrap();
        println!("Result: {data:#}");
        data.update_or_create("".split('.'), true.into()).unwrap();
        println!("Replaced: {data:?}")
    }
}
