use minijinja::{value::StructObject, Environment, Value};

struct Bar;

impl Drop for Bar {
    fn drop(&mut self) {
        eprintln!("Dropping Bar");
    }
}

impl StructObject for Bar {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "world" => Some(Value::from("Hello, World!")),
            _ => None,
        }
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo");
    }
}

impl StructObject for Foo {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "hello" => Some(Value::from_struct_object(Bar)),
            _ => None,
        }
    }
}

fn main() {
    let mut environment = Environment::empty();
    environment
        .add_template(
            "macro-arg",
            r#"
    {%- macro foo(arg) -%}
        {{ arg.world }}
    {%- endmacro -%}

    {{- foo(arg=hello) }}
"#,
        )
        .unwrap();

    environment
        .add_template(
            "inside-macro",
            r#"
    {%- macro foo() -%}
        {{ hello.world }}
    {%- endmacro -%}

    {{- foo() }}
"#,
        )
        .unwrap();

    environment
        .add_template("outside-macro", "{{ hello.world }}")
        .unwrap();

    for i in 0..3 {
        eprintln!("({i}) Render macro-arg:");
        let template = environment.get_template("macro-arg").unwrap();
        let res = template.render(Value::from_struct_object(Foo)).unwrap();
        assert_eq!(res, "Hello, World!");
        eprintln!();

        eprintln!("({i}) Render inside-macro:");
        let template = environment.get_template("inside-macro").unwrap();
        let res = template.render(Value::from_struct_object(Foo)).unwrap();
        assert_eq!(res, "Hello, World!");
        eprintln!();

        eprintln!("({i}) Render outside-macro:");
        let template = environment.get_template("outside-macro").unwrap();
        let res = template.render(Value::from_struct_object(Foo)).unwrap();
        assert_eq!(res, "Hello, World!");
        eprintln!();
    }

    drop(environment);

    eprintln!("Environment dropped");
}
