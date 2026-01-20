use crate::smart::linked_list::LinkedList::{Cons, Nil};

fn rc_test(){
    let x = Cons(454, Box::new(Cons(453,Box::new(Cons(452, Box::new(Nil))))));
    let a = Cons(455, Box::new(x));
    // let b = Cons(457, Box::new(x));
    // cant be used here since x is already being used, so 2 Box cant ref to one.
}