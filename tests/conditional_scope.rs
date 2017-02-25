#[macro_use] extern crate bart_derive;

#[test]
fn it_supports_conditional_scope_with_boolean() {
    #[derive(BartDisplay)]
    #[template_string="{{#a?}}yes{{/a}}"]
    struct Test { a: bool }

    assert_eq!(
        "yes",
        format!("{}", Test { a: true })
    );

    assert_eq!(
        "",
        format!("{}", Test { a: false })
    );
}

#[test]
fn it_supports_negative_conditional_scope_with_boolean() {
    #[derive(BartDisplay)]
    #[template_string="{{^a?}}no{{/a}}"]
    struct Test { a: bool }

    assert_eq!(
        "",
        format!("{}", Test { a: true })
    );

    assert_eq!(
        "no",
        format!("{}", Test { a: false })
    );
}

#[test]
fn it_supports_conditional_scope_with_non_bool() {
    extern crate bart;

    struct TestBool<'a> {
        name: &'a str,
    }

    impl<'a> bart::Conditional for TestBool<'a> {
        fn val(&self) -> bool {
            self.name.len() > 2
        }
    }

    #[derive(BartDisplay)]
    #[template_string="{{cond.name}}: {{#cond?}}Hello {{.name}}{{/cond}}"]
    struct Test<'a> { cond: TestBool<'a> }

    assert_eq!(
        "Joe: Hello Joe",
        format!("{}", Test { cond: TestBool { name: "Joe" } })
    );

    assert_eq!(
        "No: ",
        format!("{}", Test { cond: TestBool { name: "No" } })
    );
}