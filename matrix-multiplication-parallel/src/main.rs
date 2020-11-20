use rand::Rng;
use std::time::Instant;
use rayon::prelude::*;

const N: usize = 1000;
const T: usize = 1;

fn main() 
{
    rayon::ThreadPoolBuilder::new().num_threads(T).build_global().unwrap();
    
    let mut a = vec![vec![0_f64; N]; N];
    for i in 0..N
    {
        for j in 0..N
        {
            let mut rng = rand::thread_rng();
            a[i][j] = rng.gen::<f64>()
        }
    }

    let start = Instant::now();
    let result = matrixmultiplyrayon(N, &a, &a);
    //println!("{:.2?} * {:.2?}", a,a);
    //println!("\t\t=");
    //println!("{:.2?}", result);
    let duration = start.elapsed();

    println!("\nTime elapsed for parallel square matrix multiplication with size {} using {} thread is: {:?}\n", N, T, duration);
}

fn matrixmultiplyrayon(n: usize, a1: &[Vec<f64>], a2: &[Vec<f64>]) -> Vec<Vec<f64>>
{
    let mut rows = (0..n).into_par_iter().map(move |i|{
        let a1_row = &a1[i];

        (i, rowofsum(a1_row, a2, n))
    }).collect::<Vec<(usize, Vec<f64>)>>();

    rows.par_sort_by(|left, right| left.0.cmp(&right.0));
    rows.into_iter().map(|(_, row)| row).collect()
}

fn rowofsum(a1_row: &[f64], a2: &[Vec<f64>], n: usize) -> Vec<f64>
{
    (0..n).map(|j| (0..n).map(|k| a1_row[k] * a2[k][j]).sum()).collect::<Vec<f64>>()
}
