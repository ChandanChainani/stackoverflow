// extern crate futures; // [dependencies] futures = "0.1"

// use std::io::BufRead;
// use std::thread;

// use futures::sync::mpsc::{Sender, channel};
// use futures::{Future, Stream, Sink};

// fn main() {
//     let mut worker = Some(spawn_worker());
//
//     let stdin = ::std::io::stdin();
//     for line in stdin.lock().lines() {
//         let line = line.unwrap();
//         if line == "stop" {
//             drop(worker.take());
//             continue;
//         };
//
//         if let Some(w) = worker {
//             worker = Some(w.send(Msg::Echo(line)).wait().unwrap())
//         } else {
//             println!("The worker has been stopped!");
//         }
//     }
//
//     println!("Bye!");
// }
//
// enum Msg {
//     Echo(String),
// }
//
// fn spawn_worker() -> Sender<Msg> {
//     let (tx, rx) = channel(1);
//     thread::spawn(move || {
//         rx.for_each(|msg| {
//             match msg {
//                 Msg::Echo(msg) => println!("{} ❤️ ", msg),
//             }
//             Ok(())
//         }).map(|()| {
//             println!("The worker has stopped!");
//         }).wait().unwrap();
//     });
//     tx
// }

// fn main() {
//     use std::sync::mpsc::sync_channel;
//     use std::thread;
//
//     let (sender, receiver) = sync_channel(1);
//
//     // this returns immediately
//     sender.send(1).unwrap();
//
//     thread::spawn(move|| {
//         // this will block until the previous message has been received
//         sender.send(2).unwrap();
//     });
//
//     let r1 = receiver.recv().unwrap();
//     println!("{:?}", r1);
//
//     let r2 = receiver.recv().unwrap();
//     println!("{:?}", r2);
//
//     assert_eq!(r1, 1);
//     assert_eq!(r2, 2);
// }
// https://matklad.github.io/2018/03/03/stopping-a-rust-worker.html


// use rand::Rng;
// use std::thread;
// use std::io::BufRead;
// use std::error::Error;
// use std::sync::Arc;
// use std::sync::mpsc::{channel, Sender};
// use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
// use std::time::Duration;

// v1
// fn main() {
//     let (tx, rx) = sync_channel(1);
//     // let arc_tx = Arc::new(rx);
//     spawn_worker(rx);
//
//     let stdin = ::std::io::stdin();
//     for line in stdin.lock().lines() {
//         let line = line.unwrap();
//         if line == "stop" || line == "quit" {
//             drop(&tx);
//             break;
//         };
//         // worker = worker.send(msg).wait().unwrap();
//         let _ = &tx.send(line).unwrap();
//     }
//
//     println!("Bye!");
// }
//
// fn spawn_worker(rx: Receiver<String>) {
//     thread::spawn(move || {
//         // let _ = rx.for_each(|msg| {
//         //     match msg {
//         //         Msg::Echo(msg) => {
//         //             println!("{} ❤️", msg);
//         //             Ok(())
//         //         },
//         //         Msg::Stop => Err(()),
//         //     }
//         // }).then(|result| {
//         //     println!("The worker has stopped!");
//         //     result
//         // }).wait();
//
//         for msg in rx {
//             println!("{:?} ❤️ ", msg);
//         }
//         println!("The worker has stopped!");
//     });
// }

// fn main() {
//     let (tx, rx) = sync_channel(1);
//     // let arc_tx = Arc::new(rx);
//     spawn_worker(rx);
//
//     let lines: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
//     for line in lines {
//         // if line == "stop" || line == "quit" {
//         //     drop(&tx);
//         //     break;
//         // };
//         // worker = worker.send(msg).wait().unwrap();
//         let _ = &tx.send(line.to_string()).unwrap();
//     }
//
//     println!("Bye!");
// }
//
// fn spawn_worker(rx: Receiver<String>) {
//     thread::spawn(move || {
//         let mut rng = rand::thread_rng();
//         // let _ = rx.for_each(|msg| {
//         //     match msg {
//         //         Msg::Echo(msg) => {
//         //             println!("{} ❤️", msg);
//         //             Ok(())
//         //         },
//         //         Msg::Stop => Err(()),
//         //     }
//         // }).then(|result| {
//         //     println!("The worker has stopped!");
//         //     result
//         // }).wait();
//
//         for msg in rx {
//             println!("{:?} ❤️ ", msg);
//             let sleep = rng.gen_range(0..10);
//             println!("Sleep {:?} ❤️ ", sleep);
//             thread::sleep(Duration::from_secs(sleep + 1));
//         }
//         println!("The worker has stopped!");
//     });
// }
// v2
// use rand::Rng;
// use std::thread;
// use std::time::Duration;
// use std::sync::{Arc, Mutex};
// use crossbeam_channel::bounded;
// use crossbeam_utils::thread as cthread;
//
// fn main() {
//     let workers = 15;
//     let mut rng = rand::thread_rng();
//     let (sx, rx) = bounded(workers);
//     let (sf, rf) = bounded(1);
//     let mut search_str = String::from("one");
//     let mut idx: u32 = 0;
//     let mut found  = false;
//
//     cthread::scope(|s| {
//         s.spawn(move |_| {
//             // println!("Worker: {:?}, Sleep: {:?}", idx, _sleep);
//             found = rf.recv().unwrap();
//             println!("IN FOUND {:?}", found);
//         });
//
//         while !found && search_str.len() > 0 {
//             // println!("SEARCH {:?}", search_str);
//             // if search_str.len() == 0 || found {
//             //     break;
//             // }
//
//             println!("IDX: {:?}, FOUND: {:?}", idx, found);
//
//             sx.send(search_str.clone()).unwrap();
//             let r1 = rx.clone();
//             let _sleep = rng.gen_range(0..30);
//
//             s.spawn(move |_| {
//                 // println!("Worker: {:?}, Sleep: {:?}", idx, _sleep);
//                 thread::sleep(Duration::from_secs(_sleep + 1));
//                 println!("Worker: {:?}, Sleeped: {:?}, Searched : {:?}", idx, _sleep, r1.recv().unwrap());
//                 // temporary found the string under 45 thread/worker
//                 if idx == 10 {
//                     found = true;
//                     println!("FOUND {:?} {:?}", found, idx);
//                     sf.send(true).unwrap();
//                 }
//             });
//
//             if idx > 0 && idx % 15 == 0 {
//                 // println!("BEFORE {:?}", search_str);
//                 search_str.pop();
//                 // println!("AFTER {:?}", search_str);
//                 // let mut tmp = search_str.chars();
//                 // tmp.next_back();
//                 // search_str = tmp.as_str().to_string();
//             }
//
//             idx += 1;
//         }
//
//         // println!("Done");
//     }).unwrap();
//
//     println!("Bye!");
// }

// v3
// use rand::Rng;
// use rand::rngs::StdRng;
// use rand::SeedableRng;
//
// use std::thread;
// use std::time::Duration;
// use threadpool::ThreadPool;
// use std::sync::mpsc::channel;
// use std::sync::Arc;
//
// fn main() {
//     let mut search_str = String::from("one");
//     let workers = 4;
//     let pool = ThreadPool::new(workers);
//     let mut found = Arc::new(false);
//     let mut idx = 0;
//
//     let (tx, rx) = channel();
//     // let p = pool.clone();
//     thread::spawn(move || {
//
//         found.unwrap() = rx.recv().unwrap();
//         // println!("FOUND: {:?}", found);
//         // drop(p);
//     });
//     println!("FOUND: {:?}", found);
//
//     while !found.unwrap() && search_str.len() > 0 {
//         idx += 1;
//         let tx = tx.clone();
//         // println!("IDX: {:?}, SEARCH {:?}", idx, search_str);
//         let s = search_str.clone();
//         pool.execute(move|| {
//             let mut rng: StdRng = SeedableRng::from_entropy();
//             let _sleep = rng.gen_range(0..30);
//             thread::sleep(Duration::from_secs(_sleep + 1));
//             if idx == 15 {
//                 println!("FOUND: {:?} {:?}", idx, s);
//                 tx.send(true).expect("channel will be there waiting for the pool");
//             }
//             // println!("Worker {:?} Done", idx);
//             println!("Worker: {:?}, Sleeped: {:?}, Searched : {:?}", idx, _sleep, s);
//         });
//
//         if idx % 12 == 0 {
//             search_str.pop();
//         }
//     }
//
//     pool.join();
//     println!("FOUND: {:?}", found);
// }

// fn main() {
//     let workers = 10;
//     let mut idx: u32 = 1;
//     let mut found: u8 = 0;
//     let mut word = String::from("one");
//     let (tx, rx) = sync_channel(workers);
//     let (sTx, sRx) = sync_channel(1);
//
//     thread::spawn(move || {
//         found = sRx.recv().unwrap();
//         println!("{:?}", found);
//         println!("Found");
//     });
//
//     loop {
//         if word.len() == 0 || found > 0 {
//             drop(&tx);
//             drop(&sTx);
//             break;
//         }
//         idx += 1;
//         spawn_worker(word.to_owned(), rx, sTx.clone());
//         // let w = word.clone();
//         // thread::spawn(move || {
//         //     println!("Word {:?} ❤️ ", w);
//         //     println!("The worker has stopped! {:?}", rx.recv().unwrap());
//         // });
//         let _ = &tx.send(idx).unwrap();
//     }
//
//     println!("Bye!");
// }
//
// fn spawn_worker(word: String, rx: Receiver<u32>, sx: SyncSender<u8>) {
//     thread::spawn(move || {
//         println!("Word {:?} ❤️ ", word);
//         println!("The worker has stopped! {:?}", rx.recv().unwrap());
//     });
//     println!("{:?}", "Done");
// }















// extern crate futures; // [dependencies] futures = "0.1"

// use std::io::BufRead;
// use std::thread;

// use futures::sync::mpsc::{Sender, channel};
// use futures::{Future, Stream, Sink};

// fn main() {
//     let mut worker = Some(spawn_worker());
//
//     let stdin = ::std::io::stdin();
//     for line in stdin.lock().lines() {
//         let line = line.unwrap();
//         if line == "stop" {
//             drop(worker.take());
//             continue;
//         };
//
//         if let Some(w) = worker {
//             worker = Some(w.send(Msg::Echo(line)).wait().unwrap())
//         } else {
//             println!("The worker has been stopped!");
//         }
//     }
//
//     println!("Bye!");
// }
//
// enum Msg {
//     Echo(String),
// }
//
// fn spawn_worker() -> Sender<Msg> {
//     let (tx, rx) = channel(1);
//     thread::spawn(move || {
//         rx.for_each(|msg| {
//             match msg {
//                 Msg::Echo(msg) => println!("{} ❤️ ", msg),
//             }
//             Ok(())
//         }).map(|()| {
//             println!("The worker has stopped!");
//         }).wait().unwrap();
//     });
//     tx
// }

// fn main() {
//     use std::sync::mpsc::sync_channel;
//     use std::thread;
//
//     let (sender, receiver) = sync_channel(1);
//
//     // this returns immediately
//     sender.send(1).unwrap();
//
//     thread::spawn(move|| {
//         // this will block until the previous message has been received
//         sender.send(2).unwrap();
//     });
//
//     let r1 = receiver.recv().unwrap();
//     println!("{:?}", r1);
//
//     let r2 = receiver.recv().unwrap();
//     println!("{:?}", r2);
//
//     assert_eq!(r1, 1);
//     assert_eq!(r2, 2);
// }


// use rand::Rng;
// use std::thread;
// use std::io::BufRead;
// use std::error::Error;
// use std::sync::Arc;
// use std::sync::mpsc::{channel, Sender};
// use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
// use std::time::Duration;

// v1
// fn main() {
//     let (tx, rx) = sync_channel(1);
//     // let arc_tx = Arc::new(rx);
//     spawn_worker(rx);
//
//     let stdin = ::std::io::stdin();
//     for line in stdin.lock().lines() {
//         let line = line.unwrap();
//         if line == "stop" || line == "quit" {
//             drop(&tx);
//             break;
//         };
//         // worker = worker.send(msg).wait().unwrap();
//         let _ = &tx.send(line).unwrap();
//     }
//
//     println!("Bye!");
// }
//
// fn spawn_worker(rx: Receiver<String>) {
//     thread::spawn(move || {
//         // let _ = rx.for_each(|msg| {
//         //     match msg {
//         //         Msg::Echo(msg) => {
//         //             println!("{} ❤️", msg);
//         //             Ok(())
//         //         },
//         //         Msg::Stop => Err(()),
//         //     }
//         // }).then(|result| {
//         //     println!("The worker has stopped!");
//         //     result
//         // }).wait();
//
//         for msg in rx {
//             println!("{:?} ❤️ ", msg);
//         }
//         println!("The worker has stopped!");
//     });
// }

// fn main() {
//     let (tx, rx) = sync_channel(1);
//     // let arc_tx = Arc::new(rx);
//     spawn_worker(rx);
//
//     let lines: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
//     for line in lines {
//         // if line == "stop" || line == "quit" {
//         //     drop(&tx);
//         //     break;
//         // };
//         // worker = worker.send(msg).wait().unwrap();
//         let _ = &tx.send(line.to_string()).unwrap();
//     }
//
//     println!("Bye!");
// }
//
// fn spawn_worker(rx: Receiver<String>) {
//     thread::spawn(move || {
//         let mut rng = rand::thread_rng();
//         // let _ = rx.for_each(|msg| {
//         //     match msg {
//         //         Msg::Echo(msg) => {
//         //             println!("{} ❤️", msg);
//         //             Ok(())
//         //         },
//         //         Msg::Stop => Err(()),
//         //     }
//         // }).then(|result| {
//         //     println!("The worker has stopped!");
//         //     result
//         // }).wait();
//
//         for msg in rx {
//             println!("{:?} ❤️ ", msg);
//             let sleep = rng.gen_range(0..10);
//             println!("Sleep {:?} ❤️ ", sleep);
//             thread::sleep(Duration::from_secs(sleep + 1));
//         }
//         println!("The worker has stopped!");
//     });
// }
// v2
// use rand::Rng;
// use std::thread;
// use std::time::Duration;
// use crossbeam_channel::bounded;
// use crossbeam_utils::thread as cthread;
//
// fn main() {
//     let workers = 30;
//     let mut rng = rand::thread_rng();
//     let (sx, rx) = bounded(workers);
//     let mut search_str = String::from("one");
//     let mut idx: u32 = 0;
//     let mut found = false;
//
//     cthread::scope(|s| {
//         while !found {
//             // println!("SEARCH {:?}", search_str);
//             if search_str.len() == 0 || found {
//                 break;
//             }
//
//             // println!("IDX: {:?}", idx);
//
//             idx += 1;
//
//             if idx == 45 {
//                 println!("FOUND {:?}", idx);
//                 found = true;
//             }
//
//             sx.send(search_str.clone()).unwrap();
//             let r1 = rx.clone();
//             let _sleep = rng.gen_range(0..30);
//
//             s.spawn(move |_| {
//                 // println!("Worker: {:?}, Sleep: {:?}", idx, _sleep);
//                 thread::sleep(Duration::from_secs(_sleep + 1));
//                 println!("Worker: {:?}, Sleeped: {:?}, Searched : {:?}", idx, _sleep, r1.recv().unwrap());
//             });
//
//             if idx % 15 == 0 {
//                 // println!("BEFORE {:?}", search_str);
//                 search_str.pop();
//                 // println!("AFTER {:?}", search_str);
//                 // let mut tmp = search_str.chars();
//                 // tmp.next_back();
//                 // search_str = tmp.as_str().to_string();
//             }
//         }
//
//         // println!("Done");
//     }).unwrap();
//
//     println!("Bye!");
// }

// use rand::Rng;
// use rand::rngs::StdRng;
// use rand::SeedableRng;
//
// use std::thread;
// use std::time::Duration;
// use threadpool::ThreadPool;
// use std::sync::mpsc::channel;
//
// fn main() {
//     let mut search_str = String::from("one");
//     let workers = 4;
//     let pool = ThreadPool::new(workers);
//     let mut found = false;
//     let mut idx = 0;
//
//     let (tx, rx) = channel();
//     // thread::spawn(move || {
//     //     found = rx.recv().unwrap();
//     //     println!("FOUND: {:?}", found);
//     // });
//
//     while !found && search_str.len() > 0 {
//         idx += 1;
//         let tx = tx.clone();
//         println!("IDX: {:?}, SEARCH {:?}", idx, search_str);
//         pool.execute(move|| {
//             let mut rng: StdRng = SeedableRng::from_entropy();
//             let _sleep = rng.gen_range(0..30);
//             thread::sleep(Duration::from_secs(_sleep + 1));
//             if idx == 15 {
//                 tx.send(true).expect("channel will be there waiting for the pool");
//             }
//         });
//
//         if idx % 12 == 0 {
//             search_str.pop();
//             found = rx.recv().unwrap();
//             println!("FOUND: {:?}", found);
//         }
//     }
// }

// fn main() {
//     let workers = 10;
//     let mut idx: u32 = 1;
//     let mut found: u8 = 0;
//     let mut word = String::from("one");
//     let (tx, rx) = sync_channel(workers);
//     let (sTx, sRx) = sync_channel(1);
//
//     thread::spawn(move || {
//         found = sRx.recv().unwrap();
//         println!("{:?}", found);
//         println!("Found");
//     });
//
//     loop {
//         if word.len() == 0 || found > 0 {
//             drop(&tx);
//             drop(&sTx);
//             break;
//         }
//         idx += 1;
//         spawn_worker(word.to_owned(), rx, sTx.clone());
//         // let w = word.clone();
//         // thread::spawn(move || {
//         //     println!("Word {:?} ❤️ ", w);
//         //     println!("The worker has stopped! {:?}", rx.recv().unwrap());
//         // });
//         let _ = &tx.send(idx).unwrap();
//     }
//
//     println!("Bye!");
// }
//
// fn spawn_worker(word: String, rx: Receiver<u32>, sx: SyncSender<u8>) {
//     thread::spawn(move || {
//         println!("Word {:?} ❤️ ", word);
//         println!("The worker has stopped! {:?}", rx.recv().unwrap());
//     });
//     println!("{:?}", "Done");
// }

// v4
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crossbeam_channel::bounded;
use crossbeam_utils::thread as cthread;

fn main() {
    let workers = 10;
    let mut rng = rand::thread_rng();
    let (sx, rx) = bounded(workers);
    let mut search_str = String::from("one");
    let found = Arc::new(Mutex::new(false));

    println!("FOUND {:?}", *found.lock().unwrap());

    cthread::scope(|s| {
        let mut idx: u32 = 0;

        while !*found.lock().unwrap() && search_str.len() > 0 {
            // temporary found the string under 45 thread/worker
            sx.send(search_str.clone()).unwrap();
            let r1 = rx.clone();
            let _sleep = rng.gen_range(0..30);

            s.spawn({
                let f = Arc::clone(&found);
                move |_| {
                    thread::sleep(Duration::from_secs(_sleep + 1));
                    if idx == 10 {
                        println!("FOUND {:?}", idx);
                        let mut tmp = f.lock().unwrap();
                        *tmp = true;
                    }

                    println!("Worker: {:?}, Sleeped: {:?}, Searched : {:?}", idx, _sleep, r1.recv().unwrap());
                }
            });

            if idx > 0 && idx % 15 == 0 {
                search_str.pop();
            }

            idx += 1;
        }
    }).unwrap();

    println!("FOUND {:?}", *found.lock().unwrap());

    println!("Bye!");
}
