#![feature(scoped)]

extern crate lfsll;

use std::thread;

#[test]
fn test_prepend() {
	let l = lfsll::List::<usize>::new();
	l.prepend(4);
	l.prepend(2);
	l.prepend(10);

	{
		let arr: Vec<&usize> = l.iter().collect();
		assert_eq!(vec![&10, &2, &4], arr);
	}

	l.prepend(3);

	{
		let arr: Vec<&usize> = l.iter().collect();
		assert_eq!(vec![&3, &10, &2, &4], arr);
	}
}

#[test]
fn test_remove() {
	let l = lfsll::List::<usize>::new();

	assert!(!l.remove(5));

	l.prepend(2);
	assert!(!l.remove(5));
	assert!(l.remove(2));
	assert_eq!(0, l.iter().count());

	l.prepend(2);
	l.prepend(4);
	assert!(!l.remove(5));
	assert!(l.remove(2));
	assert_eq!(1, l.iter().count());
}

#[test]
fn test_concurrent() {
    let l = lfsll::List::<usize>::new();

    let t1 = thread::scoped(|| {
        for i in 0..500 { l.prepend(i); }
        for i in 0..500 { l.remove(i); }
    });
    let t2 = thread::scoped(|| {
        for i in 500..1000 { l.prepend(i); }
        for i in 500..1000 { l.remove(i); }
    });

    t1.join();
    t2.join();

	assert_eq!(0, l.iter().count());
}
