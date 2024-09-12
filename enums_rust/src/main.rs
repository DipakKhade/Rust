fn main() {
    show_direction(Direction::South)
}

enum Direction {
    East,
    West,
    North,
    South,
}

fn show_direction(dir: Direction) {
    println!(" The direction is {}", dir)
}
