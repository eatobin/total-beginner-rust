pub struct Cat {
    rank: u8,
}

impl Cat {
    pub fn get_rank(&self) -> u8 { self.rank }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cat() {
        let cat1 = Cat { rank: 62 };
        assert_eq!(cat1.get_rank(), 62)
    }
}
