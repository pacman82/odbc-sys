use proc_macro::TokenStream;
use quote::quote;

/// Used to derive `From<T> for SQLPOINTER` where T is enum/struct marked with this macro
///
/// Example:
///
/// #[derive(IntoSQLPOINTER)]
/// enum MyEnum {
///     Variant1,
///     Variant2,
/// }
///
#[proc_macro_derive(IntoSQLPOINTER)]
pub fn into_sqlpointer_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let repr_type = ast.attrs.iter().find_map(|attr| {
        if attr.path.is_ident("repr") {
            Some(attr.parse_args::<syn::Ident>().unwrap())
        } else {
            None
        }
    });

    if let Some(repr_type) = repr_type {
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
        let type_name = ast.ident;

        let gen = quote! {
            impl#impl_generics From<#type_name #ty_generics> for SQLPOINTER #where_clause {
                fn from(source: #type_name) -> SQLPOINTER {
                    source as #repr_type as SQLPOINTER
                }
            }
        };

        gen.into()
    } else {
        panic!("No primitive representation found for this type. Expected #[repr(primitive)]");
    }
}

/// Used to derive `Default` trait on an enum and to export default value as a constant whose name
/// is defined by tagging an enum variant with `default(CONST_NAME)` attribute
///
/// # Example:
///
/// #[derive(EnumDefault)]
/// enum MyEnum {
///     Variant1,
///     #[default(VARIANT_DEFAULT)]
///     Variant2,
/// }
///
/// Explanation:
/// Default value for MyEnum is set to be Variant2 and is exported as VARIANT_DEFAULT constant
///
#[proc_macro_derive(EnumDefault, attributes(default))]
pub fn enum_default_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    if let syn::Data::Enum(data) = ast.data {
        let mut default = None;

        for v in &data.variants {
            for attr in &v.attrs {
                if attr.path.is_ident("default") {
                    if default.is_some() {
                        panic!("Multiple definitions of #[default(_)] attribute");
                    }

                    default = Some((&v.ident, attr.parse_args::<syn::Ident>().unwrap()));
                }
            }
        }

        if let Some((default_variant_name, default_const_name)) = default {
            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
            let enum_name = ast.ident;

            let gen = quote! {
                // Export default constant as global
                pub const #default_const_name: #enum_name = #default_variant_name;

                impl#impl_generics Default for #enum_name #ty_generics #where_clause {
                    fn default() -> Self {
                        #default_const_name
                    }
                }
            };

            gen.into()
        } else {
            panic!(
                "No enum variant tagged as default. Expected #[default(const_name)] on enum variant"
            );
        }
    } else {
        panic!("EnumDefault must be defined on enum");
    }
}
