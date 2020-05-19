use std::{
  mem::{swap, ManuallyDrop},
  ptr,
};
use std::fmt::Debug;

/// Max heap which uses a squaring strategy for the number of children
#[derive(Debug)]
pub struct SqrHeap<T> {
  data: Vec<T>,
  ptr: LastPointer,
}

impl<T: Ord + Debug> SqrHeap<T> {
  pub fn new() -> Self {
    Self {
      data: vec![],
      ptr: LastPointer::new(),
    }
  }
  pub fn push(&mut self, t: T) {
    let idx = self.data.len();
    self.data.push(t);
    self.ptr.inc();
    self.sift_up(0, idx);
  }
  // sifts up idx at least until end and returns the final index
  fn sift_up(&mut self, end: usize, idx: usize) -> usize {
    unsafe {
      let mut hole = Hole::new(&mut self.data, idx);
      let mut depth = self.ptr.depth;
      let mut base = self.ptr.base;
      while hole.pos > end {
        let (b, o) = parent_index(hole.pos, depth, base);
        let parent = b + o;
        if hole.curr() <= &hole.data[parent] {
          break;
        }
        hole.move_to(parent);
        depth -= 1;
        base -= base_layer(depth) as usize;
      }
      hole.pos
    }
  }
  pub fn peek(&self) -> Option<&T> { Some(self.data.get(0)?) }
  pub fn pop(&mut self) -> Option<T> {
    let mut item = self.data.pop()?;
    self.ptr.dec();
    if let Some(mut min) = self.data.get_mut(0) {
      swap(&mut item, &mut min);
      self.sift_down(0, self.data.len());
    }
    Some(item)
  }
  fn sift_down(&mut self, idx: usize, end: usize) -> usize {
    let mut curr = idx;
    let mut depth = 0;
    let mut base = 0;
    let mut curr_sibling = 0;
    while curr < end {
      let (b, o) = child_index(base, depth, curr_sibling);
      let offset = b + o;
      let num_children = 2 << depth;
      let posses = &self.data[offset..(offset + num_children).min(self.data.len())];
      // find position of max child
      let max_pos = posses
        .into_iter()
        .enumerate()
        .max_by_key(|v| v.1)
        .map(|v| v.0);

      match max_pos {
        Some(mp) if self.data[offset + mp] > self.data[curr] => {
          self.data.swap(curr, offset + mp);
          curr_sibling = curr_sibling * num_children + mp;
          curr = offset + mp;
        },
        _ => return curr,
      }
      base += base_layer(depth) as usize;
      depth += 1;
    }
    curr
  }
}

#[derive(Debug)]
struct LastPointer {
  base: usize,
  depth: u32,
  last_row_fill: u32,
}

impl LastPointer {
  fn new() -> Self {
    Self {
      base: 0,
      depth: 0,
      last_row_fill: 0,
    }
  }
  fn inc(&mut self) {
    self.last_row_fill += 1;
    let bl = base_layer(self.depth);
    if self.last_row_fill > bl {
      self.base += bl as usize;
      self.depth += 1;
      self.last_row_fill = 1;
    }
  }
  fn dec(&mut self) {
    self.last_row_fill = match self.last_row_fill.checked_sub(1) {
      Some(lrf) => lrf,

      // removed an entire bottom row need to shift up one row
      None => {
        debug_assert!(
          self.depth > 0,
          "Decrementing last pointer before beginning of array"
        );
        self.depth -= 1;
        let prev = base_layer(self.depth);
        self.base -= prev as usize;
        prev as u32 - 1
      },
    };
  }
}

#[test]
fn test_last_ptr() {
  let mut p = LastPointer::new();
  let n = 1000;
  for i in 0..n {
    p.inc();
    assert_eq!(p.base + p.last_row_fill as usize, i + 1);
  }
  for i in 0..n {
    p.dec();
    assert_eq!(p.base + p.last_row_fill as usize, n - i - 1);
  }
}

// used in binary heap, thought I'd use it as well.
struct Hole<'a, T> {
  data: &'a mut [T],
  elt: ManuallyDrop<T>,
  pos: usize,
}

impl<'a, T> Hole<'a, T> {
  unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
    debug_assert!(pos < data.len());
    let elt = ManuallyDrop::new(ptr::read(data.get_unchecked(pos)));
    Self { elt, data, pos }
  }
  fn curr(&self) -> &T { &self.elt }
  unsafe fn move_to(&mut self, idx: usize) {
    debug_assert_ne!(idx, self.pos);
    debug_assert!(idx < self.data.len());
    let index_ptr: *const _ = self.data.get_unchecked(idx);
    let hole_ptr = self.data.get_unchecked_mut(self.pos);
    ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
    self.pos = idx;
  }
}
impl<T> Drop for Hole<'_, T> {
  #[inline]
  fn drop(&mut self) {
    // fill the hole again
    unsafe {
      let pos = self.pos;
      ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
    }
  }
}

/// returns parent base and offset for a given index w/ precomputed depth and base.
fn parent_index(i: usize, depth: u32, base: usize) -> (usize, usize) {
  debug_assert_ne!(depth, 0, "There is no parent for child 0");
  debug_assert!(i >= base);
  let offset = i - base;
  let sibling_num = offset >> depth; // = offset/(1 << depth)?
  let prev_base = base - base_layer(depth - 1) as usize;
  (prev_base, sibling_num)
}

const fn base_layer(d: u32) -> u32 { 1 << (d * (d + 1) / 2) }

/// Returns the next base and offset for this child. The sum is the index.
/// `sibling_num` is which sibling is this being called from
/// `child_num` is which child is being accessed.
const fn child_index(base: usize, depth: u32, sibling_num: usize) -> (usize, usize) {
  let base = base + base_layer(depth) as usize;
  let offset = (2 << depth) * sibling_num;
  (base, offset)
}

#[test]
fn test_parent() {
  // first layer
  assert_eq!((0, 0), parent_index(1, 1, 1));
  assert_eq!((0, 0), parent_index(2, 1, 1));
  // second layer
  assert_eq!((1, 0), parent_index(3, 2, 3));
  assert_eq!((1, 0), parent_index(4, 2, 3));
  assert_eq!((1, 0), parent_index(5, 2, 3));
  assert_eq!((1, 0), parent_index(6, 2, 3));

  assert_eq!((1, 1), parent_index(7, 2, 3));
  assert_eq!((1, 1), parent_index(8, 2, 3));
  assert_eq!((1, 1), parent_index(9, 2, 3));
  assert_eq!((1, 1), parent_index(10, 2, 3));
}

#[test]
fn test_child() {
  // first layer
  assert_eq!((1, 0), child_index(0, 0, 0));
  assert_eq!((1, 0), child_index(0, 0, 0));

  // second layer
  assert_eq!((3, 0), child_index(1, 1, 0));

  assert_eq!((3, 4), child_index(1, 1, 1));
}

#[test]
fn test_basic() {
  let mut sh = SqrHeap::new();
  let n = 13;
  for i in 0..n {
    sh.push(i);
  }
  let mut ptr = LastPointer::new();
  ptr.inc();
  for i in 1..n {
    ptr.inc();
    let (b, o) = parent_index(i, ptr.depth, ptr.base);
    let parent = b + o;
    // checking all parents are greater than all children
    assert!(sh.data[parent] >= sh.data[i], "{:?}: {:?}", i, sh.data);
  }

  let mut out = vec![];
  for _ in 0..n {
    out.push(sh.pop().unwrap());
  }
  for i in 0..n - 1 {
    assert!(out[i] >= out[i + 1]);
  }
}
