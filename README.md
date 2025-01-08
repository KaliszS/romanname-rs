# romanname - rust

Generator of random name styling on roman names. Still early version, so next patches may contain breaking changes.

### Features
- generate random name from 3 built-in parts (`praenomen` + `nomen` + `cognomen`)
- generate random name from 2 built-int parts (`nomen` + `cognomen`)
- control over duplicates (returns string `Failed to generate a unique name` if all options are used, it will be solved in a more elegant way later)
  - 41 452 double-barrelled names
  - 1 492 272 triple-barrelled names

### Usage

```rust
use romanname::{NameConfig, romanname};

fn main() {
    let name2p = romanname(NameConfig { praenomen: false });
    let name3p = romanname(NameConfig { praenomen: true });

    println!("2 parts: {:?}", name2p); // Julius Caesar
    println!("3 parts: {:?}", name3p); // Lucius Cornelius Sulla
}
```


