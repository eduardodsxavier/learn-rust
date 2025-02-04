use std::thread;

fn main() {

    let mut count = 0;

    let thread_join_handle1 = thread::spawn(move || {
        for n in 1..500 {
            println!("1: {count}");
            count += n;
        }
    });

    let thread_join_handle2 = thread::spawn(move || {
        for n in 1..500 {
            println!("2: {count}");
            count += n;
        }
    });

    let _ = thread_join_handle1.join();

    let _ = thread_join_handle2.join();
}
