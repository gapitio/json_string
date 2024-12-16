use crate::json_parse_approach::{parse_stringified_json_string, JsonContext};

pub fn prepare_stringified_json_string(original_str: &str) -> String {
    let trimmed_str = original_str.trim_matches([' ', '\n', '\t', ',', ';', ':']);
    let json_context = json_context(trimmed_str);
    let content_str = content_str(json_context.clone(), trimmed_str);

    let parsed_json_string = parse_stringified_json_string(&content_str, json_context.clone());

    let rewrapped_string = rewrap_string(&parsed_json_string, trimmed_str, json_context);
    rewrapped_string
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn content_str(json_context: JsonContext, trimmed_str: &str) -> String {
    let content_str = match json_context {
        JsonContext::Array | JsonContext::Object => {
            let mut content_str = trimmed_str[1..].to_string();
            content_str.pop();

            let trimmed_content_str = content_str
                .trim_matches([' ', '\n', '\t', ',', ';', ':'])
                .to_string();

            trimmed_content_str
        }
        JsonContext::Value => trimmed_str.to_string(),
    };

    content_str
}

pub(crate) fn json_context(trimmed_str: &str) -> JsonContext {
    let json_context = match trimmed_str {
        trimmed_str if trimmed_str.starts_with('{') && trimmed_str.ends_with('}') => {
            JsonContext::Object
        }
        trimmed_str if trimmed_str.starts_with('[') && trimmed_str.ends_with(']') => {
            JsonContext::Array
        }
        _ => JsonContext::Value,
    };

    json_context
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn rewrap_string(
    parsed_json_string: &str,
    trimmed_str: &str,
    json_context: JsonContext,
) -> String {
    let rewrapped_string = match json_context {
        JsonContext::Array => {
            let rewrapped_string = format!("[{parsed_json_string}]");

            rewrapped_string
        }
        JsonContext::Object => {
            let rewrapped_string = format!("{{{parsed_json_string}}}");

            rewrapped_string
        }
        JsonContext::Value => trimmed_str.to_string(),
    };

    rewrapped_string
}

#[cfg(test)]
mod tests {
    use crate::prepare_stringified_json_string;

    #[test]
    fn array_with_multiple_objects_stringified() {
        let original_str = r#"[
            {"Foo1":19, "Foo2":"BAR2", "Foo3":"BAR3"},
            {"Foo4":10, "Foo5":11, "Foo6":"BAR6"},
            {"Foo7":"BAR7", "Foo8":15, "Foo9":"BAR9"}
        ]"#;

        let prepared_str = prepare_stringified_json_string(original_str);

        let expected_str = r#"[{"Foo1": "19", "Foo2": "BAR2", "Foo3": "BAR3"}, {"Foo4": "10", "Foo5": "11", "Foo6": "BAR6"}, {"Foo7": "BAR7", "Foo8": "15", "Foo9": "BAR9"}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn prepare_stringified_json_string_w_empty_groups_stringified() {
        let original_str = r#"
            [
                {"tag":"lol", "groups":[]}
            ]
        "#;
        let prepared_str = prepare_stringified_json_string(original_str);

        let expected_str = r#"[{"tag": "lol", "groups": []}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn prepare_stringified_json_string_w_inner_json_stringified() {
        let original_str = r#"
            [
                {"label":"lol", "customtags": {"k1": 17}}
            ]
        "#;
        let prepared_str = prepare_stringified_json_string(original_str);

        let expected_str = r#"[{"label": "lol", "customtags": {"k1": "17"}}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn entirely_without_quotes_stringified() {
        let original_str = "{
            property1: 17,
            property2: iskrem
        }";

        let prepared_str = prepare_stringified_json_string(original_str);

        let expected_str = r#"{"property1": "17", "property2": "iskrem"}"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn arbitrary_test_stringified() {
        let original_str = r#"{
            [{"unit_id": 5,"ec_id": 0,"label": "SOMETHING","uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"] }]
        }"#;

        let prepared_str = prepare_stringified_json_string(original_str);

        let expected_str = r#"{[{"unit_id": "5", "ec_id": "0", "label": "SOMETHING", "uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"]}]}"#.to_string();

        assert_eq!(prepared_str, expected_str);
    }
}
