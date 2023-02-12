use core::cell::Cell;
use core::marker::Sized;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use std::sync::Mutex;

struct SpinMutex<T: ?Sized> {
	lock_atomic: AtomicBool,
	value: Cell<T>,
}
unsafe impl<T> Sync for SpinMutex<T> {} // 안전한가?

impl<T> SpinMutex<T> {
	const fn new(value: T) -> Self {
		SpinMutex {
			lock_atomic: AtomicBool::new(false),
			value: Cell::new(value),
		}
	}

	fn lock(&self) {
		while let Err(_) =
			self.lock_atomic
				.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
		{}
	}

	fn unlock(&self) {
		self.lock_atomic.store(false, Ordering::Relaxed);
	}
}

#[cfg(test)]
mod tests {
	static sum: SpinMutex<usize> = SpinMutex::new(0);
	use super::*;

	fn func() {
		for _ in 0..100000 {
			sum.lock();
			sum.value.set(sum.value.get() + 1);
			sum.unlock();
		}

		println!("{}", sum.value.get());
	}

	#[test]
	fn it_works() {
		let mut v: Vec<std::thread::JoinHandle<()>> = vec![];
		for _ in 0..3 {
			v.push(std::thread::spawn(func));
		}

		for h in v {
			h.join();
		}
	}
}
