use crate::time_execution;

#[macro_export]
macro_rules! define_solver {
    (
        $struct_name:ident,
        $year:expr,
        $day:expr,
        $input_type:ty,
        $preprocess_fn:expr,
        $solve_one_fn:expr,
        $solve_two_fn:expr
    ) => {
        pub struct $struct_name;

        impl crate::utils::solver::Solver<$input_type> for $struct_name {
            fn preprocess<'a>(input: &'a str) -> $input_type {
                $preprocess_fn(input)
            }
            fn solve_one(input: &$input_type) -> String {
                $solve_one_fn(input)
            }
            fn solve_two(input: &$input_type) -> String {
                $solve_two_fn(input)
            }
        }

        impl crate::utils::solver::SolverDyn for $struct_name {
            fn year(&self) -> &str { $year }
            fn day(&self) -> &str { $day }
            fn solve(&self, input: &str) -> (String, String) {
                crate::utils::solver::Solver::solve(self, input)
            }
        }
    };
}

pub trait Solver<T>: SolverDyn {
    fn solve_one(input: &T) -> String;
    fn solve_two(input: &T) -> String;
    fn preprocess(input: &str) -> T;
    fn solve<'a>(&self, input:  &'a str) -> (String, String) {
        println!("\n- - - - - {} day {} - - - - -", self.year(), self.day());
        let data = time_execution!(format!("Preprocessing"), Self::preprocess(input));
        let one = time_execution!(format!("Part 1"), Self::solve_one(&data));
        println!("Solution: {}", one);
        let two = time_execution!(format!("Part 2"), Self::solve_two(&data));
        println!("Solution: {}", two);
        println!("- - - - - - - - - - - - - - - -");
        (one, two)
    }
}

pub trait SolverDyn {
    fn year(&self) -> &str;
    fn day(&self) -> &str;
    fn solve(&self, input: &str) -> (String, String);
}

