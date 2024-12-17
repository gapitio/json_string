use crate::helpers::{
    format_stringified_value, handle_stringified_array_content, handle_stringified_array_w_wrapper,
    handle_stringified_object_content, handle_stringified_object_w_wrapper, JsonContext,
};

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn parse_stringified_json_string(string: &str, json_context: JsonContext) -> String {
    let new_string = match string {
        string
            if string.starts_with('{')
                && string.ends_with('}')
                && json_context == JsonContext::Value =>
        {
            let handled_object_w_wrapper = handle_stringified_object_w_wrapper(string);

            handled_object_w_wrapper
        }
        string if json_context == JsonContext::Array => {
            let handled_array_content = handle_stringified_array_content(string);

            handled_array_content
        }
        string if string.starts_with('[') && string.ends_with(']') => {
            let handled_array_w_wrapper = handle_stringified_array_w_wrapper(string);

            handled_array_w_wrapper
        }
        string if json_context == JsonContext::Object => {
            let handled_object_content = handle_stringified_object_content(string);

            handled_object_content
        }
        _ => {
            let formatted_value = format_stringified_value(string);

            formatted_value
        }
    };

    new_string
}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::JsonContext, public::parse_stringified_json_string::parse_stringified_json_string,
    };

    #[test]
    fn array_w_multiple_objects_and_semicolon_separator_stringified() {}

    #[test]
    fn array_w_multiple_objects_stringified() {
        let original_str = r#"[
            {"Foo1":"BAR1", "Foo2":"BAR2", "Foo3":"BAR3"},
            {"Foo4":"BAR4", "Foo5":"BAR5", "Foo6":"BAR6"};
            {"Foo7":"BAR7", "Foo8":"BAR8", "Foo9":"BAR9"}
        ]"#;

        let value_context = JsonContext::Value;
        let prepared_str = parse_stringified_json_string(original_str, value_context);

        let expected_str = r#"[{"Foo1": "BAR1", "Foo2": "BAR2", "Foo3": "BAR3"}, {"Foo4": "BAR4", "Foo5": "BAR5", "Foo6": "BAR6"}, {"Foo7": "BAR7", "Foo8": "BAR8", "Foo9": "BAR9"}]"#
        .to_string();

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn parse_stringified_json_string_test() {
        let original_str = r#"{
            [{unit_id: 5, ec_id: 0,"label": "SOMETHING","uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"] }],
        }"#;

        let value_context = JsonContext::Value;

        let prepared_str = parse_stringified_json_string(original_str, value_context);

        let expected_str = r#"{[{"unit_id": "5", "ec_id": "0", "label": "SOMETHING", "uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"]}]}"#.to_string();

        assert_eq!(prepared_str, expected_str);
    }
}
