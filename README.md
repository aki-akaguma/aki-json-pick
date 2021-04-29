# aki-json-pick

*aki-json-pick* is the json pick out command.

## Features

-

* command help

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

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-json-pick
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

### Example 1: root selection

command line:
```
cat fixtures/01.json | aki-json-pick -s '.'
```

result output:
```
"This is a valid JSON text with one value"
```

### Example 2: child selection

command line:
```
cat fixtures/01.json | aki-json-pick -s '"some"."property"'
```

result output:
```
"yay!"
```

### Example 3: index selection

command line:
```
cat fixtures/01.json | aki-json-pick -s '"primes".[0]'
```

result output:
```
7
```

command line:
```
cat fixtures/01.json | aki-json-pick -s '"primes"[0]"'
```

result output:
```
7
```

command line:
```
cat fixtures/01.json | aki-json-pick -s '"primes".[2,0]'
```

result output:
```
[13,7]
```

## Reference

This crate use [jql](https://crates.io/crates/jql). The `selector` is comatible.

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-json-pick/blob/main/CHANGELOG.md)
