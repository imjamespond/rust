mod tests {

    #[test]
    fn test_mpmc() {
        use std::sync::mpsc::channel;
        use std::sync::{Arc, Mutex};
        use std::{thread, time};

        const M: usize = 8;
        const N: usize = 4;

        let (tx, rx) = channel();
        let shared_rx = Arc::new(Mutex::new(rx));
        let mut handlers = Vec::new();

        // multi producers
        for producer in 0..M {
            let tx = tx.clone();
            let handler = thread::spawn(move || {
                for job in 0..100 {
                    tx.send((job, producer)).unwrap();
                }
            });
            handlers.push(handler);
        }

        // multi consumers
        for worker in 0..N {
            let shared_rx = shared_rx.clone();
            let mut done = 0;
            let handler = thread::spawn(move || loop {
                let mut tuple = None;
                {
                    let rx = shared_rx.lock().unwrap();
                    let rs = rx.recv();
                    if let Ok(tup) = rs {
                        tuple = Some(tup);
                    }
                }
                match tuple {
                    Some((job, producer)) => {
                        let millis = time::Duration::from_millis(job * 3);
                        thread::sleep(millis);
                        if job == 99 {
                            print!("==============================");
                        }
                        done = done + 1;
                        println!("job:{}, producer:{}, worker:{}-{}", job, producer, worker, done);
                    }
                    None => todo!(),
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }

    #[test]
    fn test_arc() {
        use std::sync::mpsc::channel;
        use std::sync::{Arc, Mutex};
        use std::thread;

        const N: usize = 10;

        // Spawn a few threads to increment a shared variable (non-atomically), and
        // let the main thread know once all increments are done.
        //
        // Here we're using an Arc to share memory among threads, and the data inside
        // the Arc is protected with a mutex.
        let data = Arc::new(Mutex::new(0));

        let (tx, rx) = channel();
        for _ in 0..N {
            let (data, tx) = (Arc::clone(&data), tx.clone());
            thread::spawn(move || {
                // The shared state can only be accessed once the lock is held.
                // Our non-atomic increment is safe because we're the only thread
                // which can access the shared state when the lock is held.
                //
                // We unwrap() the return value to assert that we are not expecting
                // threads to ever fail while holding the lock.
                let mut data = data.lock().unwrap();
                *data += 1;
                if *data == N {
                    tx.send(()).unwrap();
                }
                // the lock is unlocked here when `data` goes out of scope.
            });
        }

        rx.recv().unwrap();
    }
}
