use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            size: 0,
        }
    }

    pub fn push_head(&mut self, item: T) {
        let head_node = Node::new_mut_ptr(self, item);

        if self.size == 0 {
            self.head = head_node;
            self.tail = head_node;
            self.size = 1;
        } else {
            let original_head = self.head;
            unsafe {
                (*original_head).prev = head_node;
                (*head_node).next = original_head;
            }
            self.head = head_node;
            self.size += 1;
        }
    }

    pub fn push_tail(&mut self, item: T) {
        let tail_node = Node::new_mut_ptr(self, item);

        if self.size == 0 {
            self.head = tail_node;
            self.tail = tail_node;
            self.size = 1;
        } else {
            let original_tail = self.tail;
            unsafe {
                (*original_tail).next = tail_node;
                (*tail_node).prev = original_tail;
            }
            self.tail = tail_node;
            self.size += 1;
        }
    }

    pub fn head(&self) -> Option<&Node<T>> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&*self.head) }
        }
    }

    pub fn tail(&self) -> Option<&Node<T>> {
        if self.tail.is_null() {
            None
        } else {
            unsafe { Some(&*self.tail) }
        }
    }

    pub fn head_mut(&mut self) -> Option<&mut Node<T>> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.head) }
        }
    }

    pub fn tail_mut(&mut self) -> Option<&mut Node<T>> {
        if self.tail.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.tail) }
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.head;

        while !cursor.is_null() {
            unsafe {
                let next_cursor = (*cursor).next;
                drop(Box::from_raw(cursor));

                cursor = next_cursor;
            }
        }
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Display for DoublyLinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut format_string = String::new();
        let mut start = self.head();
        while let Some(c) = start {
            format_string.push_str(&format!("{}", c.item()));
            start = c.next();
        }

        write!(f, "{}", format_string)
    }
}

pub struct Node<T> {
    item: T,
    linked_list: *mut DoublyLinkedList<T>,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    fn new_mut_ptr(linked_list: *mut DoublyLinkedList<T>, item: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            item,
            linked_list,
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }))
    }

    pub fn item(&self) -> &T {
        &self.item
    }

    pub fn next(&self) -> Option<&Node<T>> {
        if self.next.is_null() {
            None
        } else {
            unsafe { Some(&*self.next) }
        }
    }

    pub fn next_mut(&mut self) -> Option<&mut Node<T>> {
        if self.next.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.next) }
        }
    }

    pub fn prev(&self) -> Option<&Node<T>> {
        if self.prev.is_null() {
            None
        } else {
            unsafe { Some(&*self.prev) }
        }
    }

    pub fn prev_mut(&mut self) -> Option<&mut Node<T>> {
        if self.prev.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.prev) }
        }
    }

    pub fn push_next(&mut self, item: T) {
        if self.next.is_null() {
            unsafe {
                (*self.linked_list).push_tail(item);
            }
        } else {
            let new_node = Node::new_mut_ptr(self.linked_list, item);
            let original_next = self.next;
            unsafe {
                (*original_next).prev = new_node;
                (*new_node).prev = self;
                (*new_node).next = original_next;
                self.next = new_node;
                (*self.linked_list).size += 1;
            }
        }
    }

    pub fn push_prev(&mut self, item: T) {
        if self.prev.is_null() {
            unsafe {
                (*self.linked_list).push_head(item);
            }
        } else {
            let new_node = Node::new_mut_ptr(self.linked_list, item);
            let original_prev = self.prev;
            unsafe {
                (*original_prev).next = new_node;
                (*new_node).next = self;
                (*new_node).prev = original_prev;
                self.prev = new_node;
                (*self.linked_list).size += 1;
            }
        }
    }

    pub fn pop_next(&mut self) -> Option<T> {
        if self.next.is_null() {
            None
        } else {
            unsafe {
                let next_ptr = self.next;
                let second_next_ptr = (*next_ptr).next;

                self.next = second_next_ptr;
                if second_next_ptr.is_null() {
                    (*self.linked_list).tail = self;
                } else {
                    (*second_next_ptr).prev = self;
                }
                (*self.linked_list).size -= 1;

                Some(Box::from_raw(next_ptr).item)
            }
        }
    }

    pub fn pop_prev(&mut self) -> Option<T> {
        if self.prev.is_null() {
            None
        } else {
            unsafe {
                let prev_ptr = self.prev;
                let second_prev_ptr = (*prev_ptr).prev;

                self.prev = second_prev_ptr;
                if second_prev_ptr.is_null() {
                    (*self.linked_list).head = self;
                } else {
                    (*second_prev_ptr).next = self;
                }
                (*self.linked_list).size -= 1;

                Some(Box::from_raw(prev_ptr).item)
            }
        }
    }
}

pub struct Cursor<'a, T> {
    current: *mut Node<T>,
    linked_list: *mut DoublyLinkedList<T>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Cursor<'a, T> {
    pub fn new(item: &'a mut Node<T>) -> Self {
        Cursor {
            current: item,
            linked_list: item.linked_list,
            _phantom: PhantomData,
        }
    }

    pub fn move_next(&mut self) -> bool {
        unsafe {
            let next_ptr = (*self.current).next;
            if !next_ptr.is_null() {
                self.current = next_ptr;
                true
            } else {
                false
            }
        }
    }

    pub fn move_prev(&mut self) -> bool {
        unsafe {
            let prev_ptr = (*self.current).prev;
            if !prev_ptr.is_null() {
                self.current = prev_ptr;
                true
            } else {
                false
            }
        }
    }
}
impl<'a, T> Deref for Cursor<'a, T> {
    type Target = Node<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.current }
    }
}

impl<'a, T> DerefMut for Cursor<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.current }
    }
}

impl<'a, T> Display for Cursor<'a, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "{}", &(*self.linked_list).to_string()) }
    }
}
