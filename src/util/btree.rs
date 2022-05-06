use std::collections::BTreeMap;
use std::collections::btree_map::Range;


pub fn btree_lower_Key<'a,K:Ord, V>(tree: &'a BTreeMap<K, V>, val: K) -> Option<(&'a K,&'a V)>{
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Included(val)));

    before.next_back()
}

pub fn btree_upper_Key<'a,K:Ord, V>(tree: &'a BTreeMap<K, V>, val: K) -> Option<(&'a K,&'a V)>{
    use std::ops::Bound::*;

    let mut after = tree.range((Included(val), Unbounded));

    after.next()
}

pub fn btree_extensive_<'a,K:Ord, V>(tree: &'a BTreeMap<K, V>, l:K,r:K) -> Range<'a,K,V> {

    // rust btreeMap没有lower_bound和upper_bound，需要间接实现扩展range

    use std::ops::Bound::*;

    let mut lower_key=btree_lower_Key(tree, l);
    let mut upper_key=btree_upper_Key(tree, r);

    let L=&l;
    let R=&r;
    if lower_key.is_some(){
        L=lower_key.unwrap().0
    }

    if upper_key.is_some(){
        R=upper_key.unwrap().0
    }

    let range = tree.range((Included(L), Included(R)));

    range
}