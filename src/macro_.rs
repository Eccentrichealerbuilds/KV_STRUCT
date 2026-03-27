#[macro_export]
macro_rules! json {
    (
		$(#[$meta:meta])*
		$vis:vis struct
		$struct_name:ident {$( $vis_field:vis $field_name:ident : $field_data:ty),* $(,)?} ) => {
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

pub trait Jsonfy {
    fn to_json(&self) -> String;
}
