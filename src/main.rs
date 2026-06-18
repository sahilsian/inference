
#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker:: PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    time::Duration,
    rc::Rc,
    sync::{*, atomic::{*, Ordering::*}},
    thread::{self, Thread},
    time::Instant
};

fn main() {
    fn increment() {
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        let mut id = NEXT_ID.load(Relaxed);
        loop {
            assert!(id < 1000, "Too many IDs!");
            match NEXT_ID.compare_exchange_weak(id, id+1, Relaxed, Relaxed) {
                Ok(_) => return,
                Err(v) => id = v
            }
        }
    }
}
