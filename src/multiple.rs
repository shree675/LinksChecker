use std::thread::{self, JoinHandle};

#[path = "./single.rs"]
mod single;

pub fn run(sites: Vec<String>, detailed: bool) {
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(sites.capacity());

    for site in sites {
        let handle = thread::spawn(move || {
            single::run(site, detailed);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
