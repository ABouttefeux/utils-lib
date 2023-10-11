//! Contains the `version_sync` and `trybuild` tests

#[cfg(test)]
mod version_sync {
    #[test]
    fn test_readme_deps() {
        version_sync::assert_markdown_deps_updated!("README.md");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }
}

#[cfg(test)]
mod trybuild {
    // run $env:TRYBUILD="overwrite"; cargo t -p utils-lib-derive
    // we run the test even during test coverage just to make sure that the test pass the ui
    // component
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.pass("ui_test/pass/*.rs");
        t.compile_fail("ui_test/fail/*.rs");
    }
}

/// ```compile_fail
#[doc = include_str!("../../ui_test/fail/get_enum.rs")]
/// ```
/// ```compile_fail
#[doc = include_str!("../../ui_test/fail/get_move_on_ref.rs")]
/// ```
/// ```compile_fail
#[doc = include_str!("../../ui_test/fail/get_repetition.rs")]
/// ```
/// ```compile_fail
#[doc = include_str!("../../ui_test/fail/get_type.rs")]
/// ```
/// ```compile_fail
#[doc = include_str!("../../ui_test/fail/get_unacceptable_parse_error.rs")]
/// ```
/// ```compile_fail
#[doc = include_str!("../../ui_test/fail/get.rs")]
/// ```
/// ```compile_fail
#[doc = include_str!("../../ui_test/fail/trait_sealed.rs")]
/// ```
/// ```
#[doc = include_str!("../../ui_test/pass/get_const.rs")]
/// ```
/// ```
#[doc = include_str!("../../ui_test/pass/get_mut.rs")]
/// ```
/// ```
#[doc = include_str!("../../ui_test/pass/get_name.rs")]
/// ```
/// ```
#[doc = include_str!("../../ui_test/pass/get_visibility.rs")]
/// ```
/// ```
#[doc = include_str!("../../ui_test/pass/sealed.rs")]
/// ```
/// ```
#[doc = include_str!("../../ui_test/pass/trait_sealed.rs")]
/// ```
#[cfg(all(feature = "coverage", doc))]
mod coverage {}
