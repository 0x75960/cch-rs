pub trait CacheDriver {
	type Item;
	type Key;
	fn add(&mut self, item: Self::Item);
	fn has_item(&mut self, key: Self::Key) -> bool;
	fn remove(&mut self, _item: Self::Key) {}
	fn dump(&mut self) {}
	fn load(&mut self) {}
}

pub struct Cache<T: CacheDriver> {
	driver: T,
}

impl<T: CacheDriver> Cache<T> {
	pub fn with(mut d: T) -> Cache<T> {

		// load cache already exists
		d.load();

		Cache{
			driver: d,
		}
	}

	pub fn has_item(&mut self, key: T::Key) -> bool {
		self.driver.has_item(key)
	}

	pub fn remove(&mut self, key: T::Key) {
		self.driver.remove(key);
	}

	pub fn add(&mut self, item: T::Item) {
		self.driver.add(item);
	}
}

impl<T: CacheDriver> Drop for Cache<T> {
	fn drop(&mut self) {
		self.driver.dump();
	}
}

#[cfg(test)]
mod tests {

	use super::*;

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

	#[test]
	fn it_works() {
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
}
