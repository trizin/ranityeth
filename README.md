# Rust Vanity ETH Address
Generate vanity ethereum addresses with rust.

## Usage

```js
vanityeth 0.1.0

USAGE:
    vanityeth [OPTIONS] --pattern <PATTERN> --strategy <STRATEGY>

OPTIONS:
    -c, --casesensitive          Whether the pattern is case sensitive
        --contract               Search for a contract address
    -h, --help                   Print help information
    -p, --pattern <PATTERN>      The pattern to look for
    -s, --strategy <STRATEGY>    Either use "contains" or "startswith"
    -t, --threads <THREADS>      Number of threads to use [default: 1]
    -V, --version                Print version information
```

## Example

```bash
$ ./vanityeth -p dead -s startswith -t 4 -c
Starting generation with 4 threads.
Private key: c2a6ce05488e5bacb8e4c2edc2bec4d8ae4572cbbeddb3564b52e2ca45887167
Address: 0xdeadE47Af1E325c4B5905818EC43F6bD44e18aCb
```
