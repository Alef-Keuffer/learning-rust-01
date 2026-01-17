pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn outer_fn() {
    fn inner_fn() {
        println!("I'm inner_fun");
    }

    inner_fn();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fail_01() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
            panic!("second thread panicked");
        }
    }

    #[test]
    fn fail_02() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
                panic!("second thread panicked");
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn fail_03() {
        panic!("this test fails");
    }
}
