extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{TokenStreamExt as _, quote};
use unsynn::*;
use unsynn::{Parse, Parser};

keyword! {
    KPub = "pub";
    KImpl = "impl";
    KFn = "fn"
}

unsynn! {
    struct Impl {
        _impl_keyword: KImpl,
        type_name: Ident,
        content: BraceGroupContaining<Option<Many<ImplFunction>>>
    }

    enum Vis {
        PubScoped(Cons<KPub, ParenthesisGroup>),
        Pub(KPub)
    }

    struct ImplFunction {
        _vis: Option<Vis>,
        _fn: KFn,
        function_name: Ident,
        arguments: ParenthesisGroupContaining<CommaDelimitedVec<Argument>>,
        return_type: Option<ReturnType>,
        implementation: BraceGroup
    }

    struct ReturnType {
        _arrow: RArrow,
        return_type: Ident
    }

    struct Argument {
        content: ParenthesisGroup
    }
}

#[proc_macro_attribute]
pub fn service_module(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let est = proc_macro2::TokenStream::from(input.clone());
    let test = est.to_token_iter().parse::<Impl>().unwrap();
    let ident = test.type_name.clone();
    // let ident = format!("{}Extension", test.type_name);

    let input = proc_macro2::TokenStream::from(input.clone());
    let trait_ident = Ident::new(&format!("{}Rusm", test.type_name), test.type_name.span());

    let trait_functions = test
        .content
        .content
        .as_ref()
        .map(|v| v.iter())
        .unwrap_or_default()
        .into_iter()
        .map(|f| {
            let function_name = f.value.function_name.clone();
            quote! {
                fn #function_name();
            }
        })
        .collect::<Vec<TokenStream>>();

    let impl_functions = test
        .content
        .content
        .map(|v| v.into_iter())
        .unwrap_or_default()
        .into_iter()
        .map(|f| {
            let function_name = f.value.function_name;
            let implementation = f.value.implementation.0;
            quote! {
                fn #function_name() #implementation
            }
        })
        .collect::<Vec<TokenStream>>();

    quote! {
        #input

        mod output {
            use super::*;

            trait #trait_ident: ServiceModule {
                #(#trait_functions)*
            }

            impl #trait_ident for #ident {
                #(#impl_functions)*
            }
        }
    }
    .into()
}
