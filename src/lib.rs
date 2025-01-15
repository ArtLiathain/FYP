// use pyo3::prelude::*;
// use rand::Rng;
// use union_find::QuickFindUf;

// /// Python module implemented in Rust
// #[pymodule]
// fn simulation(_py: Python, m: &PyModule) -> PyResult<()> {
//     // Add functions to the module
//     #[pyfn(m, "random_number")]
//     fn random_number() -> PyResult<i32> {
//         let mut rng = rand::thread_rng();
//         Ok(rng.gen_range(1..=100)) // Return a random number
//     }

//     #[pyfn(m, "union_find_example")]
//     fn union_find_example() -> PyResult<String> {
//         let mut uf = QuickFindUf::new(10);
//         uf.union(1, 2);
//         Ok(format!("1 and 2 connected: {}", uf.connected(1, 2))) // Return a string
//     }

//     Ok(()) // Return Ok(()) from the #[pymodule] function
// }
