use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::*;

pub fn impl_widget_deser_trait(ast: &DeriveInput) -> Result<TokenStream> {
    Ok({
        let name = &ast.ident;
        let name_str = name.to_string();
        let data = &ast.data;
        let gen;
        match data {
            Data::Enum(variants) => {
                let data_expanded_members = variants.variants.iter().map(|field| {
                    let field_name = &field.ident;
                    let span = field_name.span();
                    let field_name_stringified = LitStr::new(&field_name.to_string(), span);
                    quote_spanned! { 
                        span=> {
                            #field_name_stringified
                        }
                    }
                });
        
                gen = quote! {
                    impl FltkForm for #name {
                        fn generate(&self) -> Box<dyn WidgetExt> {
                            let mut choice = menu::Choice::default();
                            let mems = vec![#(#data_expanded_members),*];
                            for mem in mems {
                                choice.add_choice(mem);
                            }
                            choice.set_value(*self as i32);
                            let val = format!("{:?}", *self);
                            unsafe {
                                app::set_raw_callback(
                                    &mut choice,
                                    Box::into_raw(Box::new(val)) as *mut std::os::raw::c_void,
                                    None,
                                );
                            }
                            Box::new(choice)
                        }
                    }
                };
            }

            Data::Struct(DataStruct {
                fields: Fields::Named(it),
                ..
            }) => {
                let data_expanded_members = it.named.iter().map(|field| {
                    let field_name = field.ident.as_ref().expect("Unreachable");
                    let span = field_name.span();
                    let field_name_stringified = LitStr::new(&field_name.to_string(), span);
                    quote_spanned! { 
                        span=> {
                            let mut i = self.#field_name.generate();
                            i.set_label(#field_name_stringified);
                        }
                    }
                });
        
                gen = quote! {
                    impl FltkForm for #name {
                        fn generate(&self) -> Box<dyn WidgetExt> {
                            let mut flex = group::Flex::default().column().with_label(&format!("{}", #name_str)).with_align(fltk::enums::Align::Left | fltk::enums::Align::Top);
                            let mems = vec![#(#data_expanded_members),*];
                            flex.end();
                            flex.resize(flex.x(), flex.y(), flex.parent().unwrap().width() * 2 / 3, (mems.len() * 30) as i32);
                            let flex = flex.center_of_parent();
                            Box::new(flex)
                        }
                    }
                };
            },

            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "Expected a `struct` with named fields",
                ));
            }
        };
        gen.into()
    })
}
