/// All of this is now in another module now
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Q {
    rdr_cvar: Condvar,
    wrtr_cvar: Condvar,
    mutex: Mutex<i32>,
}

pub struct Worker {
    pub w_1: String,
    pub w_2: i16,
    pub w_3: i16,
}

impl Worker {
    pub fn calculate_w(&self) -> i16 {
        self.w_2 * self.w_3
    }
    fn reader(id: i32, qq: Arc<Q>) {
        let q = &*qq;
        for i in 0..10 {
            let guard = q.mutex.lock().unwrap();
            let mut guard = q.rdr_cvar.wait_while(guard, |n| *n <= 0).unwrap();
    
            println!("{} Reader {} n_resource = {}\n", i, id, *guard);
            *guard -= 1;
    
            if *guard > 0 {
                q.rdr_cvar.notify_one();
            }
            if *guard < 10 {
                q.wrtr_cvar.notify_one();
            }
        }
    }
}
