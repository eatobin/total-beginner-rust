#[derive(PartialEq, PartialOrd, Debug)]
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

    #[test]
    fn test_cat_eq() {
        let cat1 = Cat { rank: 4 };
        let cat2 = Cat { rank: 4 };
        assert_eq!(cat1 == cat2, true)
    }

    #[test]
    fn test_cat_gt() {
        let cat1 = Cat { rank: 44 };
        let cat2 = Cat { rank: 4 };
        assert_eq!(cat1 > cat2, true)
    }

    #[test]
    fn test_cat_vec_owned() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![cat44, cat4, cat16];
        let mut iterator = cats.iter();
        assert_eq!(iterator.next(), Some(&Cat { rank: 44 }));
        assert_eq!(iterator.len(), 2);
        assert_eq!(iterator.next(), Some(&Cat { rank: 4 }));
        assert_eq!(iterator.nth(0), Some(&Cat { rank: 16 }));
        assert_eq!(iterator.nth(0), None);
        assert_eq!(iterator.len(), 0)
    }

    #[test]
    fn test_cat_vec_refs() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![&cat44, &cat4, &cat16];
        let mut iterator = cats.iter();
        assert_eq!(iterator.next(), Some(&&Cat { rank: 44 }));
        assert_eq!(iterator.len(), 2);
        assert_eq!(iterator.next(), Some(&&Cat { rank: 4 }));
        assert_eq!(iterator.nth(0), Some(&&Cat { rank: 16 }));
        assert_eq!(iterator.nth(0), None);
        assert_eq!(iterator.len(), 0)
    }

    #[test]
    fn test_cat_vec_owned_into() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![cat44, cat4, cat16];
        let mut iterator = cats.into_iter();
        assert_eq!(iterator.next(), Some(Cat { rank: 44 }));
        assert_eq!(iterator.len(), 2);
        assert_eq!(iterator.next(), Some(Cat { rank: 4 }));
        assert_eq!(iterator.nth(0), Some(Cat { rank: 16 }));
        assert_eq!(iterator.nth(0), None);
        assert_eq!(iterator.len(), 0)
    }

    #[test]
    fn test_cat_vec_refs_into() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![&cat44, &cat4, &cat16];
        let mut iterator = cats.into_iter();
        assert_eq!(iterator.next(), Some(&Cat { rank: 44 }));
        assert_eq!(iterator.len(), 2);
        assert_eq!(iterator.next(), Some(&Cat { rank: 4 }));
        assert_eq!(iterator.nth(0), Some(&Cat { rank: 16 }));
        assert_eq!(iterator.nth(0), None);
        assert_eq!(iterator.len(), 0)
    }
}
