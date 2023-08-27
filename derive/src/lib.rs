#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/utils-lib-derive/0.1.0")]
//------
// main lints
//------
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
//
#![warn(clippy::absolute_paths)]
#![warn(clippy::allow_attributes)] // attributes
#![warn(clippy::allow_attributes_without_reason)] //attributes
#![warn(clippy::as_underscore)]
#![warn(clippy::assertions_on_result_states)]
#![warn(clippy::branches_sharing_code)]
#![warn(clippy::clear_with_drain)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::collection_is_never_read)]
#![warn(clippy::create_dir)]
#![warn(clippy::debug_assert_with_mut_call)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::default_union_representation)]
#![warn(clippy::disallowed_script_idents)] // cspell: ignore idents
#![warn(clippy::empty_drop)]
#![warn(clippy::empty_line_after_doc_comments)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::error_impl_error)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::impl_trait_in_params)]
#![warn(clippy::implicit_saturating_sub)]
#![warn(clippy::imprecise_flops)]
#![warn(clippy::iter_on_empty_collections)]
#![warn(clippy::iter_on_single_items)]
#![warn(clippy::iter_with_drain)]
#![warn(clippy::large_include_file)]
#![warn(clippy::large_stack_frames)]
#![warn(clippy::let_underscore_must_use)]
#![warn(clippy::let_underscore_untyped)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::manual_clamp)]
#![deny(clippy::exit)] // deny
#![warn(clippy::future_not_send)]
//#![warn(clippy::mem_forget)] // memory, mistake allow
#![warn(clippy::map_err_ignore)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_enforced_import_renames)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::mixed_read_write_in_expression)]
// #![warn(clippy::module_name_repetitions)] // allow
// #![warn(clippy::multiple_unsafe_ops_per_block)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::mutex_integer)]
#![warn(clippy::needless_collect)]
#![warn(clippy::needless_raw_strings)]
#![warn(clippy::nonstandard_macro_braces)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::option_if_let_else)]
#![warn(clippy::or_fun_call)]
#![warn(clippy::path_buf_push_overwrite)]
// #![warn(clippy::pattern_type_mismatch)] // maybe
// #![warn(clippy::ptr_as_ptr)] // allowed ?
#![warn(clippy::pub_without_shorthand)] // style choice
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::readonly_write_lock)]
#![warn(clippy::redundant_clone)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::same_name_method)]
#![deny(clippy::self_named_module_files)] // style, file
//#![deny(clippy::mod_module_files)]
#![warn(clippy::semicolon_outside_block)] // style
// cspell: ignore scrutinee
#![warn(clippy::significant_drop_in_scrutinee)] // maybe ?
#![warn(clippy::significant_drop_tightening)] // maybe ?
#![warn(clippy::str_to_string)] // style
#![warn(clippy::string_add)] // restriction, style
#![warn(clippy::string_lit_chars_any)] // perf
#![warn(clippy::string_to_string)] // mistake
#![warn(clippy::suboptimal_flops)] // precision
#![warn(clippy::suspicious_operation_groupings)] // mistake
#![warn(clippy::suspicious_xor_used_as_pow)] // mistake
#![warn(clippy::tests_outside_test_module)] // mistake, perf, readability
#![warn(clippy::todo)] // reminder
#![warn(clippy::trailing_empty_array)] // mistake
#![warn(clippy::trait_duplication_in_bounds)] // mistake, readability
// cspell: ignore repr
#![warn(clippy::transmute_undefined_repr)] // safety
#![warn(clippy::trivial_regex)] // perf, mistake
#![warn(clippy::try_err)] // restriction. style
#![warn(clippy::tuple_array_conversions)] // style
#![warn(clippy::type_repetition_in_bounds)] // style, mistake
#![warn(clippy::undocumented_unsafe_blocks)] // Doc
#![warn(clippy::unimplemented)] // reminder
#![warn(clippy::unnecessary_self_imports)] // style
#![warn(clippy::unnecessary_struct_initialization)] // style , readability
// cspell: ignore unseparated
#![warn(clippy::unseparated_literal_suffix)] // style
// cspell: ignore peekable
#![warn(clippy::unused_peekable)] // mistake
#![warn(clippy::unused_rounding)] // mistake, readability
#![warn(clippy::unwrap_in_result)] // mistake, error propagation
#![warn(clippy::unwrap_used)] // allow ? style
#![warn(clippy::use_debug)] // debug removing
#![warn(clippy::use_self)] // style
#![warn(clippy::useless_let_if_seq)] // style
#![warn(clippy::verbose_file_reads)] // style

//
//---------------
#![warn(missing_docs)] // doc
#![warn(clippy::missing_docs_in_private_items)] // doc

//--
//#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize)]

#[cfg(test)]
mod test;

use proc_macro::TokenStream;
use proc_macro2::{
    token_stream::IntoIter, Delimiter, Ident, Span, TokenStream as TokenStream2,
    TokenTree as TokenTree2,
};
use quote::quote;

//#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]

/// Find the name of the type of the derive macro
fn find_name(iter: &mut IntoIter) -> Ident {
    let Some(TokenTree2::Ident(i)) = iter.find(|el| {
        if let TokenTree2::Ident(ident) = el {
            let s = ident.to_string();
            s != "pub"
                && s != "crate"
                && s != "fn"
                && s != "struct"
                && s != "union"
                && s != "enum"
                && s != "for"
        } else {
            false
        }
    }) else {
        panic!("no name found")
    };

    Ident::new(i.to_string().as_str(), Span::call_site())
}

/// Derive the `Sealed` trait
#[proc_macro_derive(Sealed)]
pub fn derive_sealed(item: TokenStream) -> TokenStream {
    let item: TokenStream2 = item.into();
    let name = find_name(&mut item.into_iter());

    quote!(
        impl crate::private::Sealed for #name {}
    )
    .into()
}

#[proc_macro_derive(Getter, attributes(get))]
pub fn derive_getter(item: TokenStream) -> TokenStream {
    // Let us find the inner part of the structure

    let item: TokenStream2 = item.into();
    let mut iter = item.into_iter();
    let name = find_name(&mut iter);

    let Some(TokenTree2::Group(group)) =
        iter.find(|el| matches!(el, TokenTree2::Group(gp) if gp.delimiter() == Delimiter::Bracket))
    else {
        panic!("no groupe found")
    };

    let stream = group.stream();

    let vector = stream.into_iter().collect::<Vec<_>>();

    let vector = vector
        .split(|el| matches!(el, TokenTree2::Punct(p) if p.as_char() == ','))
        .collect::<Vec<_>>();

    let code_vec = Vec::<TokenStream2>::new();

    const MESSAGE_MISFORMED: &str = "misformed structure, no first element";

    for array in vector {
        let Some(element) = array.first() else {
            panic!("{}", MESSAGE_MISFORMED)
        };

        let function_add_code = |_config: (), _ident: &Ident, _ty: &Ident| todo!();

        match element {
            TokenTree2::Punct(p) if p.as_char() == '#' => {
                function_add_code((), todo!() /* array[2] */, todo!() /* array[4] */);
            }
            TokenTree2::Ident(ident) => {
                function_add_code((), ident, todo!() /* array[2] */);
            }
            _ => panic!("{}", MESSAGE_MISFORMED),
        }
    }

    quote! {
        impl #name {
            #(#code_vec)*
        }
    }
    .into()
}
