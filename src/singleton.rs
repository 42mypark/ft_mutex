use core::mem::MaybeUninit;

use crate::mutex::spin_mutex::{MutexGuard, SpinMutex};

#[derive(Debug)]
pub struct Singleton<T> {
	data: SpinMutex<MaybeUninit<T>>,
}

impl<T> Singleton<T> {
	pub const fn uninit() -> Self {
		Self {
			data: SpinMutex::new(MaybeUninit::uninit()),
		}
	}

	pub fn lock<'a>(&'a self) -> SingletonGuard<'a, T> {
		SingletonGuard::new(self)
	}

	pub unsafe fn as_ptr(&self) -> *mut T {
		self.data.as_ptr().cast()
	}
}

pub struct SingletonGuard<'a, T> {
	lock: MutexGuard<'a, MaybeUninit<T>>,
}

impl<'a, T> SingletonGuard<'a, T> {
	pub fn new(single: &'a Singleton<T>) -> Self {
		let lock = single.data.lock();
		SingletonGuard { lock }
	}

	pub fn get_mut(&self) -> &mut T {
		unsafe { self.lock.get_mut().assume_init_mut() }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug)]
	struct A(usize);

	impl A {
		unsafe fn construct_at(ptr: *mut A, val: usize) {
			(*ptr).0 = val;
		}
	}

	static S: Singleton<A> = Singleton::uninit();

	#[test]
	fn test() {
		unsafe { A::construct_at(S.as_ptr(), 1) };
		println!("{:?}", S.lock().get_mut());

		S.lock().get_mut().0 = 2;

		println!("{:?}", S.lock().get_mut());
	}
}
