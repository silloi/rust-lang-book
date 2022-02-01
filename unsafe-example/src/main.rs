unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

static HELLO_WORLD: &str = "Hello, world!";

extern "C" {
    fn abs(input: i32) -> i32;
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    println!("name is: {}", HELLO_WORLD);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // let address = 0x01234usize;
    // let r = address as *mut i32;

    // let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}
