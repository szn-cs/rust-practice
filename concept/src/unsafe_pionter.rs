use std::ptr::NonNull;

pub fn non_null() {
    let mut x: i64 = 943857203;
    let p1 = &mut x;
    let p2: *mut _ = p1;
    let p3 = std::ptr::from_mut(p1);
    let p4 = p3;
    let p7 = std::ptr::addr_of_mut!(*p1);
    let p9 = p1 as *mut _;

    unsafe {
        p3.write(3);
        *p4 = 20;
    }

    println!("{p1}");
    println!("{x}");
    unsafe {
        println!("{}", *p7);
    }

    let p5 = NonNull::new(p2);

    {
        let mut v = [1, 2, 3, 4, 5];
        let p = v.as_mut_ptr();
    }
}
