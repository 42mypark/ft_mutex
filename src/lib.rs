// use core::arch::asm;
use core::marker::Sized;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

struct SpinMutex<T: ?Sized> {
	lock_atomic: AtomicBool,
	pub value: T,
}

impl<T> SpinMutex<T> {
	const fn new(value: T) -> Self {
		SpinMutex {
			lock_atomic: AtomicBool::new(false),
			value,
		}
	}

	fn lock(&self) {
		while let Err(_) =
			self.lock_atomic
				.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
		{}
	}

	fn unlock(&mut self) {
		self.lock_atomic.store(false, Ordering::Relaxed);
	}
}

#[cfg(test)]
mod tests {
	static mut sum: SpinMutex<usize> = SpinMutex::new(0);
	use super::*;

	fn func() {
		for _ in 0..100000 {
			unsafe {
				sum.lock();
				sum.value += 1;
				sum.unlock();
			}
		}

		unsafe { println!("{}", sum.value) };
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
