# aki-json-pick

The json pick out command.

## Features

- the json pick out command.
- minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)

## Command help

```
aki-json-pick --help
```

```
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

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-json-pick/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
