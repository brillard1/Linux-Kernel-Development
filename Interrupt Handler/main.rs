#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();

    // trigger a page fault
    unsafe {
        *(0xce45a53 as *mut u64) = 42;
    };

    // as before
    #[cfg(test)]
    test_main();

    println!("Compilation Successful!");
    loop {}
}

