// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,

    _marker: PhantomData<T>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    elem: T,
    front: Link<T>,
    back: Link<T>,
}

pub struct Cursor<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    front: Link<T>,
    _marker: PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            _marker: PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.front.is_none()
    }

    pub fn len(&self) -> usize {
        unsafe {
            let mut len = 0;
            let mut cur_link = self.front;
            while let Some(cur) = cur_link {
                len += 1;
                cur_link = (*cur.as_ptr()).back;
            }
            len
        }
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.front,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            cur: self.back,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            front: self.front,
            _marker: PhantomData,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.cur.map(|node| unsafe { &mut (*node.as_ptr()).elem })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur = if let Some(cur) = self.cur {
                (*cur.as_ptr()).back
            } else {
                self.list.front
            };
            self.cur.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur = if let Some(cur) = self.cur {
                (*cur.as_ptr()).front
            } else {
                self.list.back
            };
            self.cur.map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unsafe {
            if let Some(cur) = self.cur.take() {
                let mut boxed_node = Box::from_raw(cur.as_ptr());
                let back = boxed_node.back.take();
                let front = boxed_node.front.take();

                if let Some(back) = back {
                    (*back.as_ptr()).front = front;
                }

                if let Some(front) = front {
                    (*front.as_ptr()).back = back;
                }

                if let Some(back) = back {
                    self.cur = Some(back);
                } else if let Some(front) = front {
                    self.cur = Some(front);
                }

                if self.list.front.unwrap() == cur {
                    self.list.front = self.cur;
                }

                if self.list.back.unwrap() == cur {
                    self.list.back = self.cur;
                }

                Some(boxed_node.elem)
            } else if let Some(front) = self.list.front.take() {
                let mut boxed_node = Box::from_raw(front.as_ptr());

                self.cur = boxed_node.back.take();
                if let Some(cur) = self.cur {
                    (*cur.as_ptr()).front = None;
                }

                self.list.front = self.cur;

                if self.list.back.unwrap() == front {
                    self.list.back = self.cur;
                }

                Some(boxed_node.elem)
            } else {
                None
            }
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem: element,
                front: None,
                back: None,
            })));

            if let Some(cur) = self.cur {
                let back = (*cur.as_ptr()).back;
                (*cur.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(cur);

                if let Some(back) = back {
                    (*back.as_ptr()).front = Some(new);
                    (*new.as_ptr()).back = Some(back);
                }

                if self.list.back.unwrap() == cur {
                    self.list.back = Some(new)
                }
            } else if let Some(back) = self.list.back {
                (*back.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(back);
                self.list.back = Some(new);
            } else {
                self.list.front = Some(new);
                self.list.back = Some(new);
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                elem: element,
                front: None,
                back: None,
            })));

            if let Some(cur) = self.cur {
                let front = (*cur.as_ptr()).front;
                (*cur.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(cur);

                if let Some(front) = front {
                    (*front.as_ptr()).back = Some(new);
                    (*new.as_ptr()).front = Some(front);
                }

                if self.list.front.unwrap() == cur {
                    self.list.front = Some(new);
                }
            } else if let Some(front) = self.list.front {
                (*front.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(front);
                self.list.front = Some(new);
            } else {
                self.list.front = Some(new);
                self.list.back = Some(new);
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.front.map(|node| unsafe {
            self.front = (*node.as_ptr()).back;
            &(*node.as_ptr()).elem
        })
    }
}

use std::fmt::{self, Debug};

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}
