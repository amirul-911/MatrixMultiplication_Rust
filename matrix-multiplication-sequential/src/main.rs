use rand::Rng;
use std::time::Instant;

const N:usize = 1000;

fn main() 
{
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
    let result = matrixmultiply(N, &a, &a);
    //println!("{:.2?} * {:.2?}", a,a);
    //println!("=");
    //println!("{:.2?}", result);
    let duration = start.elapsed();

    println!("\nTime elapsed for sequential square matrix multiplication with size {} is: {:?}\n", N, duration);
}

fn matrixmultiply(n:usize, a1: &[Vec<f64>], a2: &[Vec<f64>]) -> Vec<Vec<f64>>
{
    let mut result = vec![vec![0_f64; n]; n];
    for i in 0..n
    {
        for j in 0..n
        {
            for k in 0..n
            {
                result[i][j] += a1[i][k] * a2[k][j];
            }
        }
    }
    return result;
}

