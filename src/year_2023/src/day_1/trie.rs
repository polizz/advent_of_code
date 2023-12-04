use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, Clone)]
pub(crate) struct TrieNode<T> {
    pub value: Option<T>,
    pub children: Vec<Option<Box<TrieNode<T>>>>,
}

impl<T> Default for TrieNode<T>
where
    T: Clone,
{
    fn default() -> Self {
        TrieNode {
            value: None,
            children: vec![None; 26],
        }
    }
}

impl<T> TrieNode<T>
where
    T: Clone,
{
    fn add_str_recurse(&mut self, st: &[char], k: usize, value: T) {
        if k == st.len() {
            self.value = Some(value);

            return;
        }

        let c = st[k];
        let char_index = c as usize - 61 - 36;

        // println!("char:{}, char_index:{}, k:{}", c, char_index, k);

        let child = &mut self.children[char_index];
        if let Some(ref mut child_box) = child {
            let child_bor: &mut TrieNode<T> = child_box.borrow_mut();
            child_bor.add_str_recurse(st, k + 1, value);
        } else {
            let mut new_trie_node = TrieNode::default();
            new_trie_node.add_str_recurse(st, k + 1, value);
            let new_child = Box::new(new_trie_node);
            child.replace(new_child);
        }
    }

    fn has_string_r(&self, st: &[char], k: usize) -> bool {
        if k >= st.len() {
            return true;
        }

        let c = st[k];
        let char_index = c as usize - 61 - 36;

        let child = &self.children[char_index];
        if let Some(ref child_box) = child {
            let child_bor: &TrieNode<T> = child_box.borrow();
            return child_bor.has_string_r(st, k + 1);
        }

        false
    }

    fn get_value_r(&self, st: &[char], k: usize) -> Option<T> {
        if k == st.len() {
            return self.value.clone();
        }

        let c = st[k];
        let char_index = c as usize - 61 - 36;

        let child = &self.children[char_index];
        if let Some(ref child_box) = child {
            let child_bor: &TrieNode<T> = child_box.borrow();
            return child_bor.get_value_r(st, k + 1);
        }

        None
    }
}

#[derive(Debug)]
pub(crate) struct Trie<T> {
    pub(crate) root: TrieNode<T>,
}

impl<T> Trie<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn has_string(&self, st: &str) -> bool {
        let st: Vec<char> = st.chars().collect();
        let st_chars = st.as_slice();

        self.root.has_string_r(st_chars, 0)
    }

    pub fn get_value(&self, st: &str) -> Option<T> {
        let st: Vec<char> = st.chars().collect();
        let st_chars = st.as_slice();

        self.root.get_value_r(st_chars, 0)
    }

    pub fn add_string(&mut self, st: &str, value: T) {
        let st: Vec<char> = st.chars().collect();
        let st_chars = st.as_slice();

        self.root.add_str_recurse(st_chars, 0, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_smoke() {
        let mut t: Trie<usize> = Trie::new();
        let st = "Andrew".to_lowercase();
        let st2 = "Ashley".to_lowercase();

        t.add_string(&st, 32);
        t.add_string(&st2, 1);
        let search_val = t.get_value(&st);
        let search_val_not_found = t.get_value(&st2);

        // println!("has_string: {}", t.has_string(&"Ash".to_lowercase()));

        assert_eq!(32, search_val.unwrap());
        assert_eq!(1, search_val_not_found.unwrap());

        assert_eq!(true, t.has_string(&"Ash".to_lowercase()));
        assert_eq!(false, t.has_string(&"Ashleyyo".to_lowercase()));
        assert_eq!(false, t.has_string(&"b"));
        assert_eq!(true, t.has_string(&"An".to_lowercase()));
        assert_eq!(false, t.has_string(&"Andra".to_lowercase()));

        // println!("t is: {:#?}, search_val:{:?}", &t, search_val);
    }
}
