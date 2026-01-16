pub fn print_vec_2d<T: std::fmt::Display>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for val in row {
            print!("{:} ", val);
        }
        println!();
    }
}