// SPDX-FileCopyrightText: 2025 undoredo contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(all(feature = "undoredo", feature = "std"))]

use std::collections::HashMap;

use maplike::{
    containers::Container,
    iter::IntoIter,
    ops::{Get, Insert, Remove},
};
use rstar::primitives::Rectangle;
use rstared::RTreed;
use std::collections::BTreeMap;
use undoredo::aliases::BTreeMapHalfDelta;
use undoredo::{ApplyDelta, Delta, HistoryTree, Recorder, Snapshot, UndoRedo};

impl FromUsize for (i32, i32) {
    fn from_usize(u: usize) -> (i32, i32) {
        (u as i32, 0)
    }
}

impl FromUsize for Rectangle<(i32, i32)> {
    fn from_usize(u: usize) -> Rectangle<(i32, i32)> {
        Rectangle::from_corners((u as i32, 0), (u as i32, 0))
    }
}

#[test]
fn test_recorder_apply_delta_on_set() {
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();
    let recorder = Recorder::<
        RTreed<HashMap<i32, Rectangle<(i32, i32)>>>,
        HashMap<i32, Rectangle<(i32, i32)>>,
    >::new(RTreed::new(rect_hashmap));
    test_recorder_apply_delta_at_specified_indices(recorder);
}

#[test]
fn test_snapshot_undo_redo_() {
    let rect_hashmap: HashMap<i32, Rectangle<(i32, i32)>> = HashMap::new();
    test_snapshot_undo_redo::<i32, Rectangle<(i32, i32)>, _>(RTreed::new(rect_hashmap));
}

trait Keyed<K>: Container<Key = K> {}
impl<T: Container<Key = K>, K> Keyed<K> for T {}

trait Map<V>: Container<Value = V> {}
impl<T: Container<Value = V>, V> Map<V> for T {}

trait FromUsize {
    fn from_usize(u: usize) -> Self;
}

impl FromUsize for i32 {
    fn from_usize(u: usize) -> i32 {
        u.try_into().unwrap()
    }
}

impl FromUsize for usize {
    fn from_usize(u: usize) -> usize {
        u
    }
}

fn test_recorder_apply_delta_at_specified_indices<
    K: Clone + FromUsize + std::fmt::Debug + PartialEq + Ord,
    V: Clone + FromUsize + std::fmt::Debug + PartialEq + Ord,
    C: Keyed<K> + Map<V> + Insert<K> + Remove<K> + Get<K>,
    DC: Clone + Keyed<K> + Map<V> + Get<K> + Insert<K> + IntoIter<K> + Remove<K, Output = Option<V>>,
>(
    mut recorder: Recorder<C, DC>,
) where
    C: ApplyDelta<BTreeMapHalfDelta<K, V>>,
{
    recorder.insert(K::from_usize(1), V::from_usize(10));
    recorder.insert(K::from_usize(2), V::from_usize(20));
    recorder.insert(K::from_usize(3), V::from_usize(30));
    recorder.insert(K::from_usize(4), V::from_usize(40));
    recorder.insert(K::from_usize(5), V::from_usize(50));

    let delta = Delta::with_removed_inserted(
        BTreeMap::from([(K::from_usize(2), V::from_usize(20))]),
        BTreeMap::from([
            (K::from_usize(3), V::from_usize(33)),
            (K::from_usize(6), V::from_usize(66)),
        ]),
    );
    recorder.apply_delta(delta);

    assert_eq!(recorder.get(&K::from_usize(1)), Some(&V::from_usize(10)));
    assert_eq!(recorder.get(&K::from_usize(2)), None);
    assert_eq!(recorder.get(&K::from_usize(3)), Some(&V::from_usize(33)));
    assert_eq!(recorder.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(recorder.get(&K::from_usize(5)), Some(&V::from_usize(50)));
    assert_eq!(recorder.get(&K::from_usize(6)), Some(&V::from_usize(66)));
}

fn test_snapshot_undo_redo<
    K: Clone + FromUsize + std::fmt::Debug + PartialEq,
    V: Clone + FromUsize + std::fmt::Debug + PartialEq,
    C: Keyed<K> + Map<V> + Get<K> + Insert<K> + IntoIter<K> + Remove<K> + Clone,
>(
    mut container: C,
) {
    let mut undoredo: UndoRedo<Snapshot<C>> = UndoRedo::new();
    assert_eq!(undoredo.undo(&mut container), None);
    assert_eq!(undoredo.redo(&mut container), None);

    container.insert(K::from_usize(1), V::from_usize(10));
    container.insert(K::from_usize(2), V::from_usize(20));
    container.insert(K::from_usize(3), V::from_usize(30));
    container.insert(K::from_usize(4), V::from_usize(40));
    container.insert(K::from_usize(5), V::from_usize(50));

    undoredo.commit(&mut container);

    container.remove(&K::from_usize(2));
    container.insert(K::from_usize(1), V::from_usize(11));
    container.insert(K::from_usize(3), V::from_usize(33));

    assert!(undoredo.undo(&mut container).is_some());
    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(10)));
    assert_eq!(container.get(&K::from_usize(2)), Some(&V::from_usize(20)));
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(30)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    assert_eq!(undoredo.undo(&mut container), None);

    assert!(undoredo.redo(&mut container).is_some());
    assert_eq!(container.get(&K::from_usize(1)), Some(&V::from_usize(11)));
    assert_eq!(container.get(&K::from_usize(2)), None);
    assert_eq!(container.get(&K::from_usize(3)), Some(&V::from_usize(33)));
    assert_eq!(container.get(&K::from_usize(4)), Some(&V::from_usize(40)));
    assert_eq!(container.get(&K::from_usize(5)), Some(&V::from_usize(50)));

    assert_eq!(undoredo.redo(&mut container), None);
}

#[test]
fn test_history_tree_command_checkout() {
    let mut history_tree = HistoryTree::<(), u8>::new();
    let mut state = ();

    history_tree.cmd_commit(1, &mut state);
    history_tree.cmd_commit(3, &mut state);
    let left_leaf = history_tree.curr_node();

    assert_eq!(history_tree.undo(&mut state), Some(3));
    assert_eq!(history_tree.undo(&mut state), Some(1));

    history_tree.cmd_commit(2, &mut state);
    history_tree.cmd_commit(4, &mut state);
    let right_leaf = history_tree.curr_node();

    assert_eq!(history_tree.checkout(&mut state, left_leaf), vec![1, 3]);
    assert_eq!(history_tree.curr_node(), left_leaf);

    assert_eq!(history_tree.checkout(&mut state, right_leaf), vec![2, 4]);
    assert_eq!(history_tree.curr_node(), right_leaf);

    // Checkouting to the same node again results in no commands emitted.
    assert_eq!(history_tree.checkout(&mut state, right_leaf), vec![]);
    assert_eq!(history_tree.curr_node(), right_leaf);
}
