use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use proc_macro2::{Ident, Span};
use syn::{__private::TokenStream2, *, punctuated::Punctuated, token::Comma};

/* ---------------------------------- Core ---------------------------------- */

    macro_rules! ident {($($arg:tt)*) => {{
        Ident::new(&format!($($arg)*), Span::call_site())
    }}}

    trait IntuplePath {
        fn get_option (&self) -> Option<&'static str>;
    }
    impl IntuplePath for Path {
        fn get_option (&self) -> Option<&'static str> {
            if self.is_ident("ignore") || self.is_ident("igno") {
                Some("ignore")
            } else if self.is_ident("recursive") || self.is_ident("rcsv") {
                Some("recursive")
            } else if self.is_ident("recursive_enum") || self.is_ident("rcsve") {
                Some("recursive_enum")
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
        fn is_recursive_enum(&self) -> bool;
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
        fn is_recursive_enum(&self) -> bool {
            self.attrs.as_strings().contains(&"recursive_enum")
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

    #[derive(Default)]
    struct TypeOut {
        object: TokenStream2,
        reference: TokenStream2,
        reference_mut: TokenStream2,
    }

    trait IntupleFieldVec {
        fn intuple_tuple_map <T:ToTokens,F:Fn(usize,&Field)->T> (&self,func:F) -> (TokenStream2,TypeOut);
        fn intuple_types_fn <F:Fn(&mut TokenStream2,Type,&Field)> (&self,func:F) -> TokenStream2;
        fn intuple_types (&self,) -> TypeOut;
        fn intuple_blocks <T:ToTokens,F:Fn(usize,&Field)->T> (&self,tup_offset:usize,func:F) -> (TypeOut,TokenStream2);
    }
    impl IntupleFieldVec for Punctuated<Field,Comma> {
        fn intuple_tuple_map <T:ToTokens,F:Fn(usize,&Field)->T> (&self,func:F) -> (TokenStream2,TypeOut) {
            let mut left = quote![];
            for (pos,field) in self.iter().enumerate() {
                let id = func(pos,field);
                left = quote!{#left #id,};
            }           
            let mut right = TypeOut::default();
            for (pos,field) in self.iter().enumerate() {
                let id = func(pos,field);
                if field.not_ignored() {
                    right.object.extend(match (field.is_recursive(),field.is_recursive_enum()) {
                        (true,_) | (_,true) => quote!{#id.into(),},
                        _ => quote!{#id,},
                    });
                    right.reference.extend(match (field.is_recursive(),field.is_recursive_enum()) {
                        (true,_) => quote!{#id.as_tuple_ref(),},
                        (_,true) => quote!{#id.as_tuple_enum_ref(),},
                        _        => quote!{#id,},
                    });
                    right.reference_mut.extend(match (field.is_recursive(),field.is_recursive_enum()) {
                        (true,_) => quote!{#id.as_tuple_ref_mut(),},
                        (_,true) => quote!{#id.as_tuple_enum_ref_mut(),},
                        _        => quote!{#id,},
                    });
                }
            }
            (left,right)
        }
        fn intuple_types_fn <F:Fn(&mut TokenStream2,Type,&Field)> (&self,func:F) -> TokenStream2 {
            let mut out = quote!{};
            for field in self {
                if field.not_ignored() {
                    let ty = field.ty.clone();
                    func(&mut out,ty,field);
                    out.extend(quote!(,))
                }
            }
            out
        }
        fn intuple_types (&self) -> TypeOut {
            let object = self.intuple_types_fn(|quote,ty,field|{
                quote.extend( match (field.is_recursive(),field.is_recursive_enum()) {
                    (true,_) => quote!(<#ty as Intuple>::Tuple),
                    (_,true) => quote!(<#ty as IntupleEnum>::Tuple),
                    _        => quote!{#ty},
                });
            });
            let object = quote!{(#object)};
            let reference = self.intuple_types_fn(|quote,ty,field|{
                quote.extend( match (field.is_recursive(),field.is_recursive_enum()) {
                    (true,_) => quote!(<#ty as IntupleRef<'intuple>>::Tuple),
                    (_,true) => quote!(<#ty as IntupleEnumRef<'intuple>>::Tuple),
                    _        => quote!{&'intuple #ty},
                });
            });
            let reference = quote!{(#reference)};
            let reference_mut = self.intuple_types_fn(|quote,ty,field|{
                quote.extend( match (field.is_recursive(),field.is_recursive_enum()) {
                    (true,_) => quote!(<#ty as IntupleRef<'intuple>>::TupleMut),
                    (_,true) => quote!(<#ty as IntupleEnumRef<'intuple>>::TupleMut),
                    _        => quote!{&'intuple mut #ty},
                });
            });
            let reference_mut = quote!{(#reference_mut)};
            TypeOut{object,reference,reference_mut}
        }
        fn intuple_blocks <T:ToTokens,F:Fn(usize,&Field)->T> (&self,mut tup_offset:usize,func:F) -> (TypeOut,TokenStream2){
            let mut dataty_tuple = TypeOut::default();
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
                    dataty_tuple.object.extend(match (field.is_recursive(),field.is_recursive_enum()) {
                        (true,_) | (_,true) => quote!{dataty.#fname.into(),},
                        _ => quote!{dataty.#fname,},
                    });
                    dataty_tuple.reference.extend(match (field.is_recursive(),field.is_recursive_enum()) {
                        (true,_) => quote!{self.#fname.as_tuple_ref(),},
                        (_,true) => quote!{self.#fname.as_tuple_enum_ref(),},
                        _        => quote!{&self.#fname,},
                    });
                    dataty_tuple.reference_mut.extend(match (field.is_recursive(),field.is_recursive_enum()) {
                        (true,_) => quote!{self.#fname.as_tuple_ref_mut(),},
                        (_,true) => quote!{self.#fname.as_tuple_enum_ref_mut(),},
                        _        => quote!{&mut self.#fname,},
                    });
                }
                // Tuple -> Struct
                let value = match tupid {
                    Some(id) => match (field.is_recursive(),field.is_recursive_enum()) {
                        (true,_) | (_,true) => quote!{tuple.#id.into()},
                        _ => quote!{tuple.#id},
                        
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
                    TypeOut{object:quote!{false},reference:quote!{false},reference_mut:quote!{false}},
                    TypeOut{object:quote!{bool},reference:quote!{bool},reference_mut:quote!{bool}},                    
                )
            }
        }
    }

/* --------------------------------- Intuple -------------------------------- */

    #[proc_macro_derive(Intuple, attributes(recursive,recursive_enum,ignore,igno,rcsv,rcsve,intuple))]
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
                q.into()
            },
            Data::Enum(enm) => {
                // Types
                let mut tup_object = quote!{};
                let mut tup_ref = quote!{};
                let mut tup_refmut = quote!{};
                for variant in &enm.variants {
                    if &variant.ident.to_string() == "None" {continue}
                    let (_,_,TypeOut { object, reference, reference_mut }) = variant.fields.intuple();
                    tup_object.extend(quote!{Option<#object>,});
                    tup_ref.extend(quote!{Option<#reference>,});
                    tup_refmut.extend(quote!{Option<#reference_mut>,});
                }
                let tup_object = quote!{(#tup_object)};
                let tup_ref = quote!{(#tup_ref)};
                let tup_refmut = quote!{(#tup_refmut)};

                // Enum -> Tuple
                let type_count = enm.variants.len()-1;
                let mut position = 0;
                // None
                let mut outtup = quote!();
                for _ in 0..type_count {outtup.extend(quote!(None,));}
                let mut qi_object = quote!{ #name::None => (#outtup),};
                let mut qi_ref = qi_object.clone();
                let mut qi_refmut = qi_object.clone();
                // Any other
                for variant in &enm.variants {
                    if &variant.ident.to_string() == "None" {continue}
                    let varnam = variant.ident.clone();
                    let mut outtup = TypeOut::default();
                    match &variant.fields {
                        Fields::Named(nmd) => {
                            let (tup_io_l,TypeOut { object, reference, reference_mut}) = nmd.named.intuple_tuple_map(|_,field| field.ident());
                            // Object
                            for i in 0..type_count {
                                outtup.object.extend(if i == position {
                                    quote!{Some((#object)),}
                                } else {quote!(None,)});
                            }
                            let r = outtup.object;
                            qi_object.extend(quote!{ #name::#varnam{#tup_io_l} => (#r),});
                            // Reference
                            for i in 0..type_count {
                                outtup.reference.extend(if i == position {
                                    quote!{Some((#reference)),}
                                } else {quote!(None,)});
                            }
                            let r = outtup.reference;
                            qi_ref.extend(quote!{ #name::#varnam{#tup_io_l} => (#r),});
                            // Reference Mut
                            for i in 0..type_count {
                                outtup.reference_mut.extend(if i == position {
                                    quote!{Some((#reference_mut)),}
                                } else {quote!(None,)});
                            }
                            let r = outtup.reference_mut;
                            qi_refmut.extend(quote!{ #name::#varnam{#tup_io_l} => (#r),});
                        },
                        Fields::Unnamed(unmd) => {
                            let (tup_io_l,TypeOut { object, reference, reference_mut }) = unmd.unnamed.intuple_tuple_map(|pos,_| ident!("x{}",pos));
                            // Object
                            for i in 0..type_count {
                                outtup.object.extend(if i == position {
                                    quote!{Some((#object)),}
                                } else {quote!(None,)});
                            }
                            let r = outtup.object;
                            qi_object.extend(quote!{ #name::#varnam(#tup_io_l) => (#r),});
                            // Reference
                            for i in 0..type_count {
                                outtup.reference.extend(if i == position {
                                    quote!{Some((#reference)),}
                                } else {quote!(None,)});
                            }
                            let r = outtup.reference;
                            qi_ref.extend(quote!{ #name::#varnam(#tup_io_l) => (#r),});
                            // Reference Mut
                            for i in 0..type_count {
                                outtup.reference_mut.extend(if i == position {
                                    quote!{Some((#reference_mut)),}
                                } else {quote!(None,)});
                            }
                            let r = outtup.reference_mut;
                            qi_refmut.extend(quote!{ #name::#varnam(#tup_io_l) => (#r),});
                        },
                        Fields::Unit => {
                            for i in 0..type_count {
                                outtup.object.extend(if i == position {
                                    quote!{Some(false),}
                                } else {quote!(None,)});
                            }
                            let r = outtup.object;
                            qi_object.extend(quote!{ #name::#varnam => (#r),});
                            qi_ref.extend(quote!{ #name::#varnam => (#r),});
                            qi_refmut.extend(quote!{ #name::#varnam => (#r),});
                        },
                    }
                    position += 1;
                }
                // let qi_object = quote!{#qi_object};

                // Tuple -> Enum
                let mut qout_from = quote!();
                let mut position = 0;
                for variant in &enm.variants {
                    if &variant.ident.to_string() == "None" {continue}
                    let varnam = variant.ident.clone();
                    let mut left = quote!{};
                    for i in 0..type_count {
                        left.extend(if i == position {
                            quote!{Some(tuple),}
                        } else {quote!(None,)});
                    }
                    let (object,_,_) = variant.fields.intuple();
                    qout_from.extend(quote!(
                        (#left) => Self::#varnam #object,
                    ));
                    position += 1;
                }

                quote! {
                    impl #impl_generics Intuple for #name #ty_generics #where_clause {
                        type Tuple = #tup_object;
                        fn from_tuple(item: #tup_object) -> Self{item.into()}
                        fn into_tuple(self) -> #tup_object {self.into()}
                        fn intuple(self) -> #tup_object {self.into()}
                    }
                    impl #ref_impl_generics IntupleRef<'intuple> for #name #ty_generics #where_clause {
                        type Tuple = #tup_ref;
                        type TupleMut = #tup_refmut;
                        fn as_tuple_ref(&'intuple self) -> #tup_ref {match self {#qi_ref}}
                        fn as_tuple_ref_mut(&'intuple mut self) -> #tup_refmut {match self {#qi_refmut}}
                    }
                    impl #impl_generics From<#tup_object> for #name #ty_generics #where_clause {
                        fn from(tuple: #tup_object) -> Self {match tuple {
                            #qout_from
                            _ => Self::None
                        }}
                    }
                    impl #impl_generics From<#name #ty_generics> for #tup_object #where_clause {
                        fn from(dataty: #name #ty_generics) -> Self {match dataty {#qi_object}}
                    }
                }.into()
            },
            Data::Union(_) => panic!("Unions not supported!"),
        }
    }

/* ------------------------------ Intuple Enum ------------------------------ */

#[proc_macro_derive(IntupleEnum, attributes(recursive,recursive_enum,ignore,igno,rcsv,rcsve,intuple))]
pub fn intuple_enum_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_intuple_enum_macro(&ast)
}

fn impl_intuple_enum_macro(ast: &DeriveInput) -> TokenStream {
    let vis = &ast.vis;
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let mut generics_ref = ast.generics.clone();
    let lifetime = Lifetime::new("'intuple",Span::call_site());
    for a in generics_ref.type_params_mut() {
        a.bounds.push(TypeParamBound::Lifetime(lifetime.clone()));
    }
    generics_ref.params.push(GenericParam::Lifetime(LifetimeParam::new(lifetime)));
    let (ref_impl_generics,ref_ty_generics,_) = &generics_ref.split_for_impl();

    let mut ederive = quote!();
    for a in &ast.attrs {
        if a.path().is_ident("intuple"){
            let expr = a.meta.require_list().unwrap().parse_args::<Expr>().unwrap().to_token_stream();
            ederive = quote!(#[#expr]);
        }
    }

    match &ast.data {
        Data::Enum(enm) => {
            // Types
            let mut tup_object = quote!{};
            let mut tup_ref = quote!{};
            let mut tup_refmut = quote!{};
            for variant in &enm.variants {
                let varnam = variant.ident.clone();
                let (_,_,TypeOut { object, reference, reference_mut }) = variant.fields.intuple();
                match &variant.fields {
                    Fields::Unit => {
                        tup_object.extend(quote!{#varnam,});
                        tup_ref.extend(quote!{#varnam,});
                        tup_refmut.extend(quote!{#varnam,});
                    },_=>{
                        tup_object.extend(quote!{#varnam(#object),});
                        tup_ref.extend(quote!{#varnam(#reference),});
                        tup_refmut.extend(quote!{#varnam(#reference_mut),});
                    }
                }
            }
            let tup_object = quote!{#tup_object};
            let tup_ref = quote!{#tup_ref};
            let tup_refmut = quote!{#tup_refmut};

            let ename_object = ident!("{}Intuple",name);
            let ename_ref = ident!("{}IntupleRef",name);
            let ename_refmut = ident!("{}IntupleRefMut",name);

            // Tuple -> Enum
            let mut qout_from = quote!();
            for variant in &enm.variants {
                let varnam = variant.ident.clone();
                match &variant.fields {
                    Fields::Unit => {
                        qout_from.extend(quote!(
                            #ename_object::#varnam => Self::#varnam,
                        ));
                    },_=>{
                        let left = quote!{#ename_object::#varnam(tuple)};
                        let (object,_,_) = variant.fields.intuple();
                        qout_from.extend(quote!(
                            #left => Self::#varnam #object,
                        ));
                    }
                }
            }

            // Enum -> Tuple
            let mut qi_object = quote!();
            let mut qi_ref = quote!();
            let mut qi_refmut = quote!();
            for variant in &enm.variants {
                let varnam = variant.ident.clone();
                match &variant.fields {
                    Fields::Named(nmd) => {
                        let (left,TypeOut { object, reference, reference_mut }) = nmd.named.intuple_tuple_map(|_,field| field.ident());
                        qi_object.extend(quote!{ #name::#varnam{#left} => #ename_object::#varnam((#object)),});
                        qi_ref.extend(quote!{ #name::#varnam{#left} => #ename_ref::#varnam((#reference)),});
                        qi_refmut.extend(quote!{ #name::#varnam{#left} => #ename_refmut::#varnam((#reference_mut)),});
                    },
                    Fields::Unnamed(unmd) => {
                        let (left,TypeOut { object, reference, reference_mut }) = unmd.unnamed.intuple_tuple_map(|pos,_| Ident::new(&format!("x{}",pos),Span::call_site()));
                        qi_object.extend(quote!{ #name::#varnam(#left) => #ename_object::#varnam((#object)),});
                        qi_ref.extend(quote!{ #name::#varnam(#left) => #ename_ref::#varnam((#reference)),});
                        qi_refmut.extend(quote!{ #name::#varnam(#left) => #ename_refmut::#varnam((#reference_mut)),});
                    },
                    Fields::Unit => {
                        qi_object.extend(quote!{ #name::#varnam => #ename_object::#varnam,});
                        qi_ref.extend(quote!{ #name::#varnam => #ename_ref::#varnam,});
                        qi_refmut.extend(quote!{ #name::#varnam => #ename_refmut::#varnam,});
                    },
                }
            }

            let q = quote! {
                impl #impl_generics IntupleEnum for #name #ty_generics #where_clause {
                    type Tuple = #ename_object;
                    fn from_tuple_enum(item: #ename_object) -> Self{item.into()}
                    fn into_tuple_enum(self) -> #ename_object {self.into()}
                    fn intuple_enum(self) -> #ename_object {self.into()}
                }
                impl #ref_impl_generics IntupleEnumRef<'intuple> for #name #ty_generics #where_clause {
                    type Tuple = #ename_ref #ref_ty_generics;
                    type TupleMut = #ename_refmut #ref_ty_generics;
                    fn as_tuple_enum_ref(&'intuple self) -> #ename_ref {match self {#qi_ref}}
                    fn as_tuple_enum_ref_mut(&'intuple mut self) -> #ename_refmut {match self {#qi_refmut}}
                }
                impl #impl_generics From<#ename_object> for #name #ty_generics #where_clause {
                    fn from(tuple: #ename_object) -> Self {match tuple {#qout_from}}
                }
                impl #impl_generics From<#name #ty_generics> for #ename_object #where_clause {
                    fn from(dataty: #name #ty_generics) -> Self {match dataty {#qi_object}}
                }
                #ederive
                #vis enum #ename_object #ty_generics #where_clause {#tup_object}
                #ederive
                #vis enum #ename_ref #ref_ty_generics #where_clause {#tup_ref}
                #ederive
                #vis enum #ename_refmut #ref_ty_generics #where_clause {#tup_refmut}
            };
            q.into()
        },
        Data::Struct(_) => panic!("Structs not supported!"),
        Data::Union(_)  => panic!("Unions not supported!"),
    }
}