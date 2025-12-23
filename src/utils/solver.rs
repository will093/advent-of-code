pub trait Solver {
    fn year(&self) -> &str;
    fn day(&self) -> &str;
    fn label(&self) -> &str;
    fn solve(&self, input: &str) -> String;
}
