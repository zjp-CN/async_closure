//! Codes in `../examples/capture_lifetimes_mut.rs` and `examples/capture_no_lifetimes_mut.rs`
//! are almost the same, except for an additianl lifetime parameter when capturing lifetimes.
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

mod owned {
    use async_closure::{async_owned_closure_mut, capture_no_lifetimes::AsyncFnMut};
    use async_lock::{Mutex, MutexGuard};

    struct S {
        mutex: Mutex<Vec<u8>>,
    }

    impl S {
        async fn take_and_return_a_mut_closure<T, F>(
            &self,
            mut f: F,
            buffer: &mut Vec<u8>,
        ) -> (T, F)
        where
            F: for<'any> AsyncFnMut<(&'any mut Vec<u8>, MutexGuard<'any, Vec<u8>>), Output = T>,
        {
            let output = {
                let args = (&mut *buffer, self.mutex.lock().await);
                f.call_mut(args).await
            };
            (output, f)
        }
    }

    async fn run(s: &S, buffer: &mut Vec<u8>) {
        let outer = vec![0u8];

        let cb = async_owned_closure_mut!({
            buf: Vec<u8> = outer
        }; async |buffer: &mut Vec<u8>, lock: MutexGuard<'_, Vec<u8>>| -> () {
            std::mem::swap(buffer, buf);
            let mut lock = lock;
            lock.extend_from_slice(buf);
            buf.extend_from_slice(&lock);
            println!("buffer= {buffer:?}\nbuf   = {buf:?}\nlock  = {:?}\n", &*lock);
        });

        // callback can be passed back
        let (_, cb) = s.take_and_return_a_mut_closure(cb, buffer).await;
        // and its states are accessible
        assert_eq!(cb.buf, &[1, 2, 1]);
        let (_, mut cb) = s.take_and_return_a_mut_closure(cb, buffer).await;
        assert_eq!(cb.buf, &[0, 2, 1, 0]);

        // and invoke call_* methods again!
        {
            let args = (&mut *buffer, s.mutex.lock().await);
            cb.call_mut(args).await;
        }
        {
            use async_closure::capture_no_lifetimes::AsyncFnOnce;
            let args = (buffer, s.mutex.lock().await);
            cb.call_once(args).await; // AsyncFnMut is a subtrait of AsyncFnOnce
        }
    }

    pub async fn test() {
        let mut buffer = vec![1];
        let s = S {
            mutex: Mutex::new(vec![2]),
        };
        run(&s, &mut buffer).await;
        assert_eq!(buffer, &[1, 2, 1, 2, 1, 0, 1, 2, 1]);
        assert_eq!(&**s.mutex.lock().await, &[2, 1, 0, 1, 2, 1, 0, 2, 1, 0]);
    }
}

#[pollster::main]
async fn main() {
    owned::test().await;
}
