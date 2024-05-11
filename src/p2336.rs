use std::collections::BTreeSet;

#[derive(Default)]
pub struct SmallestInfiniteSet {
    popped: BTreeSet<i32>,
}

impl SmallestInfiniteSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pop_smallest(&mut self) -> i32 {
        // _adds_ to popped
        // TODO: use a better solution
        let smallest = (1..).find(|n| !self.popped.contains(n)).unwrap();
        self.popped.insert(smallest);
        smallest
    }

    pub fn add_back(&mut self, num: i32) {
        // _removes_ from popped
        self.popped.remove(&num);
    }
}

#[cfg(test)]
#[test]
fn tests() {
    let mut set = SmallestInfiniteSet::new();
    set.add_back(2);
    assert_eq!(set.pop_smallest(), 1);
    assert_eq!(set.pop_smallest(), 2);
    assert_eq!(set.pop_smallest(), 3);
    set.add_back(1);
    assert_eq!(set.pop_smallest(), 1);
    assert_eq!(set.pop_smallest(), 4);
    assert_eq!(set.pop_smallest(), 5);
}
