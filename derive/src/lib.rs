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
//#![doc(test(attr(deny(warnings))))]
#![warn(clippy::missing_docs_in_private_items)] // doc
#![warn(missing_docs)] // doc

//--
//#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize)]

mod getter;
mod new;
mod sealed;
#[cfg(any(test, doctest))] // cspell: ignore doctest
mod test;

use proc_macro::TokenStream;

/// Derive the `Sealed` trait
///
/// # Panic
///
/// panic if the derive macro is not applied to an struct, enum or union
///
/// # Example
///
/// ```
/// use utils_lib_derive::{trait_sealed, Sealed};
///
/// // this create a module named [`private`] with a trait named [`Sealed`]
/// // without method inside that module.
/// trait_sealed!();
///
/// #[derive(Sealed)]
/// struct S;
///
/// // this trait is sealed and cannot me implemented outside of this crate
/// // because [`Sealed`] is a private trait that can't be implemented outside
/// // of this crate.
/// pub trait Trait: private::Sealed {}
///
/// impl Trait for S {}
/// # fn main() {}
/// ```
#[inline]
#[must_use]
#[proc_macro_derive(Sealed)]
pub fn derive_sealed(item: TokenStream) -> TokenStream {
    sealed::derive(item)
}

/// Creates a trait `Sealed` into a private module `private`.
///
/// # Example
///
/// ```
/// use utils_lib_derive::{trait_sealed, Sealed};
///
/// // this create a module named [`private`] with a trait named [`Sealed`]
/// // without method inside that module.
/// trait_sealed!();
///
/// #[derive(Sealed)]
/// struct S;
///
/// // this trait is sealed and cannot me implemented outside of this crate
/// // because [`Sealed`] is a private trait that can't be implemented outside
/// // of this crate.
/// pub trait Trait: private::Sealed {}
///
/// impl Trait for S {}
/// # fn main() {}
/// ```
#[inline]
#[must_use]
#[proc_macro]
pub fn trait_sealed(item: TokenStream) -> TokenStream {
    sealed::trait_sealed(item)
}

// TODO doc
/// Derive getter macro
///
/// valid field attribute:
/// - `#[get]` for immutable getter
/// - `#[get_mut]` for mutable getter
///
/// Valid option for mutable getter :
/// - Name
/// - Visibility
///
/// Valid option for immutable getter :
/// - Name
/// - Visibility
/// - Constant type
/// - Getter type
/// - Self Type
///
/// ## Name
///
/// determine the name og the getter. By default it is the name of the field for
/// immutable getter and `{name}_mut` for mutable getter. It can be rename using
/// the option `name = "{name}"` or `name({name})` with `{name}` the name of the getter.
///
/// ### Example
/// ```
/// use utils_lib_derive::Getter;
///
/// #[derive(Getter)]
/// struct S {
///     #[get(name = "field")]
///     #[get_mut(name(mut_getter))]
///     f: usize,
///     #[get_mut] // by default the name of the getter is `c_mut`
///     #[get] // by default the name of the getter is `c`
///     c: char,
/// }
///
/// let mut s = S { f: 0, c: 'A' };
/// assert_eq!(s.field(), &0);
/// assert_eq!(s.mut_getter(), &mut 0);
///
/// assert_eq!(s.c(), &'A');
/// assert_eq!(s.c_mut(), &mut 'A');
/// ```
///
/// In the case of a tuple struct the name is a requirement.
/// ```compile_fail
/// use utils_lib_derive::Getter;
///
/// #[derive(Getter)]
/// struct Tuple(#[get] f32)
/// ```
/// should be changed to
/// ```
/// use utils_lib_derive::Getter;
///
/// #[derive(Getter)]
/// struct Tuple(#[get(name = "field")] f32);
///
/// let t = Tuple(0_f32);
/// assert_eq!(t.field(), &0_f32);
/// ```
///
/// ## Visibility
///
/// Determine the visibility of the getter, i.e. if it is private, public or restrained.
/// As for function in rust by default getter are private. It is possible to change
/// the visibility of the getter using the following syntax
///  accepted option :
/// - value:
///   - `Pub`
///   - `Crate`
///   - `pub` (wip)
///   - `public`
///   - `crate` (wip)
///   - `pub({path})` (wip)
///   - `private`
/// - `Visibility = "{value}"` with `{value}` a previously define value
/// - `Visibility({value})`
///
/// ### Example
///
/// TODO
///
/// ```
/// mod private {
///     use utils_lib_derive::Getter;
///
///     #[derive(Getter)]
///     pub struct S {
///         #[get(public)]
///         pub f: usize,
///     }
/// }
///
/// let mut s = private::S { f: 0 };
/// assert_eq!(s.f(), &0);
/// ```
///
/// ## Constant type
///
/// Determine if the function is constant or not. By default it is not but I would strongly
/// advice to make it constant.
/// accepted option :
/// - value:
///   - `Const`
///   - `const` (WIP)
/// - `{value} = {bool}`
/// - `{value}({bool})` (wip)
/// with `{bool}` a boolean.
///
/// ### Example
///
/// ```
/// use utils_lib_derive::Getter;
///
/// #[derive(Getter, Clone)]
/// struct S {
///     #[get(Const)]
///     f: usize,
/// }
///
/// const fn cst_fn(s: &S) -> &usize {
///     // we can call f() in a const fn as it is const
///     s.f()
/// }
/// ```
///
/// ## Getter type
///
/// Determine how the value is returned. It can be returned by reference, by copy or by clone.
/// An explicit definition can be found after the example.
/// By default the value is return by reference.
/// accepted option :
/// - value
///   - `by_ref`
///   - `by ref`
///   - `by_value` : copy type
///   - `by_copy`
///   - `copy`
///   - `Copy`
///   - `by_clone`
///   - `clone`
///   - `Clone`
/// - `{left} = "{value}"`
/// - `{left} ({value})`
/// with {left}
/// - `getter_ty`
/// - `Getter_ty`
/// - `getter_type`
/// - `Getter_type`
///
/// ### Example
///
/// ```
/// use utils_lib_derive::Getter;
///
/// #[derive(Getter, Clone)]
/// struct S {
///     // this just the default getter and return `&Vec<usize>`
///     #[get(getter_ty = "by_ref")]
///     f1: Vec<usize>,
///     #[get(by_copy)] // return a copy of the field
///     f2: i32,
///     #[get(getter_ty = "by_value", self_ty = "value")] // move self and return the string
///     f3: String,
///     #[get(getter_ty(Clone))] // return a copy of the field
///     f4: String,
/// }
///
/// let s = S {
///     f1: vec![0_usize],
///     f2: 0_i32,
///     f3: "s3".to_owned(),
///     f4: "s4".to_owned(),
/// };
///
/// assert_eq!(s.f1(), &vec![0_usize]);
///
/// let mut value = s.f2();
/// assert_eq!(value, 0_i32);
/// value = 1_i32;
/// assert_eq!(s.f2(), 0_i32);
///
/// assert_eq!(s.clone().f3(), "s3".to_owned()); // we need to clone as s is moved.
///
/// let mut string = s.f4();
/// assert_eq!(string, "s4".to_owned());
/// string = String::new();
/// assert_eq!(s.f4(), "s4".to_owned());
/// ```
///
/// let us show some properties. First lest go bac to the `clone` when using the `f3`
/// getter. As before
/// ```compile_fail
/// # use utils_lib_derive::Getter;
/// #
/// #[derive(Getter, Clone)]
/// struct S {
///     #[get(getter_ty = "by_value", self_ty = "value")]
///     f3: String,
/// #    #[get(getter_ty(Clone))]
/// #    f4: String,
/// }
///
/// let s = S {
///     f3: "s3".to_owned(),
/// #   f4: "s4".to_owned(),
/// };
///
/// assert_eq!(s.f3(), "s3".to_owned()); // we "forgot" to clone s which lead to an error
/// assert_eq!(s.f4(), "s4".to_owned());
/// ```
/// Another common common mistake is to use `by_value` (or `copy`) a non copy type
/// without using a `getter_ty = "by_value"`
/// ```compile_fail
/// use utils_lib_derive::Getter;
///
/// #[derive(Getter)]
/// struct S {
///     #[get(getter_ty = "by_value")]
///     f: Vec<()>,
/// }
/// ```
///
/// ### Definition
///
/// A getter type by copy means that we write
/// ```
/// # struct S {
/// #   field: u32,
/// # }
/// #
/// # impl S {
/// fn field(&self) -> u32 {
///     self.field
/// }
/// # }
/// ```
/// It works only for type that implements [`Copy`].
///
/// A getter type by clone means that we write
/// ```
/// # struct S {
/// #   field: String,
/// # }
/// #
/// # impl S {
/// fn field(&self) -> String {
///     self.field.clone()
/// }
/// # }
/// ```
/// It works only for type that implements [`Clone`].
///
/// A getter type by reference means that we write
/// ```
/// # struct S {
/// #   field: String,
/// # }
/// #
/// # impl S {
/// fn field(&self) -> &String {
///     &self.field
/// }
/// # }
/// ```
/// This is the default behavior and does not require any traits.
///
/// ## Self Type
///
/// Determine how self is handled. It is either used by reference or by value (or moved).
/// An explicit definition can be found after the example.
/// By default the self is referenced.
/// Note that using `self_ty = "value"` require that `getter_ty` to be by
/// `value` (or by `clone`).
/// accepted option :
/// - `{left} = "{right}"`
/// - `{left}({right})`
/// with `{left}`:
/// - `self_ty`
/// - `Self_ty`
/// - `self_type`
/// - `Self_type`
/// - `Self`
/// and `{right}`
/// - `value`
/// - `copy`
/// - `move`
/// - `ref`
///
/// ### Example
///
/// ```
/// use utils_lib_derive::Getter;
///
/// #[derive(Clone, Copy, Getter)]
/// struct S {
///     #[get(self_ty = "value", getter_ty = "copy")]
///     f: u32,
/// }
///
/// let s = S { f: 0_u32 };
/// assert_eq!(s.f(), 0_u32);
/// assert_eq!(s.f(), 0_u32);
///
/// #[derive(Clone, Getter)]
/// struct S2 {
///     #[get(Self_type(value), Getter_type(by_value))]
///     f: String,
/// }
///
/// let s = S2 {
///     f: "string".to_owned(),
/// };
/// assert_eq!(s.f(), "string".to_owned());
/// ```
/// The next example demonstrate that using `self_ty` as `value` but leaving `getter_ty`
/// as ref gives an error.
/// ```compile_fail
/// use utils_lib_derive::Getter;
///
/// [derive(Clone, Copy, Getter)]
/// struct S {
///     #[get(self_ty = "value", getter_ty = "ref")]
///     f: u32,
/// }
/// ```
///
/// ### Definition
///
/// A self type is referenced if we write
///  ```
/// # struct S {
/// #   field: String,
/// # }
/// #
/// # impl S {
/// fn field(&self) -> &String {
///     &self.field
/// }
/// # }
/// ```
/// 
/// A self type is moved if we write
/// ```
/// # struct S {
/// #   field: u32,
/// # }
/// #
/// # impl S {
/// fn field(self) -> u32 {
///     self.field
/// }
/// # }
/// ```
/// It is only recommended for Type that implement [`Copy`] and is smaller or equal in size
/// of an [`usize`] of your targeted platforms. Note also that the `getter_type` must be `by_value`
/// (or `clone`) and will give an error if left by default or set `by_ref`.
#[inline]
#[must_use]
#[proc_macro_derive(Getter, attributes(get, get_mut))]
pub fn derive_getter(item: TokenStream) -> TokenStream {
    getter::derive(item)
}

#[inline]
#[must_use]
#[proc_macro_derive(New, attributes(new))]
pub fn derive_new(item: TokenStream) -> TokenStream {
    new::derive(item)
}

// #[proc_macro_derive(Getter, attributes(get))]
// pub fn derive_getter(item: TokenStream) -> TokenStream {
//     // Let us find the inner part of the structure

//     let item: TokenStream2 = item.into();
//     let mut iter = item.into_iter();
//     let name = find_name(&mut iter).expect("no name found");

//     let Some(TokenTree2::Group(group)) =
//         iter.find(|el| matches!(el, TokenTree2::Group(gp) if gp.delimiter() == Delimiter::Bracket))
//     else {
//         panic!("no groupe found")
//     };

//     let stream = group.stream();

//     let vector = stream.into_iter().collect::<Vec<_>>();

//     let vector = vector
//         .split(|el| matches!(el, TokenTree2::Punct(p) if p.as_char() == ','))
//         .collect::<Vec<_>>();

//     let code_vec = Vec::<TokenStream2>::new();

//     const MESSAGE_MISFORMED: &str = "misformed structure, no first element";

//     for array in vector {
//         let Some(element) = array.first() else {
//             panic!("{}", MESSAGE_MISFORMED)
//         };

//         let function_add_code = |_config: (), _ident: &Ident, _ty: &Ident| todo!();

//         match element {
//             TokenTree2::Punct(p) if p.as_char() == '#' => {
//                 function_add_code((), todo!() /* array[2] */, todo!() /* array[4] */);
//             }
//             TokenTree2::Ident(ident) => {
//                 function_add_code((), ident, todo!() /* array[2] */);
//             }
//             _ => panic!("{}", MESSAGE_MISFORMED),
//         }
//     }

//     quote! {
//         impl #name {
//             #(#code_vec)*
//         }
//     }
//     .into()
// }
