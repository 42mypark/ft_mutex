use core::arch::asm;

static mut LOCK_ATOMIC: usize = 0;
static mut sum: usize = 0;

/// cmpxchg [m1], r1, al
/// if [m1] == al {
///     [m1] = r1;
/// } else {
///     al = [m1];
/// }

fn lock() {
	while unsafe { cmpxchg() } {}
}

fn unlock() {
	unsafe { LOCK_ATOMIC = 0 }
}

unsafe fn cmpxchg() -> bool {
	let mut al: i8 = 0;
	let cl: i8 = 1;
	let rdi = &LOCK_ATOMIC as *const usize;
	asm!(
		"lock cmpxchg [rdi], cl",
		in ("rdi") rdi,
		in ("cl") cl,
		inout ("al") al,
	);
	al == 1
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
