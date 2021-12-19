use std::fmt;
use std::rc::Weak;
use std::{cell::RefCell, rc::Rc};

type NodeRef = Rc<RefCell<Box<Node>>>;
type WeakNodeRef = Weak<RefCell<Box<Node>>>;

pub struct Node {
    parent: Option<WeakNodeRef>,
    val: NodeVal,
}

impl Node {
    fn new(parent: Option<WeakNodeRef>, val: NodeVal) -> Self {
        Self { parent, val }
    }

    fn leaf(&self) -> Option<usize> {
        match self.val {
            NodeVal::Leaf(x) => Some(x),
            _ => None,
        }
    }

    fn regular_pair(&self) -> Option<[usize; 2]> {
        match &self.val {
            NodeVal::Children([l, r]) => l
                .borrow()
                .leaf()
                .and_then(|left| r.borrow().leaf().map(|right| [left, right])),
            NodeVal::Leaf(_) => None,
        }
    }

    fn add_to_leaf(&mut self, add: usize) {
        let new_val = match self.val {
            NodeVal::Leaf(x) => x + add,
            _ => unreachable!(),
        };
        self.val = NodeVal::Leaf(new_val);
    }
}

macro_rules! noderef {
    ($parent: expr, $val: expr) => {
        Rc::new(RefCell::new(Box::new(Node::new($parent, $val))))
    };
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.val {
            NodeVal::Leaf(x) => write!(f, "{}", x),
            NodeVal::Children([l, r]) => {
                write!(f, "[{:?},{:?}]", l.borrow(), r.borrow())
            }
        }
    }
}

#[derive(Debug)]
enum NodeVal {
    Children([Rc<RefCell<Box<Node>>>; 2]),
    Leaf(usize),
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Explode,
    Split,
}

fn find_actionable(node: &NodeRef) -> Option<(Action, NodeRef)> {
    fn explode_action(node: &NodeRef, depth: usize) -> Option<(Action, NodeRef)> {
        let inner = node.borrow();
        if let NodeVal::Children([l, r]) = &inner.val {
            if depth == 4 && inner.regular_pair().is_some() {
                return Some((Action::Explode, node.clone()));
            } else {
                return explode_action(l, depth + 1).or_else(|| explode_action(r, depth + 1));
            }
        };
        None
    }
    fn split_action(node: &NodeRef) -> Option<(Action, NodeRef)> {
        let inner = node.borrow();
        match &inner.val {
            NodeVal::Children([l, r]) => {
                return split_action(l).or_else(|| split_action(r));
            }
            NodeVal::Leaf(x) => {
                if *x >= 10 {
                    return Some((Action::Split, node.clone()));
                }
            }
        };
        None
    }
    explode_action(node, 0).or_else(|| split_action(node))
}

fn rightmost(node: &NodeRef) -> NodeRef {
    match &node.borrow().val {
        NodeVal::Leaf(_) => node.clone(),
        NodeVal::Children([_, r]) => rightmost(r),
    }
}

fn leftmost(node: &NodeRef) -> NodeRef {
    match &node.borrow().val {
        NodeVal::Leaf(_) => node.clone(),
        NodeVal::Children([l, _]) => leftmost(l),
    }
}

fn first_left_leaf_of(node: &NodeRef) -> Option<NodeRef> {
    if let Some(parent) = node.borrow().parent.clone().and_then(|x| x.upgrade()) {
        match &parent.borrow().val {
            NodeVal::Children([left, _]) => {
                if Rc::ptr_eq(left, node) {
                    return first_left_leaf_of(&parent);
                } else {
                    return Some(rightmost(left));
                }
            }
            _ => unreachable!(),
        }
    }
    None
}

fn first_right_leaf_of(node: &NodeRef) -> Option<NodeRef> {
    if let Some(parent) = node.borrow().parent.clone().and_then(|x| x.upgrade()) {
        match &parent.borrow().val {
            NodeVal::Children([_, right]) => {
                if Rc::ptr_eq(right, node) {
                    return first_right_leaf_of(&parent);
                } else {
                    return Some(leftmost(right));
                }
            }
            _ => unreachable!(),
        }
    }
    None
}

fn reduce(node: NodeRef) {
    while let Some((action, node)) = find_actionable(&node) {
        match action {
            Action::Explode => {
                let [left, right] = node.borrow().regular_pair().unwrap();
                if let Some(first_left_leaf) = first_left_leaf_of(&node) {
                    first_left_leaf.borrow_mut().add_to_leaf(left);
                }
                if let Some(first_right_leaf) = first_right_leaf_of(&node) {
                    first_right_leaf.borrow_mut().add_to_leaf(right);
                }
                node.borrow_mut().val = NodeVal::Leaf(0);
            }
            Action::Split => {
                let mut inner = node.borrow_mut();
                if let NodeVal::Leaf(number) = inner.val {
                    let left = noderef!(
                        Some(Rc::downgrade(&node.clone())),
                        NodeVal::Leaf((number as f32 / 2f32).floor() as usize)
                    );
                    let right = noderef!(
                        Some(Rc::downgrade(&node.clone())),
                        NodeVal::Leaf((number as f32 / 2f32).ceil() as usize)
                    );
                    inner.val = NodeVal::Children([left, right]);
                } else {
                    unreachable!()
                }
            }
        }
    }
}

fn add_two(a: NodeRef, b: NodeRef) -> NodeRef {
    let top = noderef!(None, NodeVal::Children([a.clone(), b.clone()]));
    a.borrow_mut().parent = Some(Rc::downgrade(&top));
    b.borrow_mut().parent = Some(Rc::downgrade(&top));
    reduce(top.clone());
    top
}

fn magnitude(node: &NodeRef) -> usize {
    match &node.borrow().val {
        NodeVal::Leaf(x) => *x,
        NodeVal::Children([l, r]) => 3 * magnitude(l) + 2 * magnitude(r),
    }
}

fn add_snail_fish_numbers(s: &str) -> Option<NodeRef> {
    let snail_fish_numbers = s.lines().map(|s| parse_tree(s));
    snail_fish_numbers.into_iter().reduce(add_two)
}

fn largest_magnitude_of_any_sum(s: &str) -> usize {
    s.lines()
        .map(|nodea| {
            s.lines()
                .filter(move |nodeb| &nodea != nodeb)
                .map(|nodeb| magnitude(&add_two(parse_tree(nodea), parse_tree(nodeb))))
        })
        .flatten()
        .max()
        .unwrap()
}

fn main() {
    let content = std::fs::read_to_string("input/18.txt").unwrap();
    let res = add_snail_fish_numbers(content.as_str()).unwrap();
    println!("part 1: {}", magnitude(&res));

    let largest_magnitude = largest_magnitude_of_any_sum(content.as_str());
    println!("part 2: {}", largest_magnitude);
}

#[allow(clippy::redundant_closure_call)]
fn parse_tree(s: &str) -> Rc<RefCell<Box<Node>>> {
    peg::parser! {
        grammar parser() for str {
            rule number() -> usize
                = n:$(['0'..='9']) { ? n.parse().or(Err("usize")) }

            pub rule node() -> Rc<RefCell<Box<Node>>> = precedence! {
                n: number() { noderef!(None,NodeVal::Leaf(n)) }
                --
                "[" l:node() "," r:node() "]" {
                    let mut parent = noderef!(None, NodeVal::Children([l.clone(), r.clone()]));
                    l.borrow_mut().parent = Some(Rc::downgrade(&parent));
                    r.borrow_mut().parent = Some(Rc::downgrade(&parent));
                    parent
                 }
            }
        }
    }

    parser::node(s).unwrap_or_else(|e| panic!("Could not parse {:#?}: {}", s, e))
}

#[cfg(test)]
mod problem18 {
    use super::*;

    macro_rules! reduce_check {
        ($input:expr, $result:expr) => {
            let node = parse_tree($input);
            reduce(node.clone());
            assert_eq!(format!("{:?}", node.borrow()), $result);
        };
    }

    #[test]
    fn parsing() {
        let input = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let node = parse_tree(input);
        assert_eq!(format!("{:?}", node.borrow()), input);
    }

    #[test]
    fn node_magnitude() {
        let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let node = parse_tree(input);
        assert_eq!(3488, magnitude(&node));
    }

    #[test]
    fn add_without_reduce() {
        let left = "[1,2]";
        let right = "[[3,4],5]";
        let nodel = parse_tree(left);
        let noder = parse_tree(right);
        let add = add_two(nodel, noder);
        let add_str = "[[1,2],[[3,4],5]]";
        assert_eq!(format!("{:?}", add.borrow()), add_str);
    }

    #[test]
    fn explode() {
        reduce_check!("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        reduce_check!("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        reduce_check!("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        reduce_check!(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
    }

    #[test]
    fn explode_and_split() {
        reduce_check!(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn add_with_reduce() {
        let left = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let right = "[1,1]";
        let nodel = parse_tree(left);
        let noder = parse_tree(right);
        let add = add_two(nodel, noder);
        let add_str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        assert_eq!(format!("{:?}", add.borrow()), add_str);
    }

    #[test]
    fn part1() {
        let content = std::fs::read_to_string("input/18.test.txt").unwrap();
        let res = add_snail_fish_numbers(content.as_str()).unwrap();
        let res_str = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";

        assert_eq!(format!("{:?}", res.borrow()), res_str);
        assert_eq!(4140, magnitude(&res));
    }

    #[test]
    fn part2() {
        let content = std::fs::read_to_string("input/18.test.txt").unwrap();
        let largest_magnitude = largest_magnitude_of_any_sum(content.as_str());
        assert_eq!(3993, largest_magnitude);
    }
}
