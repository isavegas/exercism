use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    //dummy: ::std::marker::PhantomData<T>,
    data: Option<T>,
    next: Option<Box<SimpleLinkedList<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            next: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    pub fn len(&self) -> usize {
        let mut l = 0;
        let mut s = self;
        loop {
          if s.data.is_some() {
              l += 1;
          } else {
              break;
          }
          if s.next.is_some() {
              s = s.next.as_ref().unwrap();
          } else {
              break;
          }
        }
        l
    }

    pub fn push(&mut self, element: T) {
        let mut s = self;
        loop {
          if s.data.is_none() {
              s.data = Some(element);
              break;
          }
          if s.next.is_some() {
              s = &mut *s.next.as_mut().unwrap();
          } else {
              let mut f = SimpleLinkedList::new();
              f.data = Some(element);
              s.next = Some(Box::new(f));
              break;
          }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut s = self;
        loop {
          if s.data.is_none() {
              break None;
          }
          if s.next.is_some() {
              s = &mut *s.next.as_mut().unwrap();
          } else {
              let mut r: Option<SimpleLinkedList<T>> = None;
              std::mem::swap(&mut r, &mut Some(s));
              break Some(r.data.unwrap());
          }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
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

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
