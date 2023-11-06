use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use proc_macro2::{Ident, Span};
use syn::{__private::TokenStream2, *, punctuated::Punctuated, token::Comma};

/* ---------------------------------- Core ---------------------------------- */

    trait IntupleAttributes {
        fn as_strings(&self) -> Vec<&'static str>;
    }
    impl IntupleAttributes for Vec<Attribute> {
        fn as_strings(&self) -> Vec<&'static str> {
            let mut names = vec![];
            for aaa in self {
                let ewr = aaa.meta.path().get_ident();
                let nme = ewr.as_ref().unwrap().to_string();
                if &nme == "ignore" || &nme == "igno" {
                    names.push("ignore");
                } else if &nme == "recursive" || &nme == "rcsv" {
                    names.push("recursive");
                }
            }
            names
        }
    }

    trait IntupleField {
        fn ident(&self) -> Ident;
        fn is_ignored(&self) -> bool;
        fn is_recursive(&self) -> bool;
        fn not_ignored(&self) -> bool;
        fn value_from_tuple_or_default(&self,index:&mut usize) -> TokenStream2;
    }
    impl IntupleField for Field {
        fn ident(&self) -> Ident {
            self.ident.as_ref().unwrap().clone()
        }
        fn is_ignored(&self) -> bool {
            self.attrs.as_strings().contains(&"ignore")
        }
        fn is_recursive(&self) -> bool {
            self.attrs.as_strings().contains(&"recursive")
        }
        fn not_ignored(&self) -> bool {
            !self.is_ignored()
        }
        fn value_from_tuple_or_default(&self,index:&mut usize) -> TokenStream2 {
            if self.not_ignored() {
                let id = Index::from(*index);
                *index += 1;
                quote!{tuple.#id}
            } else {
                let ty = self.ty.clone();
                quote!{#ty::default()}
            }
        }
    }

    trait IntupleFieldVec {
        fn intuple_types (&self,is_trait:bool) -> TokenStream2;
        fn intuple_blocks <T:ToTokens,F:Fn(usize,&Field)->T> (&self,tup_offset:usize,func:F) -> (TokenStream2,TokenStream2);
    }
    impl IntupleFieldVec for Punctuated<Field,Comma> {
        fn intuple_types (&self,is_trait:bool) -> TokenStream2 {
            let mut out = quote!{};
            for field in self {
                if field.not_ignored() {
                    let ty = field.ty.clone();
                    out = match field.is_recursive() {
                        true  => match is_trait {
                            true => quote!(#out <#ty as Intuple>::Intuple,),
                            false => {
                                let typeid = Ident::new(&(ty.into_token_stream().to_string()+"Intuple"), Span::call_site());
                                quote!(#out #typeid,)
                            },
                        },
                        false => quote!{#out #ty,},
                    };
                }
            }
            out
        }
        fn intuple_blocks <T:ToTokens,F:Fn(usize,&Field)->T> (&self,mut tup_offset:usize,func:F) -> (TokenStream2,TokenStream2){
            let mut dataty_tuple = quote!{};
            let mut tuple_dataty = quote!{};
            for (position,field) in self.iter().enumerate() {
                // Tuple Index
                let tupid = if field.not_ignored() {
                    let out:Index = tup_offset.into();
                    tup_offset += 1;
                    Some(out)
                } else {None};
                // Struct -> Tuple
                let fname = func(position,field);
                if field.not_ignored() {
                    dataty_tuple = match field.is_recursive() {
                        true => quote!{#dataty_tuple dataty.#fname.into(),},
                        false => quote!{#dataty_tuple dataty.#fname,},
                    };
                }
                // Tuple -> Struct
                let value = match tupid {
                    Some(id) => match field.is_recursive() {
                        true => quote!{tuple.#id.into()},
                        false => quote!{tuple.#id},
                    },
                    None => {
                        let ftype = field.ty.clone();
                        quote!{#ftype::default()}
                    },
                };
                tuple_dataty = quote!{#tuple_dataty #fname: #value,};
            }
            (dataty_tuple,tuple_dataty)
        }
    }


    trait IntupleFields {
        fn intuple (&self,is_trait:bool) -> (TokenStream2,TokenStream2,TokenStream2);
    }
    impl IntupleFields for Fields {
        fn intuple (&self,is_trait:bool) -> (TokenStream2,TokenStream2,TokenStream2) {
            match &self {
                /* ---------------------------------- Named --------------------------------- */
                Fields::Named(fields) => {
                    let (dataty_tuple,tuple_dataty) = fields.named.intuple_blocks(0,|_,field| field.ident());
                    (quote!{{#tuple_dataty}},
                    quote!{(#dataty_tuple)},
                    fields.named.intuple_types(is_trait))
                },
                /* --------------------------------- Unnamed -------------------------------- */
                syn::Fields::Unnamed(fields) => {
                    let (dataty_tuple,tuple_dataty) = fields.unnamed.intuple_blocks(0,|position,_| Index::from(position));
                    (quote!{{#tuple_dataty}},
                    quote!{(#dataty_tuple)},
                    fields.unnamed.intuple_types(is_trait))
                }
                /* ---------------------------------- Unit ---------------------------------- */
                syn::Fields::Unit => (quote!{},quote!{()},quote!{})
            }
        }
    }


/* ------------------------------ Intuple Lite ------------------------------ */

    #[proc_macro_derive(IntupleLite, attributes(recursive,igno,rcsv))]
    pub fn intuple_from_macro_derive(input: TokenStream) -> TokenStream {
        let ast = syn::parse(input).unwrap();
        impl_intuple_from_macro(&ast)
    }

    fn impl_intuple_from_macro(ast: &syn::DeriveInput) -> TokenStream {
        let vis = &ast.vis;
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
        match &ast.data {
            Data::Struct(strct) => {
                let (qout_from,qout_into,types) = strct.fields.intuple(false);
                let qout_from = quote!{Self #qout_from};
                let intuple = quote!{(#types)};
                let typeid = Ident::new(&(name.to_string()+"Intuple"), Span::call_site());
                quote! {
                    impl #impl_generics From<#intuple> for #name #ty_generics #where_clause {
                        fn from(tuple: #intuple) -> Self { #qout_from }
                    }
                    impl #impl_generics From<#name #ty_generics> for #intuple #where_clause {
                        fn from(dataty: #name #ty_generics) -> Self { #qout_into }
                    }
                    #vis type #typeid #ty_generics = #intuple;
                }.into()
            },
            Data::Enum(_) => panic!("Enums not supported!"),
            Data::Union(_) => panic!("Unions not supported!"),
        }
    }

/* --------------------------------- Intuple -------------------------------- */

    #[proc_macro_derive(Intuple, attributes(recursive,igno,rcsv))]
    pub fn intuple_macro_derive(input: TokenStream) -> TokenStream {
        let ast = syn::parse(input).unwrap();
        impl_intuple_macro(&ast)
    }

    fn impl_intuple_macro(ast: &syn::DeriveInput) -> TokenStream {
        let vis = &ast.vis;
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
        match &ast.data {
            Data::Struct(strct) => {
                let (qout_from,qout_into,types) = strct.fields.intuple(true);
                let qout_from = quote!{Self #qout_from};
                let intuple = quote!{(#types)};
                let typeid = Ident::new(&(name.to_string()+"Intuple"), Span::call_site());
                quote! {
                    impl #impl_generics Intuple for #name #ty_generics #where_clause {
                        type Intuple = #intuple;
                        fn from_tuple(item: Self::Intuple) -> Self{item.into()}
                        fn into_tuple(self) -> Self::Intuple {self.into()}
                        fn intuple(self) -> Self::Intuple {self.into()}
                    }
                    impl #impl_generics From<#intuple> for #name #ty_generics #where_clause {
                        fn from(tuple: #intuple) -> Self { #qout_from }
                    }
                    impl #impl_generics From<#name #ty_generics> for #intuple #where_clause {
                        fn from(dataty: #name #ty_generics) -> Self { #qout_into }
                    }
                    #vis type #typeid #ty_generics = #intuple;
                }.into()
            },
            Data::Union(_) => panic!("Unions not supported!"),
            Data::Enum(_) => panic!("Enums not supported!"),
        }
    }