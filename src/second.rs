/* ----------------------------- 从编程角度 ---------------------------- */
// 实现细节保留在内部
pub struct List<T> {
    head: Link<T>,
}
// 使用类型别名
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // Option 直接提供了一个方法 take 用于替代std::mem::replace
            // next: std::mem::replace(&mut self.head, None),
            next: self.head.take(),
        });
        self.head = Link::Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        // 使用map来进行映射
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    // 以下方式报错，因为尝试将一个局部的内部值返回给函数调用者了。
    // pub fn peek(&self) -> Option<&T> {
    //     self.head.map(|node| &node.elem)
    // }
    pub fn peek(&self) -> Option<&T> {
        // .as_ref()将一个 Option<T> 变成了 Option<&T>
        // self.head.map获取到的是一个Option<T>
        self.head.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

/* ----------------------------- 迭代器实现 ---------------------------- */
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // 直接弹出元组结构体的第0个元素
        self.0.pop()
    }
}
/* ----------------------------- Iter实现 ----------------------------- */
// Iter是获取不可变引用的迭代器类型
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// 这里无需生命周期，因为 List 没有使用生命周期的关联项
impl<T> List<T> {
    // 这里我们为 `iter` 声明一个生命周期 'a , 此时 `&self` 需要至少和 `Iter` 活得一样久
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// 这里声明生命周期是因为下面的关联类型 Item 需要
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    // 这里无需更改，因为上面已经处理了.
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

/* ---------------------------- IterMut --------------------------- */

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

// TDD测试驱动开发
/* ----------------------------- 测试代码 ----------------------------- */
#[cfg(test)]
mod test {
    use std::ops::{Deref, DerefMut};

    use super::List;
    #[test]
    fn basic() {
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
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);

        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(list.peek(), Some(&1));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
