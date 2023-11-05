mod minigrep_main_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn searches_case_sensitive() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("_12_minigrep")?;
        cmd.arg("to").arg("./tests/poem.txt");

        cmd.assert().success().stdout(predicate::str::contains("Are you nobody, too?\nHow dreary to be somebody!"));

        Ok(())
    }

    #[test]
    fn searches_case_insensitive() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("_12_minigrep")?;
        cmd.arg("PUBLIC").arg("./tests/poem.txt").env("IGNORE_CASE", "true");

        cmd.assert().success().stdout(predicate::str::contains("How public, like a frog"));

        Ok(())
    }
}
