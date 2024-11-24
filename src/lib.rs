use pyo3::prelude::*;

#[pymodule]
fn rustmath(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(gcd, m)?)?;
    m.add_function(wrap_pyfunction!(lcm, m)?)?;
    Ok(())
}

#[pyfunction]
fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[pyfunction]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n-1 {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}

#[pyfunction]
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[pyfunction]
fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 && b == 0 {
        0
    } else {
        (a * b).abs() / gcd(a, b)
    }
}
