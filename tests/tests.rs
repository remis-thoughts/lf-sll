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
fn test_concurrent() {
    let l = lfsll::List::<usize>::new();

    let t1 = thread::scoped(|| {
        for i in 0..500 { l.prepend(i) }
    });
    let t2 = thread::scoped(|| {
        for i in 0..500 { l.prepend(i) }
    });

    t1.join();
    t2.join();

	assert_eq!(1000, l.iter().count());
}
