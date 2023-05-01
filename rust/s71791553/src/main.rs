use chrono::prelude::*;
// use indexmap::indexmap;
// use indexmap::IndexMap;
use std::collections::{HashMap, BTreeMap};
// use std::collections::hash_map::Entry;


#[derive(Clone, Debug)]
struct MapVec<K, V> {
    v: Vec<V>,
    h: HashMap<K, u8>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> MapVec<K, V> {
    fn new() -> Self {
        Self{
            v: Vec::new(),
            h: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.v.len()
    }

    fn get(&self) -> &Vec<V> {
        &self.v
    }

    fn insert(&mut self, pos: K, value: V) {
        if self.h.get(&pos).is_none() {
            self.v.push(value);
            self.h.insert(pos, 0);
        }
    }

    fn contains_key(&self, key: &K) -> bool {
        self.h.contains_key(key)
    }
}

fn main() {
    let pair1 = Pair { pair_id: "abc1", p1: "x1", p2: "x2" };
    let pair2 = Pair { pair_id: "abc2", p1: "x1", p2: "x3" };
    let pair3 = Pair { pair_id: "abc3", p1: "x4", p2: "x2" };
    let pair4 = Pair { pair_id: "abc4", p1: "x2", p2: "x1" };
    let pair5 = Pair { pair_id: "abc5", p1: "x4", p2: "x1" };
    let vec = vec![pair1, pair2, pair3, pair4, pair5];

    fast_tree(&vec);

    let start = Utc::now();
    let input = process(&vec);
    let mut output = vec![vec![]];
    let current_path = vec![];
    // println!("{:#?}", input);
    traverse("x1", "x1", &input, current_path, &mut output);
    let difference = Utc::now() - start;
    //// println!("{:#?}", output);

    println!("Time:\t {:?}", difference);

    // v2;
    // bfs(&vec, &vec[2], 2);
    // v3;
    //// for (pos, pair) in vec.iter().enumerate() {
    ////     let mut pos_h: HashMap<usize, u8> = HashMap::new();
    ////     let mut node_h: HashMap<&'_ str, u8> = HashMap::new();
    ////     pos_h.entry(pos).or_insert(1);
    ////     node_h.entry(pair.p1).or_insert(1);
    ////     println!("============================================================");
    ////     println!("Outer: {:?}", pair);
    ////     //println!("Outer: {} {:?}", pos, pair);
    ////     //println!("Pos: {:?}", pos_h);
    ////     //println!("Node: {:?}", node_h);
    ////     search_edges(&vec, pair.p1, pair.p2, &mut pos_h, &mut node_h);
    //// }


    let start = Utc::now();
    // println!("{:#?}", &vec);
    let ds_h = create_ds(&vec);
    // println!("DS: {:#?}", ds_h);
    let mut output = vec![vec![]];
    // traverse_ds("x1", ds_h, &mut output);
    // let mut nodes: HashMap<&'_ str, usize> = HashMap::new();
    let mut current_path: BTreeMap<usize, u8> = BTreeMap::new();
    traverse_ds("x1", "x1", &vec, &ds_h, &mut current_path, &mut output);
    // let mut current_path: MapVec<usize, &'_ Pair<'_>> = MapVec::new();
    // println!("{:?}", current_path);

    // let mut current_path: BTreeMap<usize, u8> = BTreeMap::new();
    // traverse_ds("x1", "x1", &vec, &ds_h, &mut current_path, &mut output);
    let difference = Utc::now() - start;

    println!("Time:\t {:?}", difference);
    //// println!("{:#?}", output);
}

fn fast_tree<'a>(pairs: &'a Vec<Pair<'_>>) {
    let mut pos_h: HashMap<&str, u8> = HashMap::new();
    let mut elems: Vec<&str> = vec![];
    for (pos, pair) in pairs.iter().enumerate() {
        println!("{} {:?}", pos, pair);
        elems.push(pair.p1);
        elems.push(pair.p2);
        pos_h.entry(pair.p1).or_default();
        pos_h.entry(pair.p2).or_default();
    }
    println!("{:?} {:?}", pos_h, elems);
}

fn search_edges<'a>(pairs: &'a Vec<Pair<'_>>, root: &str, current: &str, pos_h: &mut HashMap<usize, u8>, node_h: &mut HashMap<&'a str, u8>) {
    println!("FIRST Root={}:Current={}", root, current);
    println!("FIRST Counter={:?}|IS_NONE={:?}|IS_ONE={:?}", node_h.get(current) == Option::None, node_h.get(current), node_h.contains_key(current) && node_h.get(current).unwrap() == &1);
    println!("FIRST Pos: {:?}", pos_h);
    println!("FIRST Node: {:?}", node_h);
    if node_h.get(current) == Option::None || node_h.get(current).unwrap() == &1 {
        // println!("{:?}", node_h);
        for (pos, pair) in pairs.iter().enumerate() {
            println!("LOOP: {}", pos);
            if !pos_h.contains_key(&pos) && current == pair.p1 {
                println!("MATCH: {}", current);
                //println!("Before Pos: {:?}", pos_h);
                //println!("Before Node: {:?}", node_h);
                //println!("Inner: {} {:?}", pos, pair);
                pos_h.entry(pos).or_default();
                node_h.entry(pair.p1).and_modify(|e| *e += 1).or_insert(0);
                //println!("Pos: {:?}", pos_h);
                //println!("Node: {:?}", node_h);
                // println!("Root: {:?}", root);
                //println!("=== Second Round {} ===", pair.pair_id);
                if root != pair.p2 {
                    println!("Inner: {:?}", pair);
                    println!("Recursive Root={}:Current={}", root, current);
                    search_edges(pairs, root, pair.p2, pos_h, node_h);
                    pos_h.remove(&pos);
                    *node_h.entry(pair.p1).or_insert(0) -= 1;
                    // let mut new_pos_h: HashMap<usize, u8> = pos_h.clone();
                    // let mut new_node_h: HashMap<&'_ str, u8> = node_h.clone();
                    // search_edges(pairs, root, pair.p2, &mut new_pos_h, &mut new_node_h)
                }
                // if root == pair.p2 {
                //     node_h.entry(pair.p2).or_default();
                //     break
                // }
                // else {
                //     for (pos2, pair2) in pairs.iter().enumerate() {
                //         if !pos_h.contains_key(&pos2) && pair.p2 == pair2.p1 {
                //             println!("=== Second Round {} ===", pair2.pair_id);
                //             let mut new_pos_h: HashMap<usize, u8> = pos_h.clone();
                //             new_pos_h.entry(pos2).or_default();
                //             let mut new_node_h: HashMap<&'_ str, u8> = node_h.clone();
                //             new_node_h.entry(pair2.p1).or_default();
                //             println!("Second Inner: {} {:?}", pos2, pair2);
                //             println!("Second Pos: {:?}", new_pos_h);
                //             println!("Second Node: {:?}", new_node_h);
                //             search_edges(pairs, root, pair2.p2, &mut new_pos_h, &mut new_node_h);
                //         }
                //     }
                // }
            }
        }
    }
    println!("End Node: {:?}", node_h);
}

// fn search_edges<'a>(pairs: &'a Vec<Pair<'_>>, root: &str, current: &str, pos_h: &mut HashMap<usize, u8>, node_h: &mut HashMap<&'a str, u8>, output: &mut Vec<Vec<&'a Pair<'a>>>) {
//     println!("FIRST Root={}:Current={}", root, current);
//     println!("FIRST Counter={:?}|IS_NONE={:?}|IS_ONE={:?}", node_h.get(current) == Option::None, node_h.get(current), node_h.contains_key(current) && node_h.get(current).unwrap() == &1);
//     println!("FIRST Pos: {:?}", pos_h);
//     println!("FIRST Node: {:?}", node_h);
//     if (node_h.get(current) == Option::None || node_h.get(current).unwrap() == &1) {
//         // println!("{:?}", node_h);
//         for (pos, pair) in pairs.iter().enumerate() {
//             println!("LOOP: {}", pos);
//             if !pos_h.contains_key(&pos) && current == pair.p1 {
//                 println!("MATCH: {}", current);
//                 let _len = output.len();
//                 println!("LEN: {}", _len);
//                 // println!("OUTPUT: {:?}", output);
//                 output[_len - 1].push(pair);
//                 //println!("Before Pos: {:?}", pos_h);
//                 //println!("Before Node: {:?}", node_h);
//                 //println!("Inner: {} {:?}", pos, pair);
//                 pos_h.entry(pos).or_default();
//                 *node_h.entry(pair.p1).or_insert(0) += 1;
//                 //println!("Pos: {:?}", pos_h);
//                 //println!("Node: {:?}", node_h);
//                 // println!("Root: {:?}", root);
//                 //println!("=== Second Round {} ===", pair.pair_id);
//                 if root != pair.p2 {
//                     println!("Inner: {:?}", pair);
//                     println!("Recursive Root={}:Current={}", root, current);
//                     output.append(&mut output.clone());
//                     search_edges(pairs, root, pair.p2, pos_h, node_h, output);
//                     pos_h.remove(&pos);
//                     *node_h.entry(pair.p1).or_insert(0) -= 1;
//                     // let mut new_pos_h: HashMap<usize, u8> = pos_h.clone();
//                     // let mut new_node_h: HashMap<&'_ str, u8> = node_h.clone();
//                     // search_edges(pairs, root, pair.p2, &mut new_pos_h, &mut new_node_h)
//                 }
//
//                 let _len = output.len();
//                 let last_edge = &output[_len - 1];
//                 // println!("EDGE LEN: {:#?}", last_edge.len());
//                 // println!("EDGE: {:#?}", last_edge);
//                 // println!("EDGE START: {}", last_edge[0].p1);
//                 // println!("EDGE END: {}", last_edge[last_edge.len() - 1].p2);
//                 if last_edge[0].p1 != last_edge[last_edge.len() - 1].p2 {
//                      output.remove(_len - 1);
//                 }
//
//                 // if _len == 1 || output[_len - 1][0].p1 != output[_len - 1][output[_len - 1].len() - 1].p2 {
//                 //     output.remove(output.len() - 1);
//                 // }
//                 // if root == pair.p2 {
//                 //     node_h.entry(pair.p2).or_default();
//                 //     break
//                 // }
//                 // else {
//                 //     for (pos2, pair2) in pairs.iter().enumerate() {
//                 //         if !pos_h.contains_key(&pos2) && pair.p2 == pair2.p1 {
//                 //             println!("=== Second Round {} ===", pair2.pair_id);
//                 //             let mut new_pos_h: HashMap<usize, u8> = pos_h.clone();
//                 //             new_pos_h.entry(pos2).or_default();
//                 //             let mut new_node_h: HashMap<&'_ str, u8> = node_h.clone();
//                 //             new_node_h.entry(pair2.p1).or_default();
//                 //             println!("Second Inner: {} {:?}", pos2, pair2);
//                 //             println!("Second Pos: {:?}", new_pos_h);
//                 //             println!("Second Node: {:?}", new_node_h);
//                 //             search_edges(pairs, root, pair2.p2, &mut new_pos_h, &mut new_node_h);
//                 //         }
//                 //     }
//                 // }
//             }
//         }
//     }
//     println!("End Node: {:?}", node_h);
// }

#[derive(Debug, PartialEq)]
struct Pair<'a> {
    pair_id: &'a str,
    p1: &'a str,
    p2: &'a str,
}

// fn bfs<'a>(pairs: &'a Vec<Pair<'_>>, root: &'a Pair<'a>, index: usize) {
//     println!();
//     println!("Root: {:?} {}", root, index);
//     for (pos, pair) in pairs.iter().enumerate() {
//         if pos == index {
//             continue
//         }
//         println!("{:?} {:?}", pos, pair);
//         if root.p2 == pair.p1 {
//             if root.p1 == pair.p2 {
//                 println!("break {:?}", pair);
//                 break
//             } else {
//                 bfs(pairs, root, pair, pos);
//                 println!("new {:?}", pair);
//             }
//         }
//         // bfs(pairs, pair, pos);
//         // res.entry(pair.p1).or_default().push(pair);
//         // res.entry(pair.p2).or_default().push(pair);
//     }
// }

fn create_ds<'a>(pairs: &'a Vec<Pair<'_>>) -> HashMap<&'a str, HashMap<&'a str, usize>> {
    let mut ret: HashMap<&'a str, HashMap<&'a str, usize>> = HashMap::new();
    for (pos, pair) in pairs.iter().enumerate() {
        // println!("{}={:?}{:?}", pos, pair.p1, pair.p2);
        ret.entry(pair.p1).or_default().insert(pair.p2, pos);
        // ret.entry(pair.p1).or_default().entry(pair.p2).and_modify(|e| {
        //     println!("E: {:?}", *e);
        //     if *e < 0 || pairs[*e as usize].p1 != pair.p1 {
        //         *e = pos as i32
        //     }
        // });

        // match ret.entry(pair.p1).or_default().entry(pair.p2) {
        //     // .or_insert();
        //     Entry::Occupied(o) => {
        //         let v = o.into_mut();
        //         println!("{:?}", v);
        //         println!("{:?}", pos);
        //         v
        //     },
        //     Entry::Vacant(v) => v.insert(pos as i32),
        // };

        // println!("{}={:?}{:?}", pos, pair.p2, pair.p1);
        ret.entry(pair.p2).or_default().entry(pair.p1).or_insert(pos);
        // println!();
    }
    ret
}

fn traverse_ds<'a>(root: &str, next: &str, pairs: &'a Vec<Pair<'a>>, input: &HashMap<&'a str, HashMap<&'a str, usize>>, current_path: &mut BTreeMap<usize, u8>, output: &mut Vec<Vec<&'a Pair<'a>>>) {
    // println!("ROOT: {:?}, CUR: {:?}", root, next);
    if current_path.len() > 1 && root == next {
        // println!("IF {:?}", current_path.iter().map(|x| &pairs[*x.0]));
        output.push(current_path.iter().map(|x| &pairs[*x.0]).collect());
    }
    else {
        // println!("ELSE");
        for (key, value) in input.get(next).unwrap().iter() {
            // println!("Looping KEY: {}", key);
            if !current_path.contains_key(value) {
                let mut new_path = current_path.clone();
                let c = &pairs[*value];
                new_path.insert(*value, 0);
                let next_pair = if c.p1 == next { c.p2 } else { c.p1 };
                // println!("Processing {:?}", next_pair);
                traverse_ds(root, key, pairs, input, &mut new_path, output);
            }
        }
    }

    // println!("Done: {:?}", output);
    // println!("Done\n");
}

// fn traverse_ds<'a>(root: &str, next: &str, pairs: &'a Vec<Pair<'a>>, input: &HashMap<&'a str, HashMap<&'a str, usize>>, current_path: &mut MapVec<usize, &'a Pair<'a>>, output: &mut Vec<Vec<&'a Pair<'a>>>) {
//     println!("ROOT: {:?}, CUR: {:?}", root, next);
//     if current_path.len() > 1 && root == next {
//         println!("IF {:?}", current_path.v);
//         output.push(*current_path.get());
//     }
//     else {
//         println!("ELSE");
//         for (key, value) in input.get(next).unwrap().iter() {
//             println!("Looping KEY: {}", key);
//             if !current_path.contains_key(value) {
//                 let mut new_path = current_path.clone();
//                 let c = &pairs[*value];
//                 new_path.insert(*value, c);
//                 let next_pair = if c.p1 == next { c.p2 } else { c.p1 };
//                 println!("Processing {:?}", next_pair);
//                 traverse_ds(root, key, pairs, input, &mut new_path, output);
//             }
//         }
//     }
//
//     // println!("Done: {:?}", output);
//     println!("Done\n");
// }

// fn traverse_ds<'a>(root: &str, current: &'a str, dst: &HashMap<&'a str, HashMap<&'a str, usize>>, nodes: &mut HashMap<&'a str, usize>, pairs: &'a Vec<Pair<'_>>, output: &mut Vec<HashMap<usize, &'a Pair<'a>>>) {
// fn traverse_ds<'a>(root: &str, current: &'a str, dst: &HashMap<&'a str, HashMap<&'a str, usize>>, nodes: &mut HashMap<&'a str, usize>, pairs: &'a Vec<Pair<'_>>, output: &mut Vec<Vec<&'a Pair<'a>>>) {
//     println!();
//     for (key, value) in dst.get(current).unwrap().iter() {
//         println!("ROOT: {:?}, CUR: {:?}, KEY: {:?}", root, current, key);
//         nodes.entry(current).or_insert(0);
//         if &root == key {
//             println!("IF {:?}", nodes);
//             println!("IF CUR {:?}", current);
//             let _len = output.len() - 1;
//             output[_len].push(&pairs[*value]);
//             // println!("{:?} {:?} {:?}", output[_len].len(), _len, output[_len].len() >= _len);
//             // if output[_len].len() >= _len && output[_len][output[_len].len() - 1].pair_id == pairs[*value].pair_id {
//             //     println!("IF Remove");
//             //     output[_len].remove(_len);
//             // }
//             // else {
//             //     println!("IF Add");
//             //     output[_len].push(&pairs[*value]);
//             // }
//             // output[_len].entry(*value).or_insert(&pairs[*value]);
//             // println!("BEFORE APPEND: {:?}", output);
//             // output.append(&mut vec![HashMap::new()]);
//             // if output[_len].len() != 0 {
//             //     output.append(&mut vec![vec![]]);
//             // }
//             // println!("AFTER APPEND: {:?}", output);
//         }
//         else if *nodes.get(current).unwrap() == 0 {
//             println!("ELSE IF {:?}", nodes);
//             let _len = output.len() - 1;
//             output[_len].push(&pairs[*value]);
//             // output[_len].entry(*value).or_insert(&pairs[*value]);
//             // println!("CHECK Output {:?}", output);
//             // println!("CHECK Len {:?}", _len);
//
//             // let _len = output.len() - 1;
//             // let mut new_output = output[_len].clone();
//             // new_output.push(&pairs[*value]);
//             // println!("NEW {:?}", new_output);
//             // output.append(&mut vec![new_output]);
//
//             *nodes.entry(current).or_insert(0) += 1;
//             traverse_ds(root, key, dst, nodes, pairs, output);
//
//             // output.append(&mut vec![]);
//             println!("NODE: {:?}", nodes);
//             println!("OUTPUT: {:?}", output);
//             println!();
//         }
//         else {
//             println!("Else");
//         }
//     }
//     println!("Done");
//     nodes.clear();
// }

fn process<'a>(pairs: &'a Vec<Pair<'_>>) -> HashMap<&'a str, Vec<&'a Pair<'a>>> {
    let mut res: HashMap<&'a str, Vec<&'a Pair<'a>>> = HashMap::new();
    for pair in pairs {
        res.entry(pair.p1).or_default().push(pair);
        res.entry(pair.p2).or_default().push(pair);
    }
    res
}

fn traverse<'a>(start: &str, next: &str, input: &HashMap<&str, Vec<&'a Pair<'a>>>, current_path: Vec<&'a Pair<'a>>, output: &mut Vec<Vec<&'a Pair<'a>>>) {
    // println!("Start: {:?}", start);
    // println!("Next: {:?}", next);
    // println!("CurrentPath: {:?}", current_path);
    if current_path.len() > 1 && start == next {
        // println!("IF");
        output.push(current_path);
    } else {
        let iter = input.get(next)
            .unwrap()
            .iter()
            .filter(|e | !current_path.contains(e));
        // println!("GET {:?}", input.get(next));
        // println!("ELSE Next {:?}", next);
        // println!("ELSE {:?}", iter);
        for i in iter {
            // println!("LOOP {:?}", i);
            let mut new_path = current_path.clone();
            new_path.push(i);
            let next_pair = if i.p1 == next { i.p2 } else { i.p1 };
            traverse(start, next_pair, input, new_path, output);
        }
        // println!();
    }
}
