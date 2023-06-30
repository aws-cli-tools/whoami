#[allow(unused_imports)]
mod cli_tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use rand::Rng;
    use std::process::Command;

    #[cfg(feature = "aws_configured")]
    #[test]
    fn wrong_profile() {
        let mut rng = rand::thread_rng();
        let mut cmd = Command::cargo_bin("whoami").unwrap();

        cmd.arg("--profile").arg(format!("{}", rng.gen::<u8>()));
        cmd.assert().failure().stderr(predicate::str::contains(
            "Failed loading AWS config details",
        ));
    }

    #[cfg(feature = "aws_configured")]
    #[test]
    fn happy_flow() {
        let mut cmd = Command::cargo_bin("whoami").unwrap();

        cmd.assert()
            .success()
            .stdout(predicate::str::contains("UserARN"));
    }
}
