#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! x_help_msg {
    () => {
        concat!(
            indoc::indoc!(
                r#"
            Options:
              -X rust-version-info     display rust version info and exit
            "#
            ),
            "\n",
        )
    };
}

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-json-pick"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}
