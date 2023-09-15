use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{
    parse::{Parse, ParseStream},
    Lit, MacroDelimiter, Token,
};

#[derive(Clone)]
enum AcceptedSyntax {
    Value(Value),
    NameValue(NameValue),
    List(List),
}

impl Parse for AcceptedSyntax {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Clone)]
enum Value {
    Ident(Ident),
    Visibility(syn::Visibility),
    Const(Token!(const)),
}

#[derive(Clone)]
enum LeftHandValue {
    Ident(Ident),
    Const(Token!(const)),
}

#[derive(Clone)]
enum RightHandValue {
    Ident(Ident),
    Visibility(syn::Visibility),
    Literal(Lit),
}

#[derive(Clone)]
struct List {
    left_hand: LeftHandValue,
    delimiter: MacroDelimiter,
    tokens: TokenStream2,
}

#[derive(Clone)]
struct NameValue {
    left_hand: LeftHandValue,
    eq: Token!(=),
    right_hand: RightHandValue,
}

#[derive(Clone, Copy)]
enum AcceptedToken {
    Const(Token!(const)),
}

impl Parse for AcceptedToken {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
