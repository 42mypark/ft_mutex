use mutex::inner_mutex::InnerMutex;

mod mutex;
mod singleton;

fn main() {
	let m = InnerMutex::new();

	m.lock();

	let a = 1;
	let b = 2;

	m.unlock();
}
