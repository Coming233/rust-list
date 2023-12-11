// 简单的链表定义
// pub enum List {
//     Empty,
//     // Box智能指针包裹住才能定义递归的数据结构。
//     Elem(i32, Box<List>),
//     //使用 Box 将值封装到堆上，然后使用栈上的定长指针来指向堆上不定长的值。
// }

// struct Node {
//     elem: i32,
//     next: List,
// }

// pub enum List {
//     Empty,
//     More(Box<Node>),
// }

// // List 的尾部不会再分配多余的 junk 值，通过!
// // List 枚举的形式可以享受 null 指针优化，完美！
// // 所有的元素都拥有统一的内存分配，Good!

/* ----------------------------- 从编程角度 ---------------------------- */
// 实现细节保留在内部
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        // 头插法
        let new_node = Box::new(Node {
            elem: elem,
            // next: self.head,报错，因为会将self.head的所有权占用了。
            // 虽然可以采用clone的方法，直接clone内存。
            // mem::replace允许我们从一个借用中偷出一个值的同时再放入一个新值。
            next: std::mem::replace(&mut self.head, Link::Empty),
            // 所以此处的self.head的值取出来了给到了next字段，然后再将Link::Empty类型传回给self.head。
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        // 此处仍然需要将self.head的值偷换出来。
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // unimplemented!()
    }
}

// 还需要自己写Drop，因为自带的Drop并不够优秀。
/* ---------------------------------------------------------------- */
/* ---------------------------------------------------------------- */
/* ---------------------------------------------------------------- */
/* ------------------------- 下方是模拟编译器的Drop ------------------------ */
// impl Drop for List {
//     fn drop(&mut self) {
//         // NOTE: 在 Rust 代码中，我们不能显式的调用 `drop` 方法，只能调用 std::mem::drop 函数
//         // 这里只是在模拟编译器!
//         self.head.drop(); // 尾递归 - good!
//     }
// }

// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {} // Done!
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop(); // 尾递归 - good!
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop(); // 糟糕，这里不是尾递归!
//         deallocate(self.ptr); // 不是尾递归的原因是在 `drop` 后，还有额外的操作
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }
/* ---------------------------------------------------------------- */
/* ---------------------------------------------------------------- */
/* ---------------------------------------------------------------- */
impl Drop for List {
    fn drop(&mut self) {
        // 先将头指针进行替换。
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
/* ----------------------------- 测试代码 ----------------------------- */
#[cfg(test)]
mod test {
    #[test]
    fn basic() {
        use super::List;
        let mut list = List::new();
        // 测试空值
        assert_eq!(list.pop(), None);

        list.push(3);
        list.push(2);
        list.push(1);

        // 测试弹出
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(5);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(3));

        // 测试空值弹出
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn long_list() {
        // 如果不优化尾递归的Drop Trait，会导致栈溢出。
        use super::List;
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }
}
