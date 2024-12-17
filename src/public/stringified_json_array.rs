use crate::{
    helpers::{content_str, ensure_array_wrapper, json_context, rewrap_string},
    public::parse_stringified_json_string::parse_stringified_json_string,
};

pub fn prepare_stringified_json_array(original_str: &str) -> String {
    let trimmed_str = original_str.trim_matches([' ', '\n', '\t', ',', ';', ':']);
    let array_input = ensure_array_wrapper(trimmed_str);
    let json_context = json_context(&array_input);
    let content_str = content_str(json_context.clone(), &array_input);

    let parsed_json_string = parse_stringified_json_string(&content_str, json_context.clone());

    let rewrapped_string = rewrap_string(&parsed_json_string, json_context);
    rewrapped_string
}

#[cfg(test)]
mod tests {
    use crate::public::stringified_json_array::prepare_stringified_json_array;

    #[test]
    fn array_with_multiple_objects_stringified_to_array() {
        let original_str = r#"
            {"Foo1":19, "Foo2":"BAR2", "Foo3":"BAR3"},
            {"Foo4":10, "Foo5":11, "Foo6":"BAR6"},
            {"Foo7":"BAR7", "Foo8":15, "Foo9":"BAR9"};
        "#;

        let prepared_str = prepare_stringified_json_array(original_str);

        let expected_str = r#"[{"Foo1": "19", "Foo2": "BAR2", "Foo3": "BAR3"}, {"Foo4": "10", "Foo5": "11", "Foo6": "BAR6"}, {"Foo7": "BAR7", "Foo8": "15", "Foo9": "BAR9"}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn prepare_stringified_json_array_w_empty_groups_stringified_to_array() {
        let original_str = r#"
            [
                {"tag":"lol", "groups":[]}
            ]
        "#;
        let prepared_str = prepare_stringified_json_array(original_str);

        let expected_str = r#"[{"tag": "lol", "groups": []}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn prepare_stringified_json_array_w_inner_json_stringified_to_array() {
        let original_str = r#"
            [
                {"label":"lol", "customtags": {"k1": 17}}
            ]
        "#;
        let prepared_str = prepare_stringified_json_array(original_str);

        let expected_str = r#"[{"label": "lol", "customtags": {"k1": "17"}}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn entirely_without_quotes_stringified_to_array() {
        let original_str = "{
            property1: 17,
            property2: iskrem
        }";

        let prepared_str = prepare_stringified_json_array(original_str);

        let expected_str = r#"[{"property1": "17", "property2": "iskrem"}]"#;

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn arbitrary_test_stringified_to_array() {
        let original_str = r#"{
            [{"unit_id": 5,"ec_id": 0,"label": "SOMETHING","uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"] }]
        }"#;

        let prepared_str = prepare_stringified_json_array(original_str);

        let expected_str = r#"[{[{"unit_id": "5", "ec_id": "0", "label": "SOMETHING", "uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"]}]}]"#.to_string();

        assert_eq!(prepared_str, expected_str);
    }
}