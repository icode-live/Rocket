#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::request::{FromForm, FromFormValue, FormItems};
use rocket::http::RawStr;

#[derive(Debug, PartialEq, FromForm)]
struct TodoTask {
    description: String,
    completed: bool
}

// TODO: Make deriving `FromForm` for this enum possible.
#[derive(Debug, PartialEq)]
enum FormOption {
    A, B, C
}

impl<'v> FromFormValue<'v> for FormOption {
    type Error = &'v str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        let variant = match v.as_str() {
            "a" => FormOption::A,
            "b" => FormOption::B,
            "c" => FormOption::C,
            _ => return Err(v)
        };

        Ok(variant)
    }
}

#[derive(Debug, PartialEq, FromForm)]
struct FormInput<'r> {
    checkbox: bool,
    number: usize,
    radio: FormOption,
    password: &'r RawStr,
    textarea: String,
    select: FormOption,
}

#[derive(Debug, PartialEq, FromForm)]
struct DefaultInput<'r> {
    arg: Option<&'r RawStr>,
}

#[derive(Debug, PartialEq, FromForm)]
struct ManualMethod<'r> {
    _method: Option<&'r RawStr>,
    done: bool
}

#[derive(Debug, PartialEq, FromForm)]
struct UnpresentCheckbox {
    checkbox: bool
}

#[derive(Debug, PartialEq, FromForm)]
struct UnpresentCheckboxTwo<'r> {
    checkbox: bool,
    something: &'r RawStr
}

#[derive(Debug, PartialEq, FromForm)]
struct FieldNamedV<'r> {
    v: &'r RawStr,
}

fn parse<'f, T: FromForm<'f>>(string: &'f str, strict: bool) -> Option<T> {
    let mut items = FormItems::from(string);
    let result = T::from_form(items.by_ref(), strict);
    if !items.exhaust() {
        panic!("Invalid form input.");
    }

    result.ok()
}

fn strict<'f, T: FromForm<'f>>(string: &'f str) -> Option<T> {
    parse(string, true)
}

fn lenient<'f, T: FromForm<'f>>(string: &'f str) -> Option<T> {
    parse(string, false)
}

#[test]
fn main() {
    // Same number of arguments: simple case.
    let task: Option<TodoTask> = strict("description=Hello&completed=on");
    assert_eq!(task, Some(TodoTask {
        description: "Hello".to_string(),
        completed: true
    }));

    // Argument in string but not in form.
    let task: Option<TodoTask> = strict("other=a&description=Hello&completed=on");
    assert!(task.is_none());

    // Ensure _method isn't required.
    let task: Option<TodoTask> = strict("_method=patch&description=Hello&completed=off");
    assert_eq!(task, Some(TodoTask {
        description: "Hello".to_string(),
        completed: false
    }));

    let form_string = &[
        "password=testing", "checkbox=off", "checkbox=on", "number=10",
        "checkbox=off", "textarea=", "select=a", "radio=c",
    ].join("&");

    let input: Option<FormInput> = strict(&form_string);
    assert_eq!(input, Some(FormInput {
        checkbox: false,
        number: 10,
        radio: FormOption::C,
        password: "testing".into(),
        textarea: "".to_string(),
        select: FormOption::A,
    }));

    // Argument not in string with default in form.
    let default: Option<DefaultInput> = strict("");
    assert_eq!(default, Some(DefaultInput {
        arg: None
    }));

    // Ensure _method can be captured if desired.
    let manual: Option<ManualMethod> = strict("_method=put&done=true");
    assert_eq!(manual, Some(ManualMethod {
        _method: Some("put".into()),
        done: true
    }));

    let manual: Option<ManualMethod> = lenient("_method=put&done=true");
    assert_eq!(manual, Some(ManualMethod {
        _method: Some("put".into()),
        done: true
    }));

    // And ignored when not present.
    let manual: Option<ManualMethod> = strict("done=true");
    assert_eq!(manual, Some(ManualMethod {
        _method: None,
        done: true
    }));

    // Check that a `bool` value that isn't in the form is marked as `false`.
    let manual: Option<UnpresentCheckbox> = strict("");
    assert_eq!(manual, Some(UnpresentCheckbox {
        checkbox: false
    }));

    // Check that a `bool` value that isn't in the form is marked as `false`.
    let manual: Option<UnpresentCheckboxTwo> = strict("something=hello");
    assert_eq!(manual, Some(UnpresentCheckboxTwo {
        checkbox: false,
        something: "hello".into()
    }));

    // Check that a structure with one field `v` parses correctly.
    let manual: Option<FieldNamedV> = strict("v=abc");
    assert_eq!(manual, Some(FieldNamedV {
        v: "abc".into()
    }));

    // Check that a structure with one field `v` parses correctly (lenient).
    let manual: Option<FieldNamedV> = lenient("v=abc");
    assert_eq!(manual, Some(FieldNamedV { v: "abc".into() }));

    let manual: Option<FieldNamedV> = lenient("v=abc&a=123");
    assert_eq!(manual, Some(FieldNamedV { v: "abc".into() }));

    let manual: Option<FieldNamedV> = lenient("c=abcddef&v=abc&a=123");
    assert_eq!(manual, Some(FieldNamedV { v: "abc".into() }));

    // Check default values (bool) with lenient parsing.
    let manual: Option<UnpresentCheckboxTwo> = lenient("something=hello");
    assert_eq!(manual, Some(UnpresentCheckboxTwo {
        checkbox: false,
        something: "hello".into()
    }));

    let manual: Option<UnpresentCheckboxTwo> = lenient("hi=hi&something=hello");
    assert_eq!(manual, Some(UnpresentCheckboxTwo {
        checkbox: false,
        something: "hello".into()
    }));

    // Check that a missing field doesn't parse, even leniently.
    let manual: Option<FieldNamedV> = lenient("a=abc");
    assert!(manual.is_none());

    let manual: Option<FieldNamedV> = lenient("_method=abc");
    assert!(manual.is_none());
}
