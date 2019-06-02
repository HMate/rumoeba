
#[derive(Debug, PartialEq)]
pub struct Player {
    name: String
}

impl Player {
    pub fn new(name: &str) -> Player {
        return Player { name: name.to_string() };
    }

    pub fn name(&self) -> &String {
        return &self.name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_player_name() {
        let p = Player::new("Kose");
        assert_eq!("Kose", p.name());
    }
}