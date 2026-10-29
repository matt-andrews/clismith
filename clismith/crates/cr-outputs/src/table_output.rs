use serde_json::Value;
use std::collections::HashMap;

use crate::OutputShape;

pub struct TableOutput {}
impl OutputShape for TableOutput {
    fn print_output(&self, content: Value) {
        let result = render_table(&to_tabular(&content));
        println!("{}", result);
    }
}

struct Tabular {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

fn render_table(t: &Tabular) -> String {
    let mut table = comfy_table::Table::new();
    table.set_header(&t.headers);
    for r in &t.rows {
        table.add_row(r);
    }
    table.to_string()
}

fn flatten(prefix: &str, v: &Value, out: &mut HashMap<String, String>) {
    match v {
        Value::Object(map) => {
            for (k, child) in map {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                flatten(&key, child, out);
            }
        }
        _ => {
            let key = if prefix.is_empty() {
                "value".to_string()
            } else {
                prefix.to_string()
            };
            let s = match v {
                Value::String(s) => s.clone(),
                Value::Null => String::new(),
                other => other.to_string(), // numbers, bools, arrays
            };
            out.insert(key, s);
        }
    }
}

fn to_tabular(v: &Value) -> Tabular {
    let items: Vec<&Value> = match v {
        Value::Array(a) => a.iter().collect(),
        other => vec![other],
    };

    let mut headers: Vec<String> = Vec::new();
    let mut flat_rows = Vec::with_capacity(items.len());
    for item in items {
        let mut flat = HashMap::new();
        flatten("", item, &mut flat);
        for k in flat.keys() {
            if !headers.contains(k) {
                headers.push(k.clone());
            }
        }
        flat_rows.push(flat);
    }

    // Objects missing a key just get an empty cell
    let rows = flat_rows
        .iter()
        .map(|f| {
            headers
                .iter()
                .map(|h| f.get(h).cloned().unwrap_or_default())
                .collect()
        })
        .collect();

    Tabular { headers, rows }
}
