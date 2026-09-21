use std::thread ;
use std::sync::atomic::{
            AtomicUsize,
            Ordering,        
        } ;

static CNT: AtomicUsize = AtomicUsize::new(0) ;


fn main() {
    let handls = 
            (0..4)
            .map(|_| {
                thread::spawn(move || {
                    for i in 0..1000 {
                        CNT.fetch_add(i, Ordering::Release) ;
                    }
                })
            }) 
            .collect::<Vec<_>>()
            ;
    for h in handls {
        h.join().unwrap() ;
    }

    println!("{}", CNT.load(Ordering::Acquire)) ;   // Out: 1998000
}
