#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}

// run $env:TRYBUILD="overwrite"; cargo t -p utils-lib-derive
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    //t.compile_fail("ui_test/fail/*.rs");
    t.pass("ui_test/pass/*.rs");
}
