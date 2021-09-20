pub enum Size {
    Small,
    Medium,
    Big,
}

enum Unit {
    cm(i32),
}

pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Black,
    White,
    custom(u32, u32, u32),
}
