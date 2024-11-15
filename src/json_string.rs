use indexmap::IndexMap;
use itertools::Itertools;
use num::Zero;

pub fn prepare_json_string(original_str: &str) -> String {
    let raw_json_multiple_devices = raw_json_multiple_devices(original_str);

    let intermediate_json_multiple_devices =
        intermediate_json_multiple_devices(&raw_json_multiple_devices);

    let prepare_unformatted_json_string =
        prepare_unformatted_json_string(&intermediate_json_multiple_devices);

    let removed_trailing_comma = prepare_unformatted_json_string.trim_end_matches([',', ' ']);
    let added_brackets = format!("[{removed_trailing_comma}]");

    added_brackets
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum KeyOrValue {
    Key,
    Value,
}

pub(crate) fn prepare_unformatted_json_string(
    intermediate_json_multiple_devices: &[String],
) -> String {
    let prepared_json_string = intermediate_json_multiple_devices
        .iter()
        .filter(|json_string_one_device| !json_string_one_device.is_empty())
        .filter(|json_string_one_device| *json_string_one_device != ";")
        .map(|json_string_one_device| {
            let prepared_json_string_one_device =
                prepare_json_string_for_one_device(json_string_one_device);

            let add_commas = format!("{prepared_json_string_one_device}, ");

            add_commas
        })
        .collect::<String>();

    prepared_json_string
}

pub(crate) fn handle_quote_char_for_quote_distinguisher(
    inside_quoteless_key: &mut bool,
    inside_quoteless_value: &mut bool,
    inside_quote: &mut bool,
    is_key: &mut bool,
    string_under_construction: &mut String,
    quote_distinguisher: &mut IndexMap<String, KeyOrValue>,
) {
    *inside_quoteless_key = false;
    *inside_quoteless_value = false;
    if *inside_quote && !string_under_construction.is_empty() {
        if *is_key {
            quote_distinguisher.insert(string_under_construction.clone(), KeyOrValue::Key);
        } else {
            quote_distinguisher.insert(string_under_construction.clone(), KeyOrValue::Value);
        };

        string_under_construction.clear();
        *is_key = !(*is_key);
    }

    *inside_quote = !(*inside_quote);
}

pub(crate) fn quote_distinguisher_from_str(trimmed_braces: &str) -> IndexMap<String, KeyOrValue> {
    let mut quote_distinguisher: IndexMap<String, KeyOrValue> = IndexMap::new();
    let mut inside_quote = false;
    let mut inside_quoteless_key = true;
    let mut inside_quoteless_value = false;
    let mut is_key = true;
    let mut left_brace_count = 0;
    let mut left_bracket_count = 0;

    let mut string_under_construction = String::new();

    for ch in trimmed_braces.chars() {
        if ch == '\"' && left_brace_count.is_zero() {
            handle_quote_char_for_quote_distinguisher(
                &mut inside_quoteless_key,
                &mut inside_quoteless_value,
                &mut inside_quote,
                &mut is_key,
                &mut string_under_construction,
                &mut quote_distinguisher,
            );
        } else if inside_quote {
            string_under_construction.push(ch);
        } else if ch == ',' && left_brace_count.is_zero() {
            if inside_quoteless_value {
                quote_distinguisher.insert(string_under_construction.clone(), KeyOrValue::Value);
                string_under_construction.clear();
            }
            inside_quoteless_key = true;
            inside_quoteless_value = false;
            is_key = true;
        } else if ch == ':' && left_brace_count.is_zero() {
            if inside_quoteless_key {
                quote_distinguisher.insert(string_under_construction.clone(), KeyOrValue::Key);
                string_under_construction.clear();
            }
            is_key = false;
            inside_quoteless_key = false;
            inside_quoteless_value = true;
        } else if ch == '{' {
            left_brace_count += 1;
            string_under_construction.push(ch);
        } else if ch == '}' {
            left_brace_count -= 1;

            if left_brace_count.is_zero() && inside_quoteless_value {
                string_under_construction.push(ch);
                quote_distinguisher.insert(string_under_construction.clone(), KeyOrValue::Value);
                string_under_construction.clear();
            }
        } else if ch == '[' {
            left_bracket_count += 1;
            string_under_construction.push(ch);
        } else if ch == ']' {
            left_bracket_count -= 1;

            if left_bracket_count.is_zero() && inside_quoteless_value {
                string_under_construction.push(ch);
                quote_distinguisher.insert(string_under_construction.clone(), KeyOrValue::Value);
                string_under_construction.clear();
            }
        } else {
            string_under_construction.push(ch);
        }
    }

    quote_distinguisher.insert(string_under_construction.clone(), KeyOrValue::Value);
    string_under_construction.clear();

    quote_distinguisher
}

#[allow(clippy::too_many_lines)]
pub(crate) fn prepare_json_string_for_one_device(original_str_one_device: &str) -> String {
    let trimmed_braces =
        if original_str_one_device.starts_with('{') && original_str_one_device.ends_with('}') {
            let mut without_first_and_last_char = original_str_one_device[1..].to_string();
            without_first_and_last_char.pop();
            without_first_and_last_char.to_string()
        } else {
            original_str_one_device.to_string()
        };

    let quote_distinguisher = quote_distinguisher_from_str(&trimmed_braces);

    let distinguisher_without_empty_str = quote_distinguisher
        .into_iter()
        .filter(|(key, _)| !key.is_empty())
        .collect::<IndexMap<String, KeyOrValue>>();

    let compressed_distinguisher = compress_distinguisher(&distinguisher_without_empty_str);

    let filtered_distinguisher = compressed_distinguisher
        .into_iter()
        .filter(|(_, value)| *value != "[]")
        .map(|(key, value)| {
            let trimmed_key = key.trim().to_string();
            let trimmed_value = value.trim().to_string();
            (trimmed_key, trimmed_value)
        })
        .collect::<IndexMap<String, String>>();

    let prepared_json_string =
        prepare_json_string_from_quote_distinguisher(&filtered_distinguisher);

    prepared_json_string
}

pub(crate) fn prepare_json_string_from_quote_distinguisher(
    quote_distinguisher: &IndexMap<String, String>,
) -> String {
    let stringified_quote_distinguisher = quote_distinguisher
        .iter()
        .map(|(key, value)| {
            let new_key = if let Ok(u64_entry) = key.parse::<u64>() {
                format!("{u64_entry:?}: ")
            } else if let Ok(f64_entry) = key.parse::<f64>() {
                format!("{f64_entry:?}: ")
            } else if let Ok(hashmap) = serde_json::from_str::<IndexMap<String, String>>(key) {
                format!("{hashmap:?}: ")
            } else {
                format!("\"{key}\": ")
            };

            let new_value = if let Ok(u64_entry) = value.parse::<u64>() {
                format!("{u64_entry:?}, ")
            } else if let Ok(f64_entry) = value.parse::<f64>() {
                format!("{f64_entry:?}, ")
            } else if let Ok(hashmap) = serde_json::from_str::<IndexMap<String, String>>(value) {
                format!("{hashmap:?}, ")
            } else if value.starts_with('[') && value.ends_with(']') {
                format!("{value}, ")
            } else {
                format!("\"{value}\", ")
            };

            let entry_string = format!("{new_key}{new_value}");

            entry_string
        })
        .collect::<String>();

    let removed_trailing_comma = stringified_quote_distinguisher.trim_matches([',', ' ']);
    let added_curly_braces = format!("{{{removed_trailing_comma}}}");

    added_curly_braces
}

pub(crate) fn compress_distinguisher(
    distinguisher_without_empty_str: &IndexMap<String, KeyOrValue>,
) -> IndexMap<String, String> {
    let clean_distinguisher = distinguisher_without_empty_str
        .iter()
        .map(|(key, _)| key)
        .collect_vec();

    let compressed_distinguisher = clean_distinguisher
        .chunks_exact(2)
        .map(|chunk| {
            let key = chunk[0].to_string();
            let value = chunk[1].to_string();
            (key, value)
        })
        .collect::<IndexMap<String, String>>();

    compressed_distinguisher
}

pub(crate) fn intermediate_json_multiple_devices(
    raw_json_multiple_devices: &[String],
) -> Vec<String> {
    let intermediate_json_multiple_devices = raw_json_multiple_devices
        .iter()
        .filter(|customtag_string_one_device| !customtag_string_one_device.is_empty())
        .map(|customtags_one_device| {
            let trimmed_ends = customtags_one_device.trim_matches(['\n', ' ', ',']);
            trimmed_ends.to_string()
        })
        .collect_vec();

    intermediate_json_multiple_devices
}

pub(crate) fn raw_json_multiple_devices(original_str: &str) -> Vec<String> {
    let trimmed_ends = original_str.trim_matches(['\n', ' ', '[', ']']);
    let mut raw_json_multiple_devices = vec![];
    let mut raw_json_one_device_under_construction = String::new();

    let mut left_brace_count = 0;

    for ch in trimmed_ends.chars() {
        raw_json_one_device_under_construction.push(ch);

        if ch == '{' {
            left_brace_count += 1;
        } else if ch == '}' {
            left_brace_count -= 1;
        }

        if !raw_json_one_device_under_construction.is_empty() && left_brace_count.is_zero() {
            raw_json_multiple_devices.push(raw_json_one_device_under_construction.clone());
            raw_json_one_device_under_construction.clear();
        }
    }

    raw_json_multiple_devices
}

#[cfg(test)]
mod tests {
    use crate::prepare_json_string;

    #[test]
    fn prepare_json_string_w_empty_groups() {
        let original_str = r#"
            [
                {"tag":"lol", "groups":[]}
            ]
        "#;
        let prepared_str = prepare_json_string(original_str);

        let expected_str = r#"[{"tag": "lol"}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn prepare_json_string_w_inner_json() {
        let original_str = r#"
            [
                {"label":"lol", "customtags": {"k1": "v1"}}
            ]
        "#;
        let prepared_str = prepare_json_string(original_str);

        let expected_str = r#"[{"label": "lol", "customtags": {"k1": "v1"}}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn entirely_without_quotes() {
        let original_str = "{
            property1: lol,
            property2: iskrem
        }";

        let prepared_str = prepare_json_string(original_str);

        let expected_str = "[{\"property1\": \"lol\", \"property2\": \"iskrem\"}]";

        assert_eq!(prepared_str, expected_str);
    }
}
