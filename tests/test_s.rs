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

/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
*/

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .perr()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.perr().lock().buffer_str()
    };
    ($sioe:expr, sout) => {
        $sioe.pout().lock().buffer_str()
    };
}

mod test_s0 {
    use libaki_json_pick::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: s\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
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

mod test_s1 {
    use libaki_json_pick::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t01() {
        // Root selection
        let (r, sioe) = do_execute!(&["-s", "."], super::IN_DAT_01);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "\"This is a valid JSON text with one value\"\n",
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t02() {
        // Child selection
        let (r, sioe) = do_execute!(&["-s", "\"some\".\"property\""], super::IN_DAT_02);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "\"yay!\"\n",);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t03() {
        // Index selection
        let (r, sioe) = do_execute!(&["-s", "\"primes\".[0]"], super::IN_DAT_03);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "7\n",);
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"primes\"[0]"], super::IN_DAT_03);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "7\n",);
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"primes\"[2,0]"], super::IN_DAT_03);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "[13,7]\n",);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t04() {
        // Range selection
        let (r, sioe) = do_execute!(&["-s", "\"cats\".[1:2]"], super::IN_DAT_04);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[{\"second\":\"Kitkat\"},{\"third\":\"Misty\"}]\n",
        );
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"cats\".[2:1]"], super::IN_DAT_04);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[{\"third\":\"Misty\"},{\"second\":\"Kitkat\"}]\n",
        );
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"cats\".[2:1].[1:0]"], super::IN_DAT_04);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[{\"second\":\"Kitkat\"},{\"third\":\"Misty\"}]\n",
        );
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"cats\".[2:1].[0].\"third\""], super::IN_DAT_04);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "\"Misty\"\n",);
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"cats\".[1:]"], super::IN_DAT_04);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[{\"second\":\"Kitkat\"},{\"third\":\"Misty\"}]\n",
        );
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"cats\".[:1]"], super::IN_DAT_04);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[{\"first\":\"Pixie\"},{\"second\":\"Kitkat\"}]\n",
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t05() {
        // Array selection
        let (r, sioe) = do_execute!(&["-s", "\"primes\".[]"], super::IN_DAT_05);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "[7,11,13]\n",);
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"primes\".[0:2]"], super::IN_DAT_05);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "[7,11,13]\n",);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t06() {
        // Property selection
        let (r, sioe) = do_execute!(&["-s", "\"object\".{\"a\",\"c\"}"], super::IN_DAT_06);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "{\"a\":1,\"c\":3}\n",);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t07() {
        // Multi-selection
        let (r, sioe) = do_execute!(&["-s", "\"one\".[2:0],\"two\",\"three\""], super::IN_DAT_07);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "[[3,2,1],2,3]\n",);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t08() {
        // Filter
        let (r, sioe) = do_execute!(&["-s", "\"laptops\"|\"laptop\""], super::IN_DAT_08);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "[{\"brand\":\"Apple\",\"options\":[\"a\",\"b\",\"c\"]},{\"brand\":\"Asus\",\"options\":[\"d\",\"e\",\"f\"]}]\n",);
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(
            &["-s", "\"laptops\"|\"laptop\".\"brand\""],
            super::IN_DAT_08
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "[\"Apple\",\"Asus\"]\n",);
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(
            &[
                "-s",
                "\"laptops\".[1:0]|\"laptop\".\"brand\",\"laptops\"|\"laptop\".\"brand\""
            ],
            super::IN_DAT_08
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[[\"Asus\",\"Apple\"],[\"Apple\",\"Asus\"]]\n",
        );
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(
            &[
                "-s",
                "\"laptops\".[1:0]|\"laptop\"|\"brand\",\"laptops\"|\"laptop\"|\"brand\""
            ],
            super::IN_DAT_08
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[[\"Asus\",\"Apple\"],[\"Apple\",\"Asus\"]]\n",
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t09() {
        // Flatten arrays
        let (r, sioe) = do_execute!(&["-s", "..\"dna\""], super::IN_DAT_09);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "[\"c\",\"a\",\"c\",\"g\",\"t\",\"a\",\"t\"]\n",
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t10() {
        // Truncate
        let (r, sioe) = do_execute!(&["-s", ".!"], super::IN_DAT_10);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "{\"foo\":{}}\n",);
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"foo\"!"], super::IN_DAT_10);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "{\"a\":null,\"b\":\"bar\",\"c\":1337,\"d\":{}}\n",
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t11() {
        // Special characters
        let (r, sioe) = do_execute!(&["-s", "\".valid\""], super::IN_DAT_11);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1337\n");
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"\""], super::IN_DAT_11);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "\"yeah!\"\n");
        assert!(r.is_ok());
        //
        let (r, sioe) = do_execute!(&["-s", "\"\\\"\""], super::IN_DAT_11);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "\"yup, valid too!\"\n");
        assert!(r.is_ok());
    }
}
