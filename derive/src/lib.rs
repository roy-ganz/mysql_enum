extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;



use proc_macro::TokenStream;



#[proc_macro_derive(MysqlEnum)]
pub fn toql_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_derive(&ast);

	//println!("GEN = {:?}", gen);
    gen.parse().unwrap()
}



fn impl_derive(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    quote! {
				impl mysql::prelude::ConvIr<#name> for mysql_enum::EnumIr<#name> {
						fn new(v: mysql::Value) -> Result< mysql_enum::EnumIr<#name>, mysql::FromValueError> {
							  mysql_enum::convert_enum::<#name>(v)
						}
						fn commit(self) -> #name {
								self.value
						}
						fn rollback(self) -> mysql::Value {
								mysql::Value::Bytes(self.string.into_bytes())
						}
				}
				impl mysql::prelude::FromValue for #name {
						type Intermediate = mysql_enum::EnumIr<#name>;
				}

				impl std::convert::From<#name> for mysql::Value {
					fn from (t: #name) -> Self {
						mysql::Value::from(t.to_string())
					}
				}
    }
}
