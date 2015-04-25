#![feature(alloc)]

use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::boxed;
use std::default::Default;

struct ListElement<T> {
	next: AtomicPtr<ListElement<T>>,
	it: T,
}

pub struct ListIter<'a, T: 'a> {
	next: &'a AtomicPtr<ListElement<T>>,
}

pub struct List<T> {
	head: AtomicPtr<ListElement<T>>,
}

unsafe impl<T> Sync for List<T> {}
unsafe impl<T> Sync for ListElement<T> {}

impl<T> List<T> {
	pub fn new() -> List<T> {
		List { head: AtomicPtr::new(null_mut()) }
	}

	pub fn iter<'a>(&'a self) -> ListIter<'a, T> {
		return ListIter { next: &self.head };
	}

	pub fn prepend(&self, it: T) {
		unsafe {
			let next = boxed::into_raw(Box::new(ListElement {
				it: it,
				next: AtomicPtr::new(null_mut()),
			}));

			loop {
				let head_snapshot = self.head.load(Ordering::Acquire);
				(*next).next.store(head_snapshot, Ordering::Release);
				let old_val = self.head.compare_and_swap(
					head_snapshot,
					next,
					Ordering::Release);
				if old_val == head_snapshot {
					return
				}
			}
		}
	}
}

impl<T: PartialEq> List<T> {
	pub fn remove(&self, it: T) -> bool {
		let mut ptr = &self.head;
		loop {
			let before = ptr.load(Ordering::Acquire);
			if before == null_mut() {
				return false
			}

			unsafe {
				if (*before).it == it {
					let after = (*before).next.load(Ordering::Acquire);
					let old_val = ptr.compare_and_swap(
						before,
						after,
						Ordering::Release);
					if old_val == before {
						Box::from_raw(before); // release memory
						return true;
					}
				} else {
					ptr = &(*before).next;
				}
			}
		}
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut tail = self.head.swap(null_mut(), Ordering::Release);
		while tail != null_mut() {
			let recl = unsafe { Box::from_raw(tail) };
			tail = recl.next.load(Ordering::Acquire);
		}
	}
}

impl<'a, T> Iterator for ListIter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<&'a T> {
		let n = self.next.load(Ordering::Acquire);
		if n == null_mut() {
			return None;
		} else {
			unsafe {
				self.next = &(*n).next;
				return Some(&(*n).it);
			}
		}
	}
}

impl<T> Default for List<T> {
    fn default() -> List<T> {
        List::new()
    }
}
