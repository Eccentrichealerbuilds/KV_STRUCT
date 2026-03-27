// src/macro_.rs

/// A macro to define a struct and automatically implement the `Jsonfy` trait.
///
/// Usage includes wrapping a standard struct definition inside the `json!` macro.
/// The macro generates an implementation of `to_json` that builds a JSON-like
/// string representing the struct's fields and values.
///
/// **Important Note:** This procedural-like macro serializes struct fields using
/// their `Debug` implementation. Therefore, every field type within the defined
/// struct *must* implement the standard `Debug` trait.
#[macro_export]
macro_rules! json {
    (
        $(#[$meta:meta])*
        $vis:vis struct $struct_name:ident {$( $vis_field:vis $field_name:ident : $field_data:ty),* $(,)?}
    ) =>{
            $(#[$meta])*
            $vis struct $struct_name {
                $( $vis_field $field_name : $field_data ),*
            }

            impl $crate::Jsonfy for $struct_name {
                fn to_json(&self) -> String{
                    let mut result = String::from("{\n");
                    let mut first = true;
                    $(
                    if !first { result.push_str(",\n");}
                    let field_name = stringify!($field_name).to_string();
                    let field_data = &self.$field_name;
                    let json = format!("  \"{}\" : {:?}", field_name, field_data);
                    let json = format!("{:#}", json);
                    let mut first_switch = || first = false;
                    first_switch();
                    result.push_str(&json);
                    )*
                    result.push_str("\n}");
                    result
                }
            }

        };
}

/// A trait for converting structs into a JSON-like string format.
///
/// Types implementing this trait provide the `to_json` method, which returns
/// a `String` containing the serialized representation.
///
/// By default, when implemented via the `json!` macro, this trait relies entirely
/// on the `Debug` trait to format the struct's data.
pub trait Jsonfy {
    /// Serializes the struct to a JSON-like formatted string.
    fn to_json(&self) -> String;
}
