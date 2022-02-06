enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn avatar<'life>(movement: Movement) -> &'life str {
    match movement {
        Movement::Up => {
            return "Avatar Go Up";
        }
        Movement::Down => {
            return "Avatar Go Down";
        }
        Movement::Left => {
            return "Avatar Go Left";
        }
        Movement::Right => {
            return "Avatar Go Right";
        }
    }
}
pub fn run() {
    println!("{}", avatar(Movement::Down));
    println!("{}", avatar(Movement::Up));
    println!("{}", avatar(Movement::Left));
    println!("{}", avatar(Movement::Right));
}
