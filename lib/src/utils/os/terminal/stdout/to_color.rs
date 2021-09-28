use console::Color;

pub fn to_color(color: &str) -> Color {
    match color {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => Color::White,
    }
}

#[cfg(test)]
mod test {
    // use crate::to_color;
    #[tokio::test]
    #[ignore]
    async fn test_to_color() {
        unimplemented!();
    }
}
