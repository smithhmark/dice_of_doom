#[derive(Debug)]
pub struct Hex {
    pub owner: usize,
    pub dice: usize,
}

pub fn as_string(h: &Hex) -> String {
    format!("{}-{}", h.owner, h.dice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_as_string() {
        let owner: usize = 0;
        let dice: usize = 3;
        let hex = Hex {owner, dice};
        let rep = as_string(&hex);
        let exp = "0-3".to_string();
        assert_eq!(rep, exp);
    }
}
