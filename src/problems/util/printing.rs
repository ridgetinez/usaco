pub fn print_grid<T: std::fmt::Debug>(grid: &Vec<Vec<T>>) {
    grid.iter().for_each(|row| println!("{:?}", row));
}
