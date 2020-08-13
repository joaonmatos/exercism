// Copyright 2020 João Nuno Matos
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    len: usize,
    top: Option<Node<T>>,
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new_simple(element: T) -> Self {
        Self {
            element,
            next: None,
        }
    }

    fn new_linked(element: T, next: Node<T>) -> Self {
        Self {
            element,
            next: Some(Box::new(next)),
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { len: 0, top: None }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        self.top = match &mut self.top {
            None => Some(Node::new_simple(element)),
            Some(_) => Some(Node::new_linked(element, self.top.take().unwrap())),
        };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let top = self.top.take()?;
        self.top = top.next.map(|node| *node);
        self.len -= 1;
        Some(top.element)
    }

    pub fn peek(&self) -> Option<&T> {
        Some(&self.top.as_ref()?.element)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut old = self;
        let mut new = Self::new();
        while let Some(element) = old.pop() {
            new.push(element);
        }
        new
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new = Self::new();
        for item in iter {
            new.push(item);
        }
        new
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::<T>::new();
        let mut list = self.rev();
        while let Some(item) = list.pop() {
            vec.push(item);
        }
        vec
    }
}
