#![feature(const_generics)]
use std::collections::BTreeSet;
use rand::prelude::*;

struct TimeSeries {
    timestamp: u64,
}

impl TimeSeries {
    fn next(&mut self) -> u64 {
        let inc:u8 = random();
        self.timestamp += inc as u64;
        return self.timestamp;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct HeapEntry {
    ts: u64,
    index: usize,
}

pub fn run_heap(num_ts: usize, len_per_ts: usize) {
    let mut vec = Vec::new();
    let mut set = BTreeSet::new();
    for i in 0..num_ts {
        vec.push(TimeSeries{timestamp:0});
        set.insert(HeapEntry{ts: 0, index: i});
    }
    let len = num_ts * len_per_ts;
    for _ in 0..len {
        let entry = *set.iter().next().unwrap(); // copy
        set.remove(&entry);

        // println!("{}", entry.ts);

        let new_entry = HeapEntry{ts: vec[entry.index].next(), index: entry.index};
        set.insert(new_entry);
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct VecEntry {
    ts: u64,
}

pub fn run_vec(num_ts: usize, len_per_ts: usize) {
    let mut vec = Vec::new();
    let mut set = Vec::new();
    for _ in 0..num_ts {
        vec.push(TimeSeries{timestamp:0});
        set.push(Some(VecEntry{ts: 0}));
    }
    let len = num_ts * len_per_ts;
    for _ in 0..len {
        let mut argmin = 0;
        let mut ts = 9999_9999_9999_9999;
        for i in 0..num_ts {
            if let Some(entry) = &set[i] {
                if entry.ts < ts {
                    ts = entry.ts;
                    argmin = i;
                }
            }
        }
        // println!("{}", ts);
        set[argmin] = Some(VecEntry{ts:vec[argmin].next()});
    }
}
