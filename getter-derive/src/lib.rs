// comes with rust
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemStruct, Lit};

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    // Simplest Derive function
    // here we are ignoring struct definition
    // just returning code
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

// ---------------------

#[proc_macro_derive(SimpleImpl)]
pub fn simple_impl(input: TokenStream) -> TokenStream {
    // Parse input struct and implement
    // simple function `simple_print`
    let input = parse_macro_input!(input as DeriveInput);

    // get name of struct
    let name_of_strut = input.ident;

    // Simple print
    TokenStream::from(quote! {
        // to reuse use `#`
        // variables which can be converted to token can be used here
        impl #name_of_strut {
            fn simple_print(&self){
                println!("simple function impl");
            }
        }
    })
}

#[proc_macro_derive(Getter)]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // get name of struct
    let name_of_strut: syn::Ident = input.ident;

    match input.data {
        // if struct
        syn::Data::Struct(struct_data) => {
            // read all fields
            // of the struct
            let getter_funs: Vec<_> = struct_data
                .fields
                .iter()
                .map(|x| {
                    // get name of the field
                    let ident_name = x.ident.clone().unwrap();
                    // get identifier of getter function
                    let name_of_field =
                        syn::Ident::new(&format!("get_{}", ident_name), Span::call_site());
                    // get type of field
                    let type_of_field = x.ty.clone();

                    quote!(
                        // generate token stream for the field
                        pub fn #name_of_field(&self) -> &#type_of_field{
                            return &self.#ident_name
                        }
                    )
                })
                .collect();
            let token_stream = quote! {
                impl #name_of_strut {
                    // write all functions
                    // token stream
                     #(#getter_funs)*
                }
            };
            TokenStream::from(token_stream)
        }
        // if enum
        //  ignoring. not applicable here
        syn::Data::Enum(_) => todo!(),
        // if union,
        // ignoring. not applicable here
        syn::Data::Union(_) => todo!(),
    }
}

#[proc_macro_attribute]
pub fn length(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);
    let args = parse_macro_input!(args as AttributeArgs);
    let arg = args.get(0).unwrap();
    match arg {
        syn::NestedMeta::Lit(lit) => match lit {
            Lit::Int(len_lit) => {
                let len: u8 = len_lit.base10_parse().unwrap();

                let ident_name = item.ident.clone();

                quote! {
                    #item
                    impl #ident_name {
                        fn size()->u8{
                            #len
                        }
                    }
                }
                .into()
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
}
