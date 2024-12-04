use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

pub fn vec(nums: &[i32]) -> Vec<i32> {
    let mut num_list = Vec::new();
    for num in nums {
        num_list.push(*num);
    }

    num_list
}

pub fn hash_map(keys: &[&str], values: &[&str]) -> HashMap<String, String> {
    let mut idx = 0 as usize;
    let mut key_val: HashMap<String, String> = HashMap::new();
    loop {
        if idx >= keys.len() {
            break;
        }

        key_val.insert(keys[idx].to_string(), values[idx].to_string());
        idx += 1;
    }

    key_val
}

pub fn hash_set(elements: Vec<String>, key: &str) -> (HashSet<String>, bool) {
    let mut set: HashSet<String> = HashSet::new();
    for e in elements {
        set.insert(e);
    }

    let contain = set.contains(key);
    (set, contain)
}

pub fn btree_set(elements: Vec<String>) -> BTreeSet<String> {
    let mut bs = BTreeSet::new();
    for e in elements {
        bs.insert(e);
    }

    bs
}

pub fn vec_dequeue(elements: Vec<String>) -> VecDeque<String> {
    let mut queue = VecDeque::new();
    for e in elements {
        queue.push_front(e);
    }

    queue
}

pub fn btree_map(elements: Vec<(String, String)>) -> BTreeMap<String, String> {
    let mut btree_map = BTreeMap::new();
    for (key, val) in elements {
        btree_map.insert(key, val);
    }

    btree_map
}
