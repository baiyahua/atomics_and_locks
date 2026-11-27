use std::{
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        let arc_data = unsafe { self.ptr.as_ref() };
        arc_data.ref_count.fetch_add(1, Relaxed);
        Self { ptr: self.ptr }
    }
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        let boxed = Box::new(ArcData {
            ref_count: AtomicUsize::new(1),
            data,
        });

        let ptr = NonNull::from_ref(Box::leak(boxed));

        Self { ptr }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let arc_data = unsafe { self.ptr.as_ref() };
        &arc_data.data
    }
}
