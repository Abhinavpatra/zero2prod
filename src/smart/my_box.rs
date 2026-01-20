use std::fmt::{Display, Formatter};
use crate::smart::linked_list::LinkedList::{Nil, Cons};


// impl Display for LinkedList{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         // write!(f, "{}", self) => {} calls Display, You are inside Display::fmt, this caused an infinite recursion
//
//     }
// }
pub fn box_test(){
    let x = Cons(454, Box::new(Cons(12,Box::new(Cons(44, Box::new(Nil))))));
    let mut i = &x;
    loop {
        match i {
            Cons(val,next)=>{
                println!("{}", val);
                i = next;
            },
            Nil=>{
                break;
            }
        }
    }
}

