use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering::{Acquire, Relaxed, Release},
        },
    },
};

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn send(&self, message: T) {
        unsafe {
            (*self.message.get()).write(message);
        }

        self.ready.store(true, Release);
    }

    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("only receive once");
        }

        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if self.ready.load(Relaxed) {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}

unsafe impl<T: Send> Sync for Channel<T> {}

pub struct Sender<T> {
    sender: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    receiver: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        self.sender.send(message);
    }
}

impl<T> Receiver<T> {
    pub fn receive(self) -> T {
        self.receiver.receive()
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel::new());
    (
        Sender {
            sender: Arc::clone(&channel),
        },
        Receiver { receiver: channel },
    )
}

// use VecDeque to release Channel
// use std::{
//     collections::VecDeque,
//     sync::{Condvar, Mutex},
// };

// pub struct Channel<T> {
//     queue: Mutex<VecDeque<T>>,
//     ready: Condvar,
// }

// impl<T> Channel<T> {
//     pub fn new() -> Self {
//         Self {
//             queue: Mutex::new(VecDeque::new()),
//             ready: Condvar::new(),
//         }
//     }

//     pub fn send(&self, message: T) {
//         self.queue.lock().unwrap().push_back(message);
//         self.ready.notify_one();
//     }

//     pub fn receive(&self) -> T {
//         let mut guard = self.queue.lock().unwrap();
//         loop {
//             if let Some(message) = guard.pop_front() {
//                 return message;
//             }

//             guard = self.ready.wait(guard).unwrap();
//         }
//     }
// }
