use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

pub fn threads() {
    let handle1 = thread::spawn(|| {
        println!("thread1 running");
    });

    let handle2 = thread::spawn(|| {
        println!("running thread2");
    });

    let handle3 = thread::spawn(|| {
        println!("thread3 running");
    });

    for handle in vec![handle1, handle2, handle3] {
        handle.join().unwrap();
    }
}

pub fn sequare_nums() {
    let nums = vec![2, 3, 8, 9, 100];
    let (rx, tx) = mpsc::channel();
    let h1 = thread::spawn(move || {
        for num in nums {
            rx.send(num).unwrap();
        }
    });

    /*
    ---- concur::tests::test_sequare_nums stdout ----
    calculating num sequare
    received: 4
    received: 9
    received: 64
    received: 81
    received: 10000
        */
    let h2 = thread::spawn(|| {
        for num in sq(tx) {
            println!("received: {}", num);
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}

fn sq(nums: Receiver<i32>) -> Receiver<i32> {
    println!("calculating num sequare");
    let (rx, tx) = mpsc::channel();
    let _ = thread::spawn(move || {
        for num in nums {
            rx.send(num * num).unwrap();
        }
    });

    tx
}

pub fn concurrent_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..100 {
        let cnt_clone = counter.clone();
        let h = thread::spawn(move || {
            let mut cnt = cnt_clone.lock().unwrap();
            *cnt += 1;
        });

        handlers.push(h);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    let value = *counter.lock().unwrap();
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel() {
        threads();
    }

    #[test]
    fn test_sequare_nums() {
        sequare_nums();
    }

    #[test]
    fn test_concurrent_counter() {
        let count = concurrent_counter();
        assert_eq!(count, 100);
    }
}
