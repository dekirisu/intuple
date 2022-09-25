/// Proc macro for untuple!  
use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};

/* ------------------------------------ . ----------------------------------- */

fn intuple_attrs(attrs: &Vec<syn::Attribute>) -> Vec<&'static str>{
    let mut names = vec![];
    for aaa in attrs {
        let ewr = aaa.path.get_ident();
        let nme = ewr.as_ref().unwrap().to_string();
        if &nme == "ignore" || &nme == "igno" {
            names.push("ignore");
        } else if &nme == "recursive" || &nme == "rcsv" {
            names.push("recursive");
        }
    }
    names
}

fn intuple_ignoref(attrs: &Vec<syn::Attribute>) -> bool {
    intuple_attrs(attrs).contains(&"ignore")
}

fn intuple_qthat(field: &syn::Field, ti: &mut usize) -> syn::__private::TokenStream2 {
    let ty = field.ty.clone();
    let mut qthat = quote!{#ty::default()};
    if !intuple_ignoref(&field.attrs) {
        let index = syn::Index::from(ti.clone());
        qthat = quote!{item.#index};
        *ti += 1;
    }
    qthat
}

/* ----------------------------- Into/From Only ----------------------------- */

#[proc_macro_derive(IntupleLite, attributes(igno))]
pub fn intuple_from_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_intuple_from_macro(&ast)
}

fn impl_intuple_from_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(strct) = &ast.data {

        let vis = &ast.vis;
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    
        let mut types       = vec![];
        #[allow(unused_assignments)] 
        let mut qout_from   = quote!{};
        #[allow(unused_assignments)] 
        let mut qout_into   = quote!{};

        let mut ti = 0;

        match &strct.fields {
            /* ---------------------------------- Named --------------------------------- */
            syn::Fields::Named(fields) => {
                //
                let (mut idents, mut qubuild) = (vec![],quote!{});
                //
                for field in &fields.named {
                    //
                    let ident = field.ident.as_ref().unwrap().clone();
                    let qthat = intuple_qthat(&field, &mut ti);
                    qubuild = quote!{#qubuild #ident: #qthat,};
                    //
                    if !intuple_ignoref(&field.attrs) {
                        idents.push(ident);
                        types.push(field.ty.clone());
                    }
                }
                qout_from = quote!{ Self {#qubuild} };
                qout_into = quote!{ (#(item.#idents),*) };
            },
            /* --------------------------------- Unnamed -------------------------------- */
            syn::Fields::Unnamed(fields) => {
                //
                let (mut idents, mut qubuild) = (vec![],quote!{});
                let mut i = 0;
                //
                for field in &fields.unnamed {
                    //
                    let ident = syn::Index::from(i);
                    let qthat = intuple_qthat(&field, &mut ti);
                    qubuild = quote!{#qubuild #qthat,};
                    //
                    if !intuple_ignoref(&field.attrs) {
                        idents.push(ident);
                        types.push(field.ty.clone());
                    }
                    //
                    i += 1;
                }
                qout_from = quote!{ Self (#qubuild) };
                qout_into = quote!{ (#(item.#idents),*) };
            }
            /* ---------------------------------- Unit ---------------------------------- */
            syn::Fields::Unit => {
                qout_from = quote!{ Self };
                qout_into = quote!{ () };
            },
        }
        //
        let intuple = quote!{(#(#types),*)};
        let typeid = Ident::new(&(name.to_string()+"Intuple"), Span::call_site());
        quote! {
            impl #impl_generics From<#intuple> for #name #ty_generics #where_clause {
                fn from(item: #intuple) -> Self { #qout_from }
            }
            impl #impl_generics From<#name #ty_generics> for #intuple #where_clause {
                fn from(item: #name #ty_generics) -> Self { #qout_into }
            }
            #vis type #typeid #ty_generics = #intuple;
        }.into()
        //
    } else {panic!("Not a Struct!")}

}

/* --------------------------------- Intuple -------------------------------- */

#[proc_macro_derive(Intuple, attributes(recursive,igno,rcsv))]
pub fn intuple_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_intuple_macro(&ast)
}

fn impl_intuple_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(strct) = &ast.data {

        let vis = &ast.vis;
        let name = &ast.ident;
        let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    
        let mut qubuild_t = quote!{};
        #[allow(unused_assignments)] 
        let mut qout_from = quote!{};
        #[allow(unused_assignments)] 
        let mut qout_into = quote!{};

        let mut ti = 0;

        match &strct.fields {
            /* ---------------------------------- Named --------------------------------- */
            syn::Fields::Named(fields) => {
                //
                let (mut qubuild_f, mut qubuild_i) = (quote!{}, quote!{});
                //
                for field in &fields.named {
                    //
                    let ident = field.ident.as_ref().unwrap().clone();
                    let mut qthat = intuple_qthat(&field, &mut ti);
                    //
                    if intuple_attrs(&field.attrs).contains(&"recursive") {
                        let ty = field.ty.clone();
                        let index = syn::Index::from(ti.clone()-1);
                        qthat = quote!{ #ty::from_tuple(item.#index) };
                    }
                    //
                    qubuild_f = quote!{#qubuild_f #ident: #qthat,};
                    //
                    if !intuple_ignoref(&field.attrs) {
                        let ty = field.ty.clone();
                        if intuple_attrs(&field.attrs).contains(&"recursive") {
                            qubuild_t = quote!{ #qubuild_t <#ty as IntupleStruct>::Intuple, };
                            qubuild_i = quote!{ #qubuild_i self.#ident.into_tuple(), };
                        } else {
                            qubuild_t = quote!{ #qubuild_t #ty, };
                            qubuild_i = quote!{ #qubuild_i self.#ident, };
                        }
                    }
                }
                qout_from = quote!{ Self {#qubuild_f} };
                qout_into = quote!{ (#qubuild_i) };
            },
            /* --------------------------------- Unnamed -------------------------------- */
            syn::Fields::Unnamed(fields) => {
                //
                let (mut qubuild_f, mut qubuild_i) = (quote!{}, quote!{});
                let mut i = 0;
                //
                for field in &fields.unnamed {
                    //
                    let ident = syn::Index::from(i);
                    let mut qthat = intuple_qthat(&field, &mut ti);
                    //
                    if intuple_attrs(&field.attrs).contains(&"recursive") {
                        let ty = field.ty.clone();
                        let index = syn::Index::from(ti.clone()-1);
                        qthat = quote!{ #ty::from_tuple(item.#index) };
                    }
                    //
                    qubuild_f = quote!{#qubuild_f #qthat,};
                    //
                    if !intuple_ignoref(&field.attrs) {
                        let ty = field.ty.clone();
                        if intuple_attrs(&field.attrs).contains(&"recursive") {
                            qubuild_t = quote!{ #qubuild_t <#ty as IntupleStruct>::Intuple, };
                            qubuild_i = quote!{ #qubuild_i self.#ident.into_tuple(), };
                        } else {
                            qubuild_t = quote!{ #qubuild_t #ty, };
                            qubuild_i = quote!{ #qubuild_i self.#ident, };
                        }
                    }
                    //
                    i += 1;
                }
                qout_from = quote!{ Self (#qubuild_f) };
                qout_into = quote!{ (#qubuild_i) };
            }
            /* ---------------------------------- Unit ---------------------------------- */
            syn::Fields::Unit => {
                qout_from = quote!{ Self };
                qout_into = quote!{ () };
            },
        }
        //
        let intuple = quote!{(#qubuild_t)};
        let typeid = Ident::new(&(name.to_string()+"Intuple"), Span::call_site());
        quote! {
            impl #impl_generics IntupleStruct for #name #ty_generics #where_clause {
                type Intuple = #intuple;
                fn from_tuple(item: Self::Intuple) -> Self{
                    #qout_from  
                }
                fn into_tuple(self) -> Self::Intuple {
                    #qout_into
                }
            }
            impl #impl_generics From<#intuple> for #name #ty_generics #where_clause {
                fn from(item: #intuple) -> Self { Self::from_tuple(item) }
            }
            impl #impl_generics From<#name #ty_generics> for #intuple #where_clause {
                fn from(item: #name #ty_generics) -> Self { item.into_tuple() }
            }
            impl #impl_generics Intuple<#name #ty_generics> for #intuple #where_clause {
                fn into_struct(self)->#name #ty_generics { self.into() }
            }
            #vis type #typeid #ty_generics = <#name #ty_generics as IntupleStruct>::Intuple;
        }.into()
        //
    } else {panic!("Not a Struct!")}

}