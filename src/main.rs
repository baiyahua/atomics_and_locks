use atomics_locks::spinlock::SpinLock;

fn main() {
    let spin_lock = SpinLock::new(0);

    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = spin_lock.lock();
                for _ in 0..1000 {
                    *guard += 1;
                }
            });
        }
    });

    let guard = spin_lock.lock();
    println!("{}", *guard);
}
