# RanityETH 
Generate vanity ethereum addresses with rust.

## Usage

```js
ranityeth 0.1.2

USAGE:
    ranityeth [OPTIONS] --pattern <PATTERN> --strategy <STRATEGY>

OPTIONS:
        --bytecode <BYTECODE>    Bytecode of the contract for create2 [default: ]
    -c, --casesensitive          Whether the pattern is case sensitive
        --continuous             Continuous mode
        --contract               Search for a contract address
        --create2                Calculate the deployment address using create2, must set bytecode
                                 and deployer address
        --deployer <DEPLOYER>    Deployer address for create2 [default: ]
    -h, --help                   Print help information
    -p, --pattern <PATTERN>      The pattern to look for
    -s, --strategy <STRATEGY>    "contains", "startswith" or "trailing"
    -t, --threads <THREADS>      Number of threads to use [default: 1]
    -V, --version                Print version informationranityeth 0.1.2
```

## Example

```bash
$ ./vanityeth -p dead -s startswith -t 4 -c
Starting generation with 4 threads.
Private key: c2a6ce05488e5bacb8e4c2edc2bec4d8ae4572cbbeddb3564b52e2ca45887167
Address: 0xdeadE47Af1E325c4B5905818EC43F6bD44e18aCb
```
