use serde_json::{Map, Value};

pub trait Raw {
    fn raw(&self) -> &Value;
}

pub trait Property {
    fn get_property(&self, full_path: &str) -> Option<&Value>;
    fn get_str_property(&self, full_path: &str) -> Option<&str>;
    fn get_string_property(&self, full_path: &str) -> Option<String>;
    fn get_int_property(&self, full_path: &str) -> Option<i64>;
    fn get_float_property(&self, full_path: &str) -> Option<f64>;
    fn get_array_property(&self, full_path: &str) -> Option<&Vec<Value>>;
    fn get_object_property(&self, full_path: &str) -> Option<&Map<String, Value>>;
}

impl<T> Property for T
where
    T: Raw,
{
    fn get_property(&self, full_path: &str) -> Option<&Value> {
        if full_path.is_empty() {
            return Some(self.raw());
        }

        let mut paths = full_path.split(".");
        let mut cur_raw = self.raw();

        while let Some(path) = paths.next() {
            if cur_raw.is_array() {
                if let Some(idx) = path.parse::<usize>().ok() {
                    match cur_raw.get(idx) {
                        Some(new_raw) => cur_raw = new_raw,
                        None => return None,
                    };
                    continue;
                }
            }

            match cur_raw.get(path) {
                Some(new_raw) => cur_raw = new_raw,
                None => return None,
            }
        }

        Some(cur_raw)
    }

    fn get_str_property(&self, full_path: &str) -> Option<&str> {
        self.get_property(full_path).and_then(|v| v.as_str())
    }

    fn get_string_property(&self, full_path: &str) -> Option<String> {
        self.get_property(full_path)
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
    }

    fn get_int_property(&self, full_path: &str) -> Option<i64> {
        self.get_property(full_path)
            .and_then(|v| v.as_i64())
            .or(self
                .get_property(full_path)
                .and_then(|v| v.as_f64())
                .map(|v| v as i64))
    }

    fn get_float_property(&self, full_path: &str) -> Option<f64> {
        self.get_property(full_path)
            .and_then(|v| v.as_f64())
            .or(self
                .get_property(full_path)
                .and_then(|v| v.as_i64())
                .map(|v| v as f64))
    }

    fn get_array_property(&self, full_path: &str) -> Option<&Vec<Value>> {
        self.get_property(full_path).and_then(|v| v.as_array())
    }

    fn get_object_property(&self, full_path: &str) -> Option<&Map<String, Value>> {
        self.get_property(full_path).and_then(|v| v.as_object())
    }
}

impl Raw for Value {
    fn raw(&self) -> &Value {
        self
    }
}
