pub struct Trie {
    root: Node,
}

pub struct Node {
    ch: char,
    is_end: bool,
    children: Vec<Self>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node {
                ch: char::default(),
                is_end: false,
                children: vec![],
            },
        }
    }

    pub fn insert(&mut self, word: impl AsRef<str>) {
        let mut cur = &mut self.root;

        for ch in word.as_ref().chars() {
            let pos = cur.children.iter().position(|node| node.ch == ch);
            cur = if let Some(pos) = pos {
                &mut cur.children[pos]
            } else {
                cur.children.push(Node {
                    ch,
                    is_end: false,
                    children: vec![],
                });
                cur.children.last_mut().unwrap()
            };
        }

        cur.is_end = true;
    }

    fn starts_with_with_end(&self, word: impl AsRef<str>) -> Option<&Node> {
        let mut cur = &self.root;
        for ch in word.as_ref().chars() {
            cur = cur.children.iter().find(|node| node.ch == ch)?;
        }
        Some(cur)
    }

    pub fn search(&self, word: impl AsRef<str>) -> bool {
        self.starts_with_with_end(word)
            .is_some_and(|node| node.is_end)
    }

    pub fn starts_with(&self, word: impl AsRef<str>) -> bool {
        self.starts_with_with_end(word).is_some()
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[test]
fn tests() {
    let mut trie = Trie::new();
    trie.insert("apple");
    assert!(trie.search("apple"));
    assert!(!trie.search("app"));
    assert!(trie.starts_with("app"));
    trie.insert("app");
    assert!(trie.search("app"));
}
