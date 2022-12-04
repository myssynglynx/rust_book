fn main() {
    {
        // raw pointers we know are valid
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // uncertain raw pointers
        let address = 0x012345usize;
        let r = address as *const i32;
    }

    {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
}
