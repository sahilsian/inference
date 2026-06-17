#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker:: PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{*, atomic::{*, Ordering::*}},
    thread::{self, Thread}
};

fn main() {

    fn x() {
        println!("Hello World")
    }

    fn f(a: &Cell<i32>, b: &Cell<i32>) {
        let before = a.get();
        b.set(b.get() + 1);
        let after = a.get();

        if before != after {
            x();
        }
    }


    f(&Cell::new(3), &Cell::new(3));
}
