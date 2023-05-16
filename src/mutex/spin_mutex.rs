use core::cell::UnsafeCell;

use super::inner_mutex::InnerMutex;

#[derive(Debug)]
pub struct SpinMutex<T> {
	inner: InnerMutex,
	value: UnsafeCell<T>,
}
unsafe impl<T> Sync for SpinMutex<T> {}

impl<T> SpinMutex<T> {
	pub const fn new(value: T) -> Self {
		SpinMutex {
			inner: InnerMutex::new(),
			value: UnsafeCell::new(value),
		}
	}

	pub fn as_ptr(&self) -> *mut T {
		self.value.get()
	}

	pub fn lock<'g>(&'g self) -> MutexGuard<'g, T> {
		self.inner.lock();
		MutexGuard::new(self)
	}

	pub fn try_lock<'g>(&'g self) -> Result<MutexGuard<'g, T>, ()> {
		let lock_result = self.inner.try_lock();

		lock_result.map(|_| MutexGuard::new(self))
	}

	pub fn unlock(&self) {
		self.inner.unlock();
	}
}

// impl<T: ?Sized> SpinMutex<T> {}

pub struct MutexGuard<'lock, T> {
	mutex: &'lock SpinMutex<T>,
}

impl<'lock, T> MutexGuard<'lock, T> {
	pub fn new(mutex: &'lock SpinMutex<T>) -> Self {
		MutexGuard { mutex }
	}

	pub fn get_mut(&self) -> &mut T {
		unsafe { &mut *self.mutex.value.get() }
	}

	pub fn get(&mut self) -> &T {
		unsafe { &*self.mutex.value.get() }
	}
}

impl<'lock, T> Drop for MutexGuard<'lock, T> {
	fn drop(&mut self) {
		// println!("drop called");
		self.mutex.unlock();
	}
}

#[cfg(test)]
mod tests {
	static sum: SpinMutex<usize> = SpinMutex::new(0);
	use super::*;
	use std::sync::Mutex;

	fn add_one(a: &mut usize) {
		*a += 1;
	}

	fn func() {
		for _ in 0..100000 {
			// let gaurd = sum.lock();
			// *data += 1;
			// (*sum.lock().get_mut()) += 1;
			add_one(sum.lock().get_mut())
			// drop(gaurd)
		}

		println!("{}", unsafe { *sum.value.get() });
	}

	#[test]
	fn it_works() {
		let mut v: Vec<std::thread::JoinHandle<()>> = vec![];
		for _ in 0..8 {
			v.push(std::thread::spawn(func));
		}

		for h in v {
			h.join();
		}
	}
}
