#![allow(non_snake_case)]

use std::mem;

trait OptionExt<T> {
    #[inline]
    fn replace_with<F>(&mut self, f: F)
    where
        F: FnOnce(Option<T>) -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn replace_with<F>(&mut self, f: F)
    where
        F: FnOnce(Option<T>) -> Option<T>,
    {
        let mut old_value = unsafe { mem::uninitialized() };
        mem::swap(self, &mut old_value);
        let mut new_value = f(old_value);
        mem::swap(self, &mut new_value);
        // After two swaps (`old_value` -> `self` -> `new_value`), `new_value`
        // holds an `uninitialized` value, so we just forget about it. 
        mem::forget(new_value);
    }
}

type NodeCell = Option<Box<Node>>;

struct Node {
    value: i32,
    left: NodeCell,
    right: NodeCell,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

fn merge__using_naive_assignment(lower: NodeCell, greater: NodeCell) -> NodeCell {
    match (lower, greater) {
        (None, greater) => greater,

        (lower, None) => lower,

        (Some(mut lower_node), Some(mut greater_node)) => {
            if lower_node.value < greater_node.value {
                lower_node.right =
                    merge__using_naive_assignment(lower_node.right.take(), Some(greater_node));
                Some(lower_node)
            } else {
                greater_node.left =
                    merge__using_naive_assignment(Some(lower_node), greater_node.left.take());
                Some(greater_node)
            }
        }
    }
}

fn merge__using_mem_swap_forget(lower: NodeCell, greater: NodeCell) -> NodeCell {
    match (lower, greater) {
        (None, greater) => greater,

        (lower, None) => lower,

        (Some(mut lower_node), Some(mut greater_node)) => {
            if lower_node.value < greater_node.value {
                let mut node = unsafe { mem::uninitialized() };
                mem::swap(&mut lower_node.right, &mut node);
                let mut merged = merge__using_mem_swap_forget(node, Some(greater_node));
                mem::swap(&mut lower_node.right, &mut merged);
                mem::forget(merged);
                Some(lower_node)
            } else {
                let mut node = unsafe { mem::uninitialized() };
                mem::swap(&mut lower_node.right, &mut node);
                let mut merged = merge__using_mem_swap_forget(Some(lower_node), node);
                mem::swap(&mut greater_node.left, &mut merged);
                mem::forget(merged);
                Some(greater_node)
            }
        }
    }
}

fn merge__using_replace_with(lower: NodeCell, greater: NodeCell) -> NodeCell {
    match (lower, greater) {
        (None, greater) => greater,

        (lower, None) => lower,

        (Some(mut lower_node), Some(mut greater_node)) => {
            if lower_node.value < greater_node.value {
                lower_node
                    .right
                    .replace_with(|node| merge__using_replace_with(node, Some(greater_node)));
                Some(lower_node)
            } else {
                greater_node
                    .left
                    .replace_with(|node| merge__using_replace_with(Some(lower_node), node));
                Some(greater_node)
            }
        }
    }
}

#[macro_use]
extern crate criterion;
use criterion::Criterion;

fn setup_nodes() -> (NodeCell, NodeCell) {
    let mut lower = Node::new(10);
    lower.left = Some(Box::new(Node::new(5)));
    lower.right = Some(Box::new(Node::new(15)));
    let mut greater = Node::new(20);
    greater.left = Some(Box::new(Node::new(16)));
    (Some(Box::new(lower)), Some(Box::new(greater)))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_functions(
        "Replace Option with a new value computed from an old value",
        vec![
            criterion::Fun::new("naive assignment", |b, _| {
                b.iter_with_setup(setup_nodes, |(lower, greater)| {
                    merge__using_naive_assignment(lower, greater)
                })
            }),
            criterion::Fun::new("mem::swap + mem::forget", |b, _| {
                b.iter_with_setup(setup_nodes, |(lower, greater)| {
                    merge__using_mem_swap_forget(lower, greater)
                })
            }),
            criterion::Fun::new("Option::replace_with", |b, _| {
                b.iter_with_setup(setup_nodes, |(lower, greater)| {
                    merge__using_replace_with(lower, greater)
                })
            }),
        ],
        0,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
