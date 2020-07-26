use proc_macro2::TokenStream;

pub fn fragment_arguments_derive(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    use quote::quote;

    let ident = &ast.ident;
    Ok(quote! {
        impl AsRef<()> for #ident {
            fn as_ref(&self) -> &() {
                &()
            }
        }

        impl ::cynic::FragmentArguments for #ident {}

        impl<'a> ::cynic::SubArguments<'a, ()> for #ident
        {
            fn from_arguments(&'a self) -> &'a () {
                &()
            }
        }
    })
}
