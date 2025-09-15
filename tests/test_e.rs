const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, [""]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: s\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_0_x_options_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option argument: X\n",
                "Missing option: s\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_invalid() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "red"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option argument: X: can not parse 'red'\n",
                "Missing option: s\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

const IN_DAT_01: &str = include_str!("../fixtures/01.json");
const IN_DAT_02: &str = include_str!("../fixtures/02.json");
const IN_DAT_03: &str = include_str!("../fixtures/03.json");
const IN_DAT_04: &str = include_str!("../fixtures/04.json");
const IN_DAT_05: &str = include_str!("../fixtures/05.json");
const IN_DAT_06: &str = include_str!("../fixtures/06.json");
const IN_DAT_07: &str = include_str!("../fixtures/07.json");
const IN_DAT_08: &str = include_str!("../fixtures/08.json");
const IN_DAT_09: &str = include_str!("../fixtures/09.json");
const IN_DAT_10: &str = include_str!("../fixtures/10.json");
const IN_DAT_11: &str = include_str!("../fixtures/11.json");

mod test_1_e {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t01() {
        // Root selection
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-s", "."], super::IN_DAT_01.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\"This is a valid JSON text with one value\"\n",);
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t02() {
        // Child selection
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"some\".\"property\""],
            super::IN_DAT_02.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\"yay!\"\n",);
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t03() {
        // Index selection
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"primes\".[0]"],
            super::IN_DAT_03.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "7\n",);
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"primes\"[0]"],
            super::IN_DAT_03.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "7\n",);
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"primes\".[2,0]"],
            super::IN_DAT_03.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[13,7]\n",);
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t04() {
        // Range selection
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"cats\".[1:2]"],
            super::IN_DAT_04.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "[{\"second\":\"Kitkat\"},{\"third\":\"Misty\"}]\n",
        );
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"cats\".[2:1]"],
            super::IN_DAT_04.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "[{\"third\":\"Misty\"},{\"second\":\"Kitkat\"}]\n",
        );
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"cats\".[2:1].[1:0]"],
            super::IN_DAT_04.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "[{\"second\":\"Kitkat\"},{\"third\":\"Misty\"}]\n",
        );
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"cats\".[2:1].[0].\"third\""],
            super::IN_DAT_04.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\"Misty\"\n",);
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"cats\".[1:]"],
            super::IN_DAT_04.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "[{\"second\":\"Kitkat\"},{\"third\":\"Misty\"}]\n",
        );
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"cats\".[:1]"],
            super::IN_DAT_04.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "[{\"first\":\"Pixie\"},{\"second\":\"Kitkat\"}]\n",
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t05() {
        // Array selection
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"primes\".[]"],
            super::IN_DAT_05.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[7,11,13]\n");
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"primes\".[0:2]"],
            super::IN_DAT_05.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[7,11,13]\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t06() {
        // Property selection
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"object\".{\"a\",\"c\"}"],
            super::IN_DAT_06.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "{\"a\":1,\"c\":3}\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t07() {
        // Multi-selection
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"one\".[2:0],\"two\",\"three\""],
            super::IN_DAT_07.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[[3,2,1],2,3]\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t08() {
        // Filter
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"laptops\"|\"laptop\""],
            super::IN_DAT_08.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[{\"brand\":\"Apple\",\"options\":[\"a\",\"b\",\"c\"]},{\"brand\":\"Asus\",\"options\":[\"d\",\"e\",\"f\"]}]\n");
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"laptops\"|\"laptop\".\"brand\""],
            super::IN_DAT_08.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[\"Apple\",\"Asus\"]\n");
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-s",
                "\"laptops\".[1:0]|\"laptop\".\"brand\",\"laptops\"|\"laptop\".\"brand\"",
            ],
            super::IN_DAT_08.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[[\"Asus\",\"Apple\"],[\"Apple\",\"Asus\"]]\n");
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-s",
                "\"laptops\".[1:0]|\"laptop\"|\"brand\",\"laptops\"|\"laptop\"|\"brand\"",
            ],
            super::IN_DAT_08.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[[\"Asus\",\"Apple\"],[\"Apple\",\"Asus\"]]\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t09() {
        // Flatten arrays
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "..\"dna\""],
            super::IN_DAT_09.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[\"c\",\"a\",\"c\",\"g\",\"t\",\"a\",\"t\"]\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t10() {
        // Truncate
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-s", ".!"], super::IN_DAT_10.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "{\"foo\":{}}\n");
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"foo\"!"],
            super::IN_DAT_10.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "{\"a\":null,\"b\":\"bar\",\"c\":1337,\"d\":{}}\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t11() {
        // Special characters
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\".valid\""],
            super::IN_DAT_11.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1337\n");
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-s", "\"\""], super::IN_DAT_11.as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\"yeah!\"\n");
        assert!(oup.status.success());
        //
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"\\\"\""],
            super::IN_DAT_11.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\"yup, valid too!\"\n");
        assert!(oup.status.success());
    }
}
