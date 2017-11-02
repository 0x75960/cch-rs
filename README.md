cch.rs
=======

Simple Cache provider

usage
------

1. add dependency into Cargo.toml

```toml
[dependencies]
cch = { git = "https://github.com/0x75960/cch-rs", branch = "master" }
```

2. import and use in your code

```rust
extern crate cch;

use cch::*;

struct TestCacheDriver {
	seen: Vec<i32>,
}

impl CacheDriver for TestCacheDriver {
	type Item = i32;
	type Key = i32;

	fn add(&mut self, item: Self::Item) {
		self.seen.push(item);
	}

	fn has_item(&mut self, key: Self::Key) -> bool {
		for i in &self.seen {
			if *i == key {
				return true;
			}
		}
		false
	}

	fn remove(&mut self, _key: Self::Key) {
		self.seen.pop();
	}

	fn dump(&mut self) {
		assert!(self.seen.len() == 6);
	}

	fn load(&mut self) {
		assert!(self.seen.len() == 0);
		for i in vec![1, 2, 3] {
			self.seen.push(i);
		}
	}
}

fn main() {
	let driver = TestCacheDriver{
		seen: vec![]
	};

	let mut cache = Cache::with(driver);

	for i in vec![1, 2, 3] {
		// check initial cache
		assert_eq!(cache.has_item(i), true);
	}

	for i in vec![4, 5, 6] {
		// check not cached yet
		assert_eq!(cache.has_item(i), false);
		// and cache item
		cache.add(i);
	}

	for i in vec![4, 5, 6] {
		// check cached
		assert_eq!(cache.has_item(i), true);
	}

	cache.add(1);
	cache.remove(1);
}
```

test
-----

```sh
cargo test
```
