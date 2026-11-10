use std::cmp::Ordering;

use serde_json::{Map, Value};

use crate::runtime::invocation::{Invocation, SYS_FLAG_OUTPUT_PROP, SYS_FLAG_OUTPUT_SORT};

pub fn result_preprocess(output: &mut Value, inv: &Invocation) {
    //
    let filter_props = inv.list(SYS_FLAG_OUTPUT_PROP);
    if let Some(filter_props) = filter_props {
        select_fields(output, &filter_props);
    }

    let sort_props = inv.list(SYS_FLAG_OUTPUT_SORT);
    if let Some(sort_props) = sort_props {
        sort_results(output, &sort_props);
    }
}

fn select_fields(value: &mut Value, fields: &[String]) {
    match value {
        Value::Object(map) => {
            let mut picked = Map::new();
            for f in fields {
                if let Some(v) = map.remove(f) {
                    picked.insert(f.clone(), v);
                }
            }
            *map = picked;
        }
        Value::Array(items) => items
            .iter_mut()
            .for_each(|item| select_fields(item, fields)),
        _ => {}
    }
}

fn sort_results(value: &mut Value, sort: &[String]) {
    let keys: Vec<(&str, bool)> = sort
        .iter()
        .map(|k| match k.strip_prefix('-') {
            Some(field) => (field, true),
            None => (k.as_str(), false),
        })
        .collect();

    if let Value::Array(items) = value {
        items.sort_by(|a, b| {
            keys.iter()
                .map(|&(key, desc)| {
                    let ord = compare_values(a.get(key), b.get(key));
                    if desc { ord.reverse() } else { ord }
                })
                .find(|ord| ord.is_ne())
                .unwrap_or(Ordering::Equal)
        });
    }
}

fn compare_values(a: Option<&Value>, b: Option<&Value>) -> Ordering {
    fn rank(v: Option<&Value>) -> u8 {
        match v {
            None | Some(Value::Null) => 0,
            Some(Value::Bool(_)) => 1,
            Some(Value::Number(_)) => 2,
            Some(Value::String(_)) => 3,
            Some(Value::Array(_)) => 4,
            Some(Value::Object(_)) => 5,
        }
    }
    match (a, b) {
        (Some(Value::Bool(x)), Some(Value::Bool(y))) => x.cmp(y),
        (Some(Value::Number(x)), Some(Value::Number(y))) => x
            .as_f64()
            .unwrap_or(0.0)
            .total_cmp(&y.as_f64().unwrap_or(0.0)),
        (Some(Value::String(x)), Some(Value::String(y))) => compare_strings(x, y),
        _ => rank(a).cmp(&rank(b)),
    }
}

fn compare_strings(x: &str, y: &str) -> Ordering {
    x.chars()
        .flat_map(char::to_lowercase)
        .cmp(y.chars().flat_map(char::to_lowercase))
        .then_with(|| x.cmp(y))
}
