use syn::{
    parse::{Parse, ParseStream},
    Meta,
};

#[allow(dead_code)] // TODO
#[derive(Clone)]
enum AcceptedSyntax {
    Meta(Meta),
    AcceptedToken(AcceptedToken),
}

impl Parse for AcceptedSyntax {
    #[allow(clippy::todo)]
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AcceptedToken {}

impl Parse for AcceptedToken {
    #[allow(clippy::todo)]
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
