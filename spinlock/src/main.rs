use spinlock::spinlock::SpinLock;

fn main() {
    let spin_lock = SpinLock::new(String::from("hello"));

    std::thread::scope(|s| {
        // for _ in 0..10 {
        //     s.spawn(|| {
        //         for _ in 0..100 {
        //             let mut guard = spin_lock.lock();
        //             *guard += 1;
        //         }
        //     });
        // }
        let handle1 = s.spawn(|| {
            let mut guard = spin_lock.lock();
            guard.push_str(" world");
        });

        let handle2 = s.spawn(|| {
            let guard = spin_lock.lock();
            println!("{}", *guard);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    });
}
