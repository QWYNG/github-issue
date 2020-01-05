extern crate assert_cli;

// TODO mockしたいんだけどどうにもうまくいかない
#[test]
fn integration_test() {
    {
        assert_cli::Assert::command(&["target/debug/github-issue", "rails", "rails"])
            .stdout()
            .contains("number")
            .stdout()
            .contains("title")
            .stdout()
            .contains("created_at")
            .unwrap();
    }
}
