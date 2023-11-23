#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc, Mutex};
    use std::sync::mpsc::Sender;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn create_thread_with_spawn() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("{} : spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("{} : main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn wait_for_threads_to_finish_with_join() {
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!("{} : spawned thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("{} : main thread", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    #[test]
    fn take_ownership_with_move() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        // assert_eq!(v, vec![1, 2, 3]); cant borrow moved value

        handle.join().unwrap();
    }

    #[test]
    fn use_channel_to_transfer_data_between_threads() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();

        assert_eq!(received, "hi");
    }

    #[test]
    fn receive_multiple_values_like_an_iterator() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];

            for val in vals {
                tx.send(val).unwrap();
            }
        });

        let mut expected = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")].into_iter();
        for received in rx {
            assert_eq!(received, expected.next().unwrap());
        }
    }

    #[test]
    fn create_multiple_transmitter_with_clone() {
        let (tx, rx) = mpsc::channel();

        send(tx.clone(), vec![String::from("1"), String::from("1"), String::from("1"), String::from("1")]);
        send(tx, vec![String::from("1"), String::from("1"), String::from("1"), String::from("1")]);

        let mut expected = vec![
            String::from("1"), String::from("1"), String::from("1"), String::from("1"),
            String::from("1"), String::from("1"), String::from("1"), String::from("1"),
        ].into_iter();

        for received in rx {
            assert_eq!(received, expected.next().unwrap());
        }
    }

    fn send(tx: Sender<String>, vals: Vec<String>) {
        thread::spawn(move || {
            for val in vals {
                tx.send(val).unwrap();
            }
        });
    }

    #[test]
    fn create_mutex_with_new() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 10;
        }

        assert_eq!(*m.lock().unwrap(), 10);
    }

    #[test]
    fn share_mutex_with_arc_smart_pointer() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || *counter.lock().unwrap() += 1);
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }
}