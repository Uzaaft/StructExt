use struct_ext::IterateFields;

#[derive(IterateFields)]
struct MyStruct {
    a: i32,
    b: String,
    c: f64,
}

fn main() {
    let my_instance = MyStruct {
        a: 42,
        b: "Hello, World!".to_string(),
        c: 3.14,
    };

    for (field_name, field_value) in my_instance.iterate_fields() {
        if let Some(value) = field_value.downcast_ref::<i32>() {
            println!("Field {} (i32): {}", field_name, value);
        } else if let Some(value) = field_value.downcast_ref::<String>() {
            println!("Field {} (String): {}", field_name, value);
        } else if let Some(value) = field_value.downcast_ref::<f64>() {
            println!("Field {} (f64): {}", field_name, value);
        } else {
            println!("Field {}: unknown type", field_name);
        }
    }
}
