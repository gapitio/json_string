use num::Zero;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum JsonContext {
    Array,
    Object,
    Value,
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn parse_json_string(string: &str, json_context: JsonContext) -> String {
    let new_string = match string {
        string
            if string.starts_with('{')
                && string.ends_with('}')
                && json_context == JsonContext::Value =>
        {
            let handled_object_w_wrapper = handle_object_w_wrapper(string);
            handled_object_w_wrapper
        }
        string if json_context == JsonContext::Array => {
            let handled_array_content = handle_array_content(string);
            handled_array_content
        }
        string if string.starts_with('[') && string.ends_with(']') => {
            let handled_array_w_wrapper = handle_array_w_wrapper(string);
            handled_array_w_wrapper
        }
        string if json_context == JsonContext::Object => {
            let handled_object_content = handle_object_content(string);
            handled_object_content
        }
        _ => {
            let formatted_value = format_value(string);
            formatted_value
        }
    };

    new_string
}

pub(crate) fn handle_object_w_wrapper(string: &str) -> String {
    let mut content_string = string[1..].to_string();
    content_string.pop();
    let content_string = content_string.trim_matches([' ', '\n', '\t', ',']);

    let object_context = JsonContext::Object;

    let new_object_substance = parse_json_string(content_string, object_context);

    let new_with_braces = format!("{{{new_object_substance}}}");

    new_with_braces
}

pub(crate) fn handle_array_content(string: &str) -> String {
    let mut array_elements = split_array_elements(string)
        .iter()
        .map(|element| {
            let value_context = JsonContext::Value;

            let new_element = parse_json_string(element, value_context);
            let formatted_element = format!("{new_element}, ");
            formatted_element
        })
        .collect::<String>();
    array_elements.pop();
    array_elements.pop();

    array_elements
}

pub(crate) fn handle_array_w_wrapper(string: &str) -> String {
    let mut content_str = string[1..].to_string();
    content_str.pop();
    let content_str = content_str.trim();

    let mut array_str = split_array_elements(content_str)
        .iter()
        .map(|element| {
            let value_context = JsonContext::Value;

            let new_element = parse_json_string(element, value_context);
            let formatted_element = format!("{new_element}, ");
            formatted_element
        })
        .collect::<String>();
    array_str.pop();
    array_str.pop();

    let add_the_brackets_back = format!("[{array_str}]");

    add_the_brackets_back
}

pub(crate) fn handle_object_content(string: &str) -> String {
    let mut key_value_pairs = split_object_elements(string)
        .iter()
        .filter_map(|kv_pair| {
            let (key, value) = kv_pair.split_once(':')?;

            let trimmed_key = key.trim();

            let new_key = if trimmed_key.starts_with('\"') && trimmed_key.ends_with('\"') {
                trimmed_key.to_string()
            } else {
                format!("\"{trimmed_key}\"")
            };

            let trimmed_value = value.trim_matches([' ', '\n', '\t', ',']);

            let value_context = JsonContext::Value;

            let new_value = parse_json_string(trimmed_value, value_context);

            let new_kv_pair = format!("{new_key}: {new_value}, ");

            Some(new_kv_pair)
        })
        .collect::<String>();
    key_value_pairs.pop();
    key_value_pairs.pop();

    key_value_pairs
}

pub(crate) fn format_value(value_str: &str) -> String {
    let formatted_value = match value_str.to_lowercase().as_str() {
        value_str
            if value_str.parse::<bool>().is_ok()
                || value_str.parse::<f64>().is_ok()
                || value_str.parse::<u64>().is_ok()
                || value_str.parse::<i64>().is_ok()
                || value_str.starts_with('[') && value_str.ends_with(']')
                || value_str.starts_with('{') && value_str.ends_with('}')
                || value_str.is_empty() =>
        {
            value_str.to_string()
        }
        "none" => "None".to_string(),
        _ => {
            let without_quotes = value_str.trim_matches('\"');
            let with_quotes = format!("\"{without_quotes}\"");
            with_quotes
        }
    };

    formatted_value
}

pub(crate) fn split_object_elements(object_str: &str) -> Vec<String> {
    let mut all_elements = Vec::new();
    let mut current_element = String::default();
    let mut array_lefts = 0;
    let mut object_lefts = 0;

    for ch in object_str.chars() {
        if ch == ',' && array_lefts.is_zero() && object_lefts.is_zero() {
            let trimmed_element = current_element
                .trim_matches([' ', '\n', '\t', ','])
                .to_string();
            all_elements.push(trimmed_element);
            current_element.clear();
        }

        if ch == '{' && array_lefts.is_zero() && object_lefts.is_zero() {
            object_lefts += 1;
        }

        if ch == '[' && array_lefts.is_zero() && object_lefts.is_zero() {
            array_lefts += 1;
        }

        if ch == ']' && !array_lefts.is_zero() {
            array_lefts -= 1;
        }

        if ch == '}' && !object_lefts.is_zero() {
            object_lefts -= 1;
        }
        current_element.push(ch);
    }

    let trimmed_element = current_element
        .trim_matches([' ', '\n', '\t', ','])
        .to_string();
    all_elements.push(trimmed_element);
    current_element.clear();

    all_elements
}

pub(crate) fn split_array_elements(string: &str) -> Vec<String> {
    let mut all_elements = Vec::new();
    let mut current_element = String::default();
    let mut array_lefts = 0;
    let mut object_lefts = 0;

    for ch in string.chars() {
        if ch == ',' && array_lefts.is_zero() && object_lefts.is_zero() {
            let trimmed_current_element = current_element
                .trim_matches([' ', '\n', '\t', ',', ';'])
                .to_string();
            all_elements.push(trimmed_current_element.clone());
            current_element.clear();
        }

        if ch == '{' && array_lefts.is_zero() {
            object_lefts += 1;
        }

        if ch == '[' && object_lefts.is_zero() {
            array_lefts += 1;
        }

        if ch == ']' && !array_lefts.is_zero() {
            array_lefts -= 1;
        }

        if ch == '}' && !object_lefts.is_zero() {
            object_lefts -= 1;
        }
        current_element.push(ch);
    }

    let trimmed_current_element = current_element
        .trim_matches([' ', '\n', '\t', ',', ';'])
        .to_string();
    all_elements.push(trimmed_current_element.clone());
    current_element.clear();

    all_elements
}

#[cfg(test)]
mod tests {
    use crate::json_parse_approach::{parse_json_string, JsonContext};

    #[test]
    fn parse_json_string_test() {
        let original_str = r#"{
            [{unit_id: 5, ec_id: 0,"label": "SOMETHING","uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"] }],
        }"#;

        let unknown_context = JsonContext::Value;

        let prepared_str = parse_json_string(original_str, unknown_context);

        let expected_str = r#"{[{"unit_id": 5, "ec_id": 0, "label": "SOMETHING", "uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"]}]}"#.to_string();

        assert_eq!(prepared_str, expected_str);
    }
}
