
use mysql::Value;

#[derive(Debug)]
pub struct EnumIr<T> {
	pub string: String,
	pub value: T,
}

pub use mysql_enum_derive::MysqlEnum;

pub fn convert_enum<T: std::str::FromStr>(v: Value) -> Result<EnumIr<T>, mysql::FromValueError> {
	match v {
		Value::Bytes(bytes) => match String::from_utf8(bytes) {
			Ok(string) => match string.parse() {
				Ok(value) => Ok(EnumIr { value, string }),
				Err(_) => Err(mysql::FromValueError(Value::Bytes(string.into_bytes()))),
			},
			Err(e) => Err(mysql::FromValueError(Value::Bytes(e.into_bytes()))),
		},
		v => Err(mysql::FromValueError(v)),
	}
}
