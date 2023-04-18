mod get_timestamp;
//use std::time::Duration;
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..6{
            println!("Spawned thread:{}",i);
            get_timestamp::get_ntp();
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..6{
        println!("Main thread: {}", i);
        get_timestamp::get_ntp();
        thread::sleep(Duration::from_millis(1));
    } 
    handle.join().unwrap();
}
