use std::{thread, time::Duration, sync::{Arc, Mutex}};

fn main() {
    let s: String = String::from("¡¡¡¡¡Hello");

    let val = Arc::new(Mutex::new(s.clone()));

    let arc1 = val.clone();
    let t1 = thread::spawn(move || { 
        for _ in 0..5 {
            arc1.lock().unwrap().push('!');
            thread::sleep(Duration::from_secs(1));
        };
    });

    let arc2 = val.clone();
    let t2 = thread::spawn(move || { 
        for _ in 0..5 {
            arc2.lock().unwrap().remove(0);
            thread::sleep(Duration::from_secs(1));
        };
    });

    for _ in 0..5 {
        println!("{}", val.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }

    t1.join().expect("ohno"); t2.join().expect("ohno2");
    println!("{}", val.lock().unwrap());
}
