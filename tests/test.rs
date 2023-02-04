const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(r#"
            Usage:
              aki-json-pick [options]

            The json pick out command.

            Options:
                  --color <when>        json colored output.
              -s, --select <selector>   pick out json value by <selector>.
              -p, --pretty              pretty output.
              -r, --raw-output          raw string output without JSON double-quote.

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Option Parameters:
              <when>        'always', 'never', or 'auto'
              <selector>    json selector

            Examples:
              pick out some.property value:
                echo -e '{ "some": { "property": "yay!" } }' | aki-json-pick -s '"some"."property"'
            "#),
            "\n",
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-json-pick"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

mod test_0 {
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
} // mod test_0

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

mod test_1 {
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
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-s", "\"\""],
            super::IN_DAT_11.as_bytes(),
        );
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
