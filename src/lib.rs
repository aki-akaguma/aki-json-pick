/*!
The json pick out command.

# Features

- the json pick out command.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

# Command help

```text
aki-json-pick --help
```

```text
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
```

# Examples

## Example 1: root selection

command line:
```text
cat fixtures/01.json | aki-json-pick -s '.'
```

result output:
```text
"This is a valid JSON text with one value"
```

## Example 2: child selection

command line:
```text
cat fixtures/01.json | aki-json-pick -s '"some"."property"'
```

result output:
```text
"yay!"
```

## Example 3: index selection

command line:
```text
cat fixtures/01.json | aki-json-pick -s '"primes".[0]'
```

result output:
```text
7
```

command line:
```text
cat fixtures/01.json | aki-json-pick -s '"primes"[0]"'
```

result output:
```text
7
```

command line:
```text
cat fixtures/01.json | aki-json-pick -s '"primes".[2,0]'
```

result output:
```text
[13,7]
```

# Reference

This crate use [jql](https://crates.io/crates/jql). The `selector` is comatible.

# Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
*/
#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

/// execute stats
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "json-pick"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// # Examples
///
/// ## Example 1: root selection
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_json_pick::execute(&RunnelIoeBuilder::new().build(),
///     "json-pick", &["-s", "."]);
/// ```
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{err}\n"));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
