use crate::json_parse_approach::{parse_json_string, JsonContext};

pub fn prepare_json_string(original_str: &str) -> String {
    let json_context = JsonContext::Value;

    let trimmed_str = original_str.trim();

    let parsed_json_string = parse_json_string(trimmed_str, json_context);

    parsed_json_string
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

        let expected_str = r#"[{"tag": "lol", "groups": []}]"#;

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

        let expected_str = "{\"property1\": \"lol\", \"property2\": \"iskrem\"}";

        assert_eq!(prepared_str, expected_str);
    }

    #[test]
    fn arbitrary_test() {
        let original_str = r#"{
            [{"unit_id": 5,"ec_id": 0,"label": "SOMETHING","uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"] }]
        }"#;

        let prepared_str = prepare_json_string(original_str);

        let expected_str = r#"{[{"unit_id": 5, "ec_id": 0, "label": "SOMETHING", "uid": "00000000-0000-0000-0000-000000000001", "customtags": {"tag1": ""}, "groups": ["input:max_adr:205"]}]}"#.to_string();

        assert_eq!(prepared_str, expected_str);
    }
}
