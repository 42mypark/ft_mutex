use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

#[derive(Debug)]
pub struct InnerMutex {
	lock_atomic: AtomicBool,
}

unsafe impl Sync for InnerMutex {}

impl InnerMutex {
	pub const fn new() -> Self {
		InnerMutex {
			lock_atomic: AtomicBool::new(false),
		}
	}

	#[no_mangle]
	pub fn lock(&self) {
		while let Err(_) =
			self.lock_atomic
				.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
		{}
	}

	pub fn try_lock(&self) -> Result<(), ()> {
		match self
			.lock_atomic
			.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire)
		{
			Ok(_) => Ok(()),
			Err(_) => Err(()),
		}
	}

	#[no_mangle]
	pub fn unlock(&self) {
		self.lock_atomic.store(false, Ordering::Release);
	}
}
