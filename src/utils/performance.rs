#[macro_export]
macro_rules! time_execution {
    ($label: expr, $expression:expr) => {{
        let start = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("System time before Unix epoch");
        let result = $expression;
        let end = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("System time before Unix epoch");
        let total_time = end - start;
        println!("{} executed in {:?}", $label, total_time);
        result
    }};
}