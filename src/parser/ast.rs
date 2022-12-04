use std::{cell::RefCell, fmt, rc::Rc};

use crate::parser::WState as State;

#[derive(Clone)]
pub struct Node {
    ele: State,
    children: Vec<Rc<RefCell<Box<Node>>>>,
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}===\n", self.ele).unwrap();
        for i in &self.children {
            write!(f, "{} ", i.borrow()).unwrap();
        }
        Ok(())
    }
}
impl Node {
    pub fn new(ele: State) -> Node {
        Node {
            ele,
            children: Vec::new(),
        }
    }
    pub fn push(&mut self, to_push: &Vec<State>) {
        let lock = Rc::new(RefCell::new(false));
        self._push(to_push, lock);
    }

    pub fn _push(&mut self, to_push: &Vec<State>, lock: Rc<RefCell<bool>>) {
        if lock.borrow().clone() {
            return;
        }
        if self.ele.is_terminal() {
            return;
        }
        if self.children.len() == 0 {
            for i in to_push {
                let node = Node::new(i.clone());
                self.children.push(Rc::new(RefCell::new(Box::new(node))));
            }
            lock.borrow_mut().clone_from(&true);
            return;
        }
        for i in &self.children {
            // take i and push to it
            i.borrow_mut()._push(to_push, lock.clone());
        }
    }
    pub fn print(&self, offset: usize) {
        println!("{:?} ", self.ele);

        for i in &self.children {
            let las = offset / 4;
            for _ in 0..las + 1 {
                print!("|-");
            }
            i.clone().borrow().print(offset + 4);
        }
    }
}
