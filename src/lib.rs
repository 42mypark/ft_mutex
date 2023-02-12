use core::arch::asm;

static mut LOCK_ATOMIC: usize = 1;
static mut sum: usize = 0;

fn lock() {
	unsafe {
		let mut al: i8 = 0;
		let cl: i8 = 0;
		let mut rdi = &LOCK_ATOMIC as *const usize;

		while al == 0 {
			al = 1;
			asm!(
				"lock cmpxchg [rdi], cl",
				inout ("rdi") rdi,
				in ("cl") cl,
				inout ("al") al,
			)
		}
	}
}

fn unlock() {
	unsafe { LOCK_ATOMIC = 1 }
}

fn func() {
	for _ in 0..100000 {
		lock();
		unsafe {
			sum += 1;
		}
		unlock();
	}

	unsafe { println!("{sum}") };
}

#[cfg(test)]
mod tests {
	use super::*;

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
