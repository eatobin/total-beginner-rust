#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Cat {
    rank: u8,
}

impl Cat {
    fn get_rank(&self) -> u8 { self.rank }
}

fn find_cat(target: u8, cats: Vec<Cat>) -> Option<Cat> {
    let mut iterator = cats.into_iter();
    let maybe_match = iterator.find(|c| c.get_rank() == target);
    maybe_match
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn find_cat2<F>(target: u8, cats: Vec<Cat>, f: F) -> Option<Cat> where
    F: Fn(&Cat) -> u8 {
    let mut iterator = cats.into_iter();
    let maybe_match = iterator.find(|c| f(c) == target);
    maybe_match
}

fn triple(x: i32) -> i32 { x * 3 }

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

    #[test]
    fn test_cat_max_min_val_ref() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![cat44, cat4, cat16];
        let mut iterator = cats.iter();
        let oldest = iterator.max_by_key(|c| c.rank);
        assert_eq!(oldest, Some(&Cat { rank: 44 }));
        let mut iterator2 = cats.into_iter();
        let youngest = iterator2.min_by_key(|c| c.rank);
        assert_eq!(youngest, Some(Cat { rank: 4 }))
    }

    #[test]
    fn test_find_cat_val() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![cat44, cat4, cat16];
        assert_eq!(find_cat(44, cats.clone()), Some(Cat { rank: 44 }));
        assert_eq!(find_cat(33, cats), None);
    }

    #[test]
    fn test_double_triple() {
        let double = |x| 2 * x;
        assert_eq!(apply_to_3(double), 6);
        assert_eq!(apply_to_3(triple), 9)
    }

    #[test]
    fn test_find_cat_closure() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![cat44, cat4, cat16];
        assert_eq!(find_cat2(44, cats.clone(), Cat::get_rank), Some(Cat { rank: 44 }));
        assert_eq!(find_cat2(33, cats, Cat::get_rank), None);
    }
}
