Formats a given string so that `serde_json` crate can understand it. 

# Example 1

``` rust
use serial_test::serial;
use json_string::prepare_json_string;

let original_str = r#"
    [
        {"tag":"lol", "groups":[]}
    ]
"#;
let prepared_str = prepare_json_string(original_str);

let expected_str = r#"[{"tag": "lol"}]"#;

assert_eq!(prepared_str, expected_str);
```

# Example 2
``` rust
use serial_test::serial;
use json_string::prepare_json_string;

let original_str = r#"
    [
        {"label":"lol", "customtags": {"k1": "v1"}}
    ]
"#;
let prepared_str = prepare_json_string(original_str);

let expected_str = r#"[{"label": "lol", "customtags": {"k1": "v1"}}]"#;

assert_eq!(prepared_str, expected_str);

```