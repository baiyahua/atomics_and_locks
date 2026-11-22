use std::{thread::sleep, time::Duration};

use atomics_locks::channel::channel;

fn main() {
    let (sender, receiver) = channel();
    std::thread::scope(|s| {
        s.spawn(|| {
            sleep(Duration::from_secs(2));
            sender.send("hello");
        });
    });

    let message = receiver.receive();

    println!("{message}");
}

// use atomics_locks::spinlock::SpinLock;

// fn main() {
//     let spin_lock = SpinLock::new(1);

//     std::thread::scope(|s| {
//         for _ in 0..10 {
//             s.spawn(|| {
//                 let mut guard = spin_lock.lock();
//                 for _ in 0..1000 {
//                     *guard += 1;
//                 }
//             });
//         }
//     });

//     let guard = spin_lock.lock();
//     println!("{}", *guard)
// }
