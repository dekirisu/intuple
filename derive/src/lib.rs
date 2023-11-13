use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use proc_macro2::{Ident, Span};
use syn::{__private::TokenStream2, *, punctuated::Punctuated, token::Comma};

/* ---------------------------------- Core ---------------------------------- */

    trait IntuplePath {
        fn get_option (&self) -> Option<&'static str>;
    }
    impl IntuplePath for Path {
        fn get_option (&self) -> Option<&'static str> {
            if self.is_ident("ignore") || self.is_ident("igno") {
                Some("ignore")
            } else if self.is_ident("recursive") || self.is_ident("rcsv") {
                Some("recursive")
            } else {None}
        }
    }

    trait IntupleAttributes {
        fn as_strings(&self) -> Vec<&'static str>;
    }
    impl IntupleAttributes for Vec<Attribute> {
        fn as_strings(&self) -> Vec<&'static str> {
            let mut names = vec![];
            for attr in self {
                if let Some(path) = attr.meta.path().get_option() {
                    names.push(path);
                }
                if attr.meta.path().is_ident("intuple") {
                    attr.parse_nested_meta(|meta|{
                        if let Some(path) = meta.path.get_option() {
                            names.push(path);
                        }
                        Ok(())
                    }).unwrap();
                }
            }
            names
        }
    }

    trait IntupleField {
        fn ident(&self) -> Ident;
        fn is_recursive(&self) -> bool;
        fn not_ignored(&self) -> bool;
        fn value_from_tuple_or_default(&self,index:&mut usize) -> TokenStream2;
    }
    impl IntupleField for Field {
        fn ident(&self) -> Ident {
            self.ident.as_ref().unwrap().clone()
        }
        fn is_recursive(&self) -> bool {
            self.attrs.as_strings().contains(&"recursive")
        }
        fn not_ignored(&self) -> bool {
            !self.attrs.as_strings().contains(&"ignore")
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

    struct TypeOut {
        object: TokenStream2,
        reference: TokenStream2,
        reference_mut: TokenStream2,
    }

    trait IntupleFieldVec {
        fn intuple_types_fn <F:Fn(&mut TokenStream2,Type,bool)> (&self,func:F) -> TokenStream2;
        fn intuple_types (&self,) -> TypeOut;
        fn intuple_blocks <T:ToTokens,F:Fn(usize,&Field)->T> (&self,tup_offset:usize,func:F) -> (TypeOut,TokenStream2);
    }
    impl IntupleFieldVec for Punctuated<Field,Comma> {
        fn intuple_types_fn <F:Fn(&mut TokenStream2,Type,bool)> (&self,func:F) -> TokenStream2 {
            let mut out = quote!{};
            for field in self {
                if field.not_ignored() {
                    let ty = field.ty.clone();
                    func(&mut out,ty,field.is_recursive());
                    out.extend(quote!(,))
                }
            }
            out
        }
        fn intuple_types (&self) -> TypeOut {
            let object = self.intuple_types_fn(|quote,ty,rcsv|{
                quote.extend( match rcsv {
                    true  => quote!(<#ty as Intuple>::Tuple),
                    false => quote!{#ty},
                });
            });
            let object = quote!{(#object)};
            let reference = self.intuple_types_fn(|quote,ty,rcsv|{
                quote.extend( match rcsv {
                    true  => quote!(<#ty as IntupleRef<'intuple>>::Tuple),
                    false => quote!{&'intuple #ty},
                });
            });
            let reference = quote!{(#reference)};
            let reference_mut = self.intuple_types_fn(|quote,ty,rcsv|{
                quote.extend( match rcsv {
                    true  => quote!(<#ty as IntupleRef<'intuple>>::TupleMut),
                    false => quote!{&'intuple mut #ty},
                });
            });
            let reference_mut = quote!{(#reference_mut)};
            TypeOut{object,reference,reference_mut}
        }
        fn intuple_blocks <T:ToTokens,F:Fn(usize,&Field)->T> (&self,mut tup_offset:usize,func:F) -> (TypeOut,TokenStream2){
            let mut dataty_tuple = TypeOut {
                object: quote!{},                
                reference: quote!(),
                reference_mut: quote!(),
            };
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
                    dataty_tuple.object.extend(match field.is_recursive() {
                        true => quote!{dataty.#fname.into(),},
                        false => quote!{dataty.#fname,},
                    });
                    dataty_tuple.reference.extend(match field.is_recursive() {
                        true => quote!{self.#fname.as_tuple_ref(),},
                        false => quote!{&self.#fname,},
                    });
                    dataty_tuple.reference_mut.extend(match field.is_recursive() {
                        true => quote!{self.#fname.as_tuple_ref_mut(),},
                        false => quote!{&mut self.#fname,},
                    });
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
                tuple_dataty.extend(quote!{#fname: #value,});
            }
            let TypeOut { object, reference, reference_mut } = dataty_tuple;
            let dataty_tuple = TypeOut {
                object: quote!{(#object)},                
                reference: quote!{(#reference)}, 
                reference_mut: quote!{(#reference_mut)}, 
            };
            (dataty_tuple,tuple_dataty)
        }
    }

    trait IntupleFields {
        fn intuple (&self) -> (TokenStream2,TypeOut,TypeOut);
    }
    impl IntupleFields for Fields {
        fn intuple (&self) -> (TokenStream2,TypeOut,TypeOut) {
            match &self {
                /* ---------------------------------- Named --------------------------------- */
                Fields::Named(fields) => {
                    let (dataty_tuple,tuple_dataty) = fields.named.intuple_blocks(0,|_,field| field.ident());
                    (quote!{{#tuple_dataty}},
                    dataty_tuple,
                    fields.named.intuple_types())
                },
                /* --------------------------------- Unnamed -------------------------------- */
                syn::Fields::Unnamed(fields) => {
                    let (dataty_tuple,tuple_dataty) = fields.unnamed.intuple_blocks(0,|position,_| Index::from(position));
                    (quote!{{#tuple_dataty}},
                    dataty_tuple,
                    fields.unnamed.intuple_types())
                }
                /* ---------------------------------- Unit ---------------------------------- */
                syn::Fields::Unit => (
                    quote!{},
                    TypeOut{object:quote!{()},reference:quote!{()},reference_mut:quote!{()}},
                    TypeOut{object:quote!{},reference:quote!{},reference_mut:quote!{}},                    
                )
            }
        }
    }

/* --------------------------------- Intuple -------------------------------- */

    #[proc_macro_derive(Intuple, attributes(recursive,ignore,igno,rcsv,intuple))]
    pub fn intuple_macro_derive(input: TokenStream) -> TokenStream {
        let ast = syn::parse(input).unwrap();
        impl_intuple_macro(&ast)
    }

    fn impl_intuple_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
        let mut generics_ref = ast.generics.clone();
        let lifetime = Lifetime::new("'intuple",Span::call_site());
        for a in generics_ref.type_params_mut() {
            a.bounds.push(TypeParamBound::Lifetime(lifetime.clone()));
        }
        generics_ref.params.push(GenericParam::Lifetime(LifetimeParam::new(lifetime)));
        let (ref_impl_generics,_,_) = &generics_ref.split_for_impl();
        
        match &ast.data {
            Data::Struct(strct) => {
                let (qout_from,qout_into,types) = strct.fields.intuple();
                let qout_from = quote!{Self #qout_from};
                let qi_object = qout_into.object;
                let qi_ref = qout_into.reference;
                let qi_refmut = qout_into.reference_mut;
                let tup_object = types.object;
                let tup_ref = types.reference;
                let tup_refmut = types.reference_mut;
                let q = quote! {
                    impl #impl_generics Intuple for #name #ty_generics #where_clause {
                        type Tuple = #tup_object;
                        fn from_tuple(item: #tup_object) -> Self{item.into()}
                        fn into_tuple(self) -> #tup_object {self.into()}
                        fn intuple(self) -> #tup_object {self.into()}
                    }
                    impl #ref_impl_generics IntupleRef<'intuple> for #name #ty_generics #where_clause {
                        type Tuple = #tup_ref;
                        type TupleMut = #tup_refmut;
                        fn as_tuple_ref(&'intuple self) -> #tup_ref {#qi_ref}
                        fn as_tuple_ref_mut(&'intuple mut self) -> #tup_refmut {#qi_refmut}
                    }
                    impl #impl_generics From<#tup_object> for #name #ty_generics #where_clause {
                        fn from(tuple: #tup_object) -> Self { #qout_from }
                    }
                    impl #impl_generics From<#name #ty_generics> for #tup_object #where_clause {
                        fn from(dataty: #name #ty_generics) -> Self { #qi_object }
                    }
                };
                // println!("{}",q);
                q.into()
            },
            Data::Union(_) => panic!("Unions not supported!"),
            Data::Enum(_) => panic!("Enums not supported!"),
        }
    }