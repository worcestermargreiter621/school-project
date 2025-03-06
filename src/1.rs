struct Circle {
    center: Point,
    radius: u32,
}

impl Circle {
    fn area(&self) -> u32 {
        self.radius * self.radius * std::f64::consts::PI
    }

    fn perimeter(&self) -> u32 {
        2 * self.radius + 2 * self.radius * std::f64::consts::PI
    }
}
