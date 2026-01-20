pub enum LinkedList{
    Cons(i32, Box<LinkedList>),
    Nil
}