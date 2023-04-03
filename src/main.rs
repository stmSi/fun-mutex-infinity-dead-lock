use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let resource1 = Arc::new(Mutex::new(()));
    let resource2 = Arc::new(Mutex::new(()));
    
    let resource1_clone = Arc::clone(&resource1);
    let resource2_clone = Arc::clone(&resource2);

    let thread1 = thread::spawn(move || {
        let _thread1_resource1 = resource1_clone.lock().unwrap();
        thread::sleep(Duration::from_secs(3));
        let _thread2_resource2 = resource2_clone.lock().unwrap();
    });
    
    let resource1_clone = Arc::clone(&resource1);
    let resource2_clone = Arc::clone(&resource2);

    let thread2 = thread::spawn(move || {
        let _thread1_resource2 = resource2_clone.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let _thread1_resource1 = resource1_clone.lock().unwrap();
        // Do some work
    });
    
    thread1.join().unwrap();
    thread2.join().unwrap();
}
