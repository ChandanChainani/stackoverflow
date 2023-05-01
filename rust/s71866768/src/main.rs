use rand::thread_rng;
use rand::Rng;
use std::time::Instant;

fn stencil(w: &mut [f64], v: &mut [f64], nx: usize, nz: usize) {
    // for i in 1..nx - 1 {
    //     for j in 1..nz - 1 {
    //         w[i * nz + j] = 0.25
    //             * (v[i * nz + j + 1]
    //                 + v[(i - 1) * nz + j]
    //                 + v[(i + 1) * nz + j]
    //                 + v[i * nz + j - 1]);
    //     }
    // }
    for (i, _) in w.iter().enumerate() {
        for (j, value) in w.iter().enumerate() {
            println!("{} {} {}", i, j, value);
        }
    }
}

fn display(arr: &mut [f64]) {
    println!("Some numbers: {:?}", arr[515]);
}

fn main() {
    let mut atime: u128 = 0;
    const NX: usize = 512;
    const NZ: usize = 512;
    const NP: u128 = 2;

    let mut rng = thread_rng();
    let distr = rand::distributions::Standard;
    let mut rho = [0f64; NX * NZ];
    let mut drho = [0f64; NX * NZ];
    println!("{:?}", rho);
    println!("{:?}", drho);

    // Fill rho with random
    for x in &mut rho {
        *x = rng.sample(distr);
    }

    display(&mut drho);

    // Exec stencil
    let start = Instant::now();
    for _ in 1..NP {
        stencil(&mut drho, &mut rho, NX, NZ);
    }
    atime += start.elapsed().as_micros();
    display(&mut drho);

    println!("Time : {} us", atime / NP)
}
