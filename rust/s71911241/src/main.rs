// use std::str;
use std::thread;
use std::time::Duration;
use std::process::Command;
// use rayon::prelude::*;
use rayon::iter::{ParallelIterator, IntoParallelIterator};

fn main() {

    let arr = [1, 2, 3, 4, 5];
    (0..arr.len()).into_par_iter().for_each(|idx| {
        let output = Command::new("sh")
                .args(["-c", &format!("echo -n {}", arr[idx])])
                .output()
                .expect("failed to execute process");
        println!("Index: {}, Output: {:?}", idx, output.stdout);
    });

    let result: Vec<_> = (0..arr.len()).into_par_iter().flat_map(|idx| {
        let output = Command::new("sh")
                .args(["-c", &format!("echo -n {}", arr[idx])])
                .output()
                .expect("failed to execute process");
        println!("Index: {}, Output: {:?}", idx, output.stdout);
        output.stdout
    }).collect();
    println!("Result {:?}", result);

    println!();

    let arr = [1, 2, 3, 4, 5];
    arr.into_par_iter().for_each(|value| {
        let output = Command::new("sh")
                .args(["-c", &format!("echo -n {}", value)])
                .output()
                .expect("failed to execute process");
        println!("Index: {}, Output: {:?}", value, output.stdout);
    });

    let result: Vec<_> = arr.into_par_iter().flat_map(|value| {
        let output = Command::new("sh")
                .args(["-c", &format!("echo -n {}", value)])
                .output()
                .expect("failed to execute process");
        println!("Index: {}, Output: {:?}", value, output.stdout);
        output.stdout
    }).collect();
    println!("Result {:?}", result);

    // arr.into_par_iter().for_each(|idx| {
    //     let output = Command::new("sh")
    //             .args(["-c", "echo hello"])
    //             .output()
    //             .expect("failed to execute process");
    //     println!("Index: {}, Output: {:?}", idx, output.stdout);
    // });

    println!();

    let vec = vec![1, 2, 3, 4, 5];
    vec.into_par_iter().for_each(|idx| {
        let output = Command::new("sh")
                .args(["-c", "echo -n hello"])
                .output()
                .expect("failed to execute process");
        println!("Index: {}, Output: {:?}", idx, output.stdout);
    });

    println!();

    // let generated_key = Command::new("generate_user_key")
    //     .arg(file_array[i])
    //     .output()
    //     .expect("generate_user_key command failed to start");
    // println!("stdout: {}", String::from_utf8_lossy(&generated_key.stdout));

    let mut threads = vec![];
    for idx in 0..arr.len() {
        // println!("hi number {} from the spawned thread!", arr[idx]);
        threads.push(thread::spawn(move || -> Vec<_> {
            let output = Command::new("sh")
                    .args(["-c", &format!("echo -n {}", idx)])
                    .output()
                    .expect("failed to execute process");
            println!("Index: {}, Output: {:?}", idx, output.stdout);
            thread::sleep(Duration::from_millis(1));
            output.stdout
        }));
    }

    let result = threads.into_iter().flat_map(|c| c.join().unwrap()).collect::<Vec<_>>();

    println!("{:?}", result);

    println!("{:?}", arr);

    println!();

    let arr = ["one", "two", "three", "four", "five"];
    (0..arr.len()).into_par_iter().for_each(|idx| {
        let output = Command::new("sh")
                .args(["-c", &format!("echo -n {}", arr[idx])])
                .output()
                .expect("failed to execute process");
        println!("Index: {}, Output: {:?}", idx, String::from_utf8(output.stdout));
    });
}
