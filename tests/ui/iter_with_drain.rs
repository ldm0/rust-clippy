#![warn(clippy::iter_with_drain)]
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn full() {
    let mut a = vec!["aaa".to_string(), "bbb".to_string()];
    let mut a: BinaryHeap<_> = a.drain(..).collect();
    let mut a: HashSet<_> = a.drain().collect();
    let mut a: VecDeque<_> = a.drain().collect();
    let mut a: Vec<_> = a.drain(..).collect();
    let mut a: HashMap<_, _> = a.drain(..).map(|x| (x.clone(), x)).collect();
    let _: Vec<(String, String)> = a.drain().collect();
}

fn closed() {
    let mut a = vec!["aaa".to_string(), "bbb".to_string()];
    let mut a: BinaryHeap<_> = a.drain(0..).collect();
    let mut a: HashSet<_> = a.drain().collect();
    let mut a: VecDeque<_> = a.drain().collect();
    let mut a: Vec<_> = a.drain(..a.len()).collect();
    let mut a: HashMap<_, _> = a.drain(0..a.len()).map(|x| (x.clone(), x)).collect();
    let _: Vec<(String, String)> = a.drain().collect();
}

fn should_not_help() {
    let mut a = vec!["aaa".to_string(), "bbb".to_string()];
    let mut a: BinaryHeap<_> = a.drain(1..).collect();
    let mut a: HashSet<_> = a.drain().collect();
    let mut a: VecDeque<_> = a.drain().collect();
    let mut a: Vec<_> = a.drain(..a.len() - 1).collect();
    let mut a: HashMap<_, _> = a.drain(1..a.len() - 1).map(|x| (x.clone(), x)).collect();
    let _: Vec<(String, String)> = a.drain().collect();

    let mut b = vec!["aaa".to_string(), "bbb".to_string()];
    let _: Vec<_> = b.drain(0..a.len()).collect();
}

fn main() {
    full();
    closed();
    should_not_help();
}
