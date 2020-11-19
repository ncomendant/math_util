pub fn gcf(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcf(b, a % b)
    }
}

pub fn lcm(a: u32, b: u32) -> u32 {
    (a / gcf(a, b)) * b
}
