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
fn ui_pass() {
    let t = trybuild::TestCases::new();
    t.pass("ui_test/pass/*.rs");
}

// run $env:TRYBUILD="overwrite"; cargo t -p utils-lib-derive
#[test]
fn ui_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("ui_test/fail/*.rs");
}
