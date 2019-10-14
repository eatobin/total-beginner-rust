#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Cat {
    rank: u8,
}

impl Cat {
    fn get_rank(&self) -> u8 { self.rank }

//    fn set_rank(self, rank: u8) -> Self { Self { rank } }

    fn set_rank(&mut self, rank: u8) { self.rank = rank }
}

fn find_cat(target: u8, cats: Vec<Cat>) -> Option<Cat> {
    let mut iterator = cats.into_iter();
    let maybe_match = iterator.find(|c| c.get_rank() == target);
    maybe_match
}

fn find_cat_9(target: u8, cats: Vec<&Cat>) -> Option<&Cat> {
    let mut iterator = cats.into_iter();
    let maybe_match = iterator.find(|c| c.get_rank() == target);
    maybe_match
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn find_cat_2<F>(target: u8, cats: Vec<Cat>, f: F) -> Option<Cat> where
    F: Fn(&Cat) -> u8 {
    let mut iterator = cats.into_iter();
    let maybe_match = iterator.find(|c| f(c) == target);
    maybe_match
}

fn find_animal<T, F>(target: u8, coll: Vec<T>, f: F) -> Option<T> where
    F: Fn(&T) -> u8 {
    let mut iterator = coll.into_iter();
    let maybe_match = iterator.find(|a| f(a) == target);
    maybe_match
}

fn find_talker<T, F>(target: &str, coll: Vec<T>, f: F) -> Option<T> where
    F: Fn(&T) -> &str {
    let mut iterator = coll.into_iter();
    let maybe_match = iterator.find(|a| f(a) == target);
    maybe_match
}

fn find_talker_refs<'a, T, F>(target: &str, coll: Vec<&'a T>, f: F) -> Option<&'a T> where
    F: Fn(&T) -> &str {
    let mut iterator = coll.into_iter();
    let maybe_match = iterator.find(|a| f(a) == target);
    maybe_match
}

fn triple(x: i32) -> i32 { x * 3 }

#[derive(PartialEq, Debug, Clone)]
struct Dog {
    age: u8,
}

impl Dog {
    fn get_age(&self) -> u8 { self.age }
}

#[derive(PartialEq, Debug, Clone)]
struct Bird {
    name: String,
}

impl Bird {
    fn get_name(&self) -> &str { &self.name }
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
        assert_eq!(find_cat(33, cats), None)
    }

    #[test]
    fn test_find_cat_val_9() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![&cat44, &cat4, &cat16];
        assert_eq!(find_cat_9(44, cats.clone()), Some(&Cat { rank: 44 }));
        assert_eq!(find_cat_9(33, cats), None)
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
        assert_eq!(find_cat_2(44, cats.clone(), Cat::get_rank), Some(Cat { rank: 44 }));
        assert_eq!(find_cat_2(33, cats, Cat::get_rank), None)
    }

    #[test]
    fn test_find_cat_generic() {
        let cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        let cats = vec![cat44, cat4, cat16];
        assert_eq!(find_animal(44, cats.clone(), Cat::get_rank), Some(Cat { rank: 44 }));
        assert_eq!(find_animal(33, cats, Cat::get_rank), None)
    }

    #[test]
    fn test_find_dog_generic() {
        let dog99 = Dog { age: 99 };
        let dog9 = Dog { age: 9 };
        let dog16 = Dog { age: 16 };
        assert_eq!(Dog::get_age(&dog9), 9);
        let dogs = vec![dog99, dog9, dog16];
        assert_eq!(find_animal(99, dogs.clone(), Dog::get_age), Some(Dog { age: 99 }));
        assert_eq!(find_animal(33, dogs, Dog::get_age), None)
    }

    #[test]
    fn test_find_bird_generic() {
        let bird77 = Bird { name: "seventy-seven".to_owned() };
        let bird7 = Bird { name: "seven".to_owned() };
        let bird16 = Bird { name: "sixteen".to_owned() };
        assert_eq!(Bird::get_name(&bird77), "seventy-seven");
        let birds = vec![bird77, bird7, bird16];
        assert_eq!(find_talker("sixteen", birds.clone(), Bird::get_name), Some(Bird { name: "sixteen".to_owned() }));
        assert_eq!(find_talker("twelve", birds.clone(), Bird::get_name), None);
    }

    #[test]
    fn test_find_bird_generic_refs() {
        let bird77 = Bird { name: "seventy-seven".to_owned() };
        let bird7 = Bird { name: "seven".to_owned() };
        let bird16 = Bird { name: "sixteen".to_owned() };
        assert_eq!(Bird::get_name(&bird77), "seventy-seven");
        let birds = vec![&bird77, &bird7, &bird16];
        assert_eq!(find_talker_refs("sixteen", birds.clone(), Bird::get_name), Some(&Bird { name: "sixteen".to_owned() }));
        assert_eq!(find_talker_refs("twelve", birds.clone(), Bird::get_name), None);
    }

    #[test]
    fn test_cat_vec_change_refs() {
        let mut cat44 = Cat { rank: 44 };
        let cat4 = Cat { rank: 4 };
        let cat16 = Cat { rank: 16 };
        Cat::set_rank(&mut cat44, 99);
        let cats = vec![cat4, cat16, cat44];
    }
}
