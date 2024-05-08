use crate::tests;

pub fn longest_common_prefix<S: AsRef<str>, V: AsRef<[S]>>(strs: V) -> String {
    let mut words = Words::default();
    for s in strs.as_ref() {
        words.add(s.as_ref());
    }
    words.longest_prefix()
}

struct Words {
    has_empty: bool,
    root: Node,
}

struct Node {
    ch: char,
    is_end: bool,
    children: Vec<Self>,
}

impl Default for Words {
    fn default() -> Self {
        Self {
            has_empty: false,
            root: Node {
                ch: char::default(),
                is_end: false,
                children: vec![],
            },
        }
    }
}

impl Words {
    pub fn add(&mut self, word: &str) {
        if word.is_empty() {
            self.has_empty = true;
        }

        if self.has_empty {
            return;
        }

        let mut cur = &mut self.root;

        for ch in word.chars() {
            let child = cur.children.iter().position(|node| node.ch == ch);
            cur = if let Some(pos) = child {
                &mut cur.children[pos]
            } else {
                cur.children.push(Node {
                    ch,
                    is_end: false,
                    children: vec![],
                });
                cur.children.last_mut().unwrap()
            }
        }

        cur.is_end = true;
    }

    pub fn longest_prefix(&self) -> String {
        let mut acc = String::new();
        if self.has_empty {
            return acc;
        }
        let mut cur = &self.root;
        while cur.children.len() == 1 && !cur.is_end {
            cur = &cur.children[0];
            acc.push(cur.ch);
        }
        acc
    }
}

tests! {
    longest_common_prefix,
    ["flower", "flow", "flight"] => "fl",
    ["dog", "racecar", "car"] => "",
    ["", "b"] => "",
    ["ab", "a"] => "a",
}
