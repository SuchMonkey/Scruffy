# Scruffy

[![Build Status](https://travis-ci.org/SuchMonkey/Scruffy.svg?branch=master)](https://travis-ci.org/SuchMonkey/Scruffy)
[![Coverage Status](https://coveralls.io/repos/github/suchmonkey/scruffdy/badge.svg?branch=master)](https://coveralls.io/github/suchmonkey/scruffy?branch=master)

Scruffy is a simple pseudo random IP v4 address generator which uses specific offsets for each octet to generate deterministic IP addresses.
## Features

* **Deterministic**
  Create IP v4 addresses deterministically using the same offsets.

* **Shuffeling through pseudorandom generated IP addresses**
  Using an iterator...

## Adding Scruffy to your project

For now you have add the dependency from git directly.
Add the following line to your dependencies:

```toml
[dependencies]
scruffy = { git = "https://github.com/SuchMonkey/Scruffy.git" }
```

## Example

```rust
// Create a new scruffy type
// To generate unique addresses the offsets for each ocatat should not be factors of 255
let my_scruffy = Scruffy::new([231, 131, 61, 31]);

for ip_v4_addr in my_scruffy.take(10) {
    println!("{}", ip_v4_addr);
}

// Create a new scruffy type with random offsets
// These generated offsets will not be factors of 255
let rand_scruffy = Scruffy::new_rand();

// Retrieve offsets of the current scruffy type
let octets_offsets = rand_scruffy.get_octets_offsets();

// Rebuild the same scruffy with the given offsets
let rebuilt_scruffy = Scruffy::new(octets_offsets);

let a_set = rand_scruffy.take(10000).collect::<Vec<Ipv4Addr>>();
let b_set = rebuilt_scruffy.take(10000).collect::<Vec<Ipv4Addr>>();

assert_eq!(a_set, b_set);
```

## Contributing

Use the issue tracker to report problems, suggestions and questions. You may also contribute by submitting pull requests.

## TODO

* Maybe adding filters
* Specify IP range by providing a subnetmask
* Remove direct application of octat offsets and replace with a seed number. Removes need for providing working offsets.
* More tests
* Verify if addresses are unique and all possible addresses can be generated with correct offsets.
