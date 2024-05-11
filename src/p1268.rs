use crate::tests;

pub fn suggested_products<V: AsRef<[S]>, S: AsRef<str>, W: AsRef<str>>(
    products: V,
    search_word: W,
) -> Vec<Vec<String>> {
    let products = products.as_ref();
    let mut trie = products.iter().map(AsRef::as_ref).fold(
        Trie::new(),
        |mut trie, product| {
            trie.insert(product);
            trie
        },
    );
    trie.sort();

    let search_word = search_word.as_ref();
    let mut acc = vec![];

    for i in 1..search_word.chars().count() + 1 {
        let mut prefix_set = trie.get_with_prefix(&search_word[0..i]);
        prefix_set.sort();
        if prefix_set.len() > 3 {
            prefix_set.drain(3..);
        }
        acc.push(prefix_set);
    }

    acc
}

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

    pub fn sort(&mut self) {
        let mut queue = vec![&mut self.root];

        while let Some(node) = queue.pop() {
            node.children.sort();
            queue.extend(node.children.iter_mut());
        }
    }

    /// Call [`Self::sort`] before this function to get ordered results.
    pub fn get_with_prefix(&self, prefix: impl AsRef<str>) -> Vec<String> {
        let mut cur = &self.root;
        let mut acc = vec![];
        let prefix = prefix.as_ref();
        let mut common = String::new();

        for ch in prefix.chars() {
            let Some(node) = cur.children.iter().find(|node| node.ch == ch)
            else {
                return vec![];
            };
            cur = node;
            common.push(cur.ch);
        }

        // end of prefix, add everything upto max

        let mut queue = vec![(cur, common)];
        while let Some((node, s)) = queue.pop() {
            if node.is_end {
                acc.push(s.clone());
            }
            node.children.iter().for_each(|node| {
                queue.push((node, {
                    let mut tmp = s.clone();
                    tmp.push(node.ch);
                    tmp
                }))
            });
        }
        acc
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.ch.eq(&other.ch)
    }
}
impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ch.cmp(&other.ch))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ch.cmp(&other.ch)
    }
}

tests! {
    suggested_products,
    ["mobile", "mouse", "moneypot", "monitor", "mousepad"], "mouse" => [
        &["mobile", "moneypot", "monitor"][..],
        &["mobile", "moneypot", "monitor"][..],
        &["mouse", "mousepad"][..],
        &["mouse", "mousepad"][..],
        &["mouse", "mousepad"][..],
    ],
}
