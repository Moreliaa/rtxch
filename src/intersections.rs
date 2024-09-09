pub struct Intersections {
    count: usize,
    xs: Vec<f64>,
}

impl Intersections {
    pub fn new(xs: Vec<f64>) -> Intersections {
        let count = xs.len();
        Intersections { xs, count }
    }

    pub fn xs(&self) -> &Vec<f64> {
        &self.xs
    }
    
    pub fn count(&self) -> usize {
        self.count
    }
}