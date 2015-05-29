
# find_folder [![Build Status](https://travis-ci.org/PistonDevelopers/find_folder.svg?branch=master)](https://travis-ci.org/PistonDevelopers/find_folder)

A simple tool for finding the absolute path to a folder with a given name.

It looks like this:

```Rust
extern crate find_folder;

use find_folder::Search;

fn main() {
    println!("{:?}", Search::Parents(3).for_folder("src"));
    println!("{:?}", Search::Kids(3).for_folder("examples"));
    println!("{:?}", Search::Both(3, 3).for_folder("target"));
}
```

You can add it to your project by adding this to your Cargo.toml:

```toml
[dependencies]
find_folder = "*"
```


