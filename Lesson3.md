# Smart Pointers in Rust: Box, Rc, RefCell
1. Box<T>
2. Linked List
   1. Cons
   2. Rc
   3. RefCell
3. Custom Pointer
4. Drop

## Box
In Box type the data is stored in the Heap(`RAM`), but the reference to that data is stored in the stack(`ROM`).

**What Box does NOT do**

Let’s kill common misconceptions:
❌ Does NOT make things mutable
❌ Does NOT mean shared ownership
❌ Does NOT mean reference counting
❌ Does NOT bypass borrow checker

_It only means: heap allocation with single ownership_
refer to my_box.rs

- Now Box is good when there is single ownership, but for handling multiple ownerships we cannot use Box, for that we use Rc/ Reference Counting
