use jsonata_core::{evaluator::Evaluator, parser::parse, value::JValue};

fn evaluate(expression: &str) -> JValue {
    let ast = parse(expression).unwrap();
    Evaluator::new().evaluate(&ast, &JValue::Null).unwrap()
}

#[test]
fn linear_backend_still_handles_existing_regex_paths() {
    assert_eq!(evaluate(r#"$contains("ab", /a/)"#), JValue::Bool(true));
    assert_eq!(
        evaluate(r#"$split("abac", /a/)"#),
        JValue::array(vec![
            JValue::string(""),
            JValue::string("b"),
            JValue::string("c")
        ])
    );
    assert_eq!(
        evaluate(r#"$replace("ab", /(a)(b)/, "$2$1")"#),
        JValue::string("ba")
    );
    assert_eq!(evaluate(r#"$match("ab", /a/).match"#), JValue::string("a"));
    assert_eq!(evaluate(r#"$match("ab", /a/, 0)"#), JValue::Undefined);
    assert_eq!(evaluate(r#"$match("ab", /z/)"#), JValue::Undefined);
    assert_eq!(
        evaluate(r#"$exists($match("ab", /z/))"#),
        JValue::Bool(false)
    );
}

#[cfg(feature = "regex-lookaround")]
#[test]
fn lookahead_works_across_regex_entry_points() {
    assert_eq!(evaluate(r#"$contains("ab", /a(?=b)/)"#), JValue::Bool(true));
    assert_eq!(
        evaluate(r#"$split("abac", /a(?=b)/)"#),
        JValue::array(vec![JValue::string(""), JValue::string("bac")])
    );
    assert_eq!(
        evaluate(r#"$replace("abac", /a(?=b)/, "x")"#),
        JValue::string("xbac")
    );
    assert_eq!(
        evaluate(r#"$replace("ab", /a(?=b)/, function($match) { "x" })"#),
        JValue::string("xb")
    );
    assert_eq!(
        evaluate(r#"$match("ab", /a(?=b)/).match"#),
        JValue::string("a")
    );
    assert_eq!(evaluate(r#"("ab" ~> /a(?=b)/).match"#), JValue::string("a"));
}

#[cfg(feature = "regex-lookaround")]
#[test]
fn fancy_backend_propagates_backtrack_limit_errors() {
    let regex = jsonata_core::functions::string::build_regex(r"(?i)(a|b|ab)*(?>c)", "").unwrap();
    let input = "ab".repeat(80);
    let error = regex.is_match(&input).unwrap_err();
    assert!(error.to_string().to_lowercase().contains("backtrack"));
}

#[cfg(feature = "regex-lookaround")]
#[test]
fn limits_stop_before_requesting_another_fancy_match() {
    let input = "ab".repeat(80);
    let pattern = r"(?i)(a|b|ab)*(?>c)";

    assert_eq!(
        evaluate(&format!(r#"$split("{input}", /{pattern}/, 0)"#)),
        JValue::array(vec![])
    );
    assert_eq!(
        evaluate(&format!(r#"$split("head,{input}", /,|{pattern}/, 1)"#)),
        JValue::array(vec![JValue::string("head")])
    );
    assert_eq!(
        evaluate(&format!(r#"$replace("{input}", /{pattern}/, "x", 0)"#)),
        JValue::string(input.clone())
    );
    assert_eq!(
        evaluate(&format!(
            r#"$replace("{input}", /{pattern}/, function($match) {{ "x" }}, 0)"#
        )),
        JValue::string(input.clone())
    );
    assert_eq!(
        evaluate(&format!(r#"$match("{input}", /{pattern}/, 0)"#)),
        JValue::Undefined
    );
}

#[cfg(not(feature = "regex-lookaround"))]
#[test]
fn default_backend_still_rejects_lookahead() {
    assert!(Evaluator::new()
        .evaluate(&parse(r#"$match("ab", /a(?=b)/)"#).unwrap(), &JValue::Null)
        .is_err());
}
