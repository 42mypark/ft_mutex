use core::arch::asm;
use core::marker::Sized;

struct SpinMutex<T: ?Sized> {
	lock_atomic: usize,
	pub value: T,
}

impl<T> SpinMutex<T> {
	const fn new(value: T) -> Self {
		SpinMutex {
			lock_atomic: 0,
			value,
		}
	}

	fn lock(&self) {
		while unsafe { self.cmpxchg() } {}
	}

	fn unlock(&mut self) {
		self.lock_atomic = 0;
	}

	/// cmpxchg [m1], r1, al
	/// if [m1] == al {
	///     [m1] = r1;
	/// } else {
	///     al = [m1];
	/// }
	unsafe fn cmpxchg(&self) -> bool {
		let mut al: i8 = 0;
		let cl: i8 = 1;
		let rdi = &self.lock_atomic as *const usize;
		asm!(
			"lock cmpxchg [rdi], cl",
			in ("rdi") rdi,
			in ("cl") cl,
			inout ("al") al,
		);
		al == 1
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
