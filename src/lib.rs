use std::boxed::Box;
use std::cell::RefCell;
use std::cmp::PartialOrd;
struct Element<K: PartialOrd, V> {
    pub key: K,
    pub value: V,
}
pub struct Heap<K: PartialOrd, V> {
    elements: Vec<RefCell<Box<Element<K, V>>>>,
}

fn father_index(element_index: usize) -> usize {
    return (element_index + 1) / 2 - 1;
}
fn child_index_l(element_index: usize) -> usize {
    return (element_index + 1) * 2 - 1;
}
fn child_index_r(element_index: usize) -> usize {
    return (element_index + 1) * 2;
}

impl<K: PartialOrd, V> Heap<K, V> {
    pub fn new() -> Heap<K, V> {
        let elements = vec![];
        Heap { elements }
    }
    fn validate(&self) {
        for (index, elem) in self.elements.iter().enumerate() {
            let left_son = self.elements.get(child_index_l(index));
            if let Some(left_son) = left_son {
                assert!(left_son.borrow().key >= elem.borrow().key);
            }
            let right_son = self.elements.get(child_index_r(index));
            if let Some(right_son) = right_son {
                assert!(right_son.borrow().key >= elem.borrow().key);
            }
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.elements
            .push(RefCell::new(Box::new(Element { key, value })));
        let mut index = self.elements.len() - 1;
        while index != 0 {
            let cur = self.elements.get(index);
            let index_of_father = father_index(index);
            let father = self.elements.get(index_of_father);
            if father.unwrap().borrow().key > cur.unwrap().borrow().key {
                father.unwrap().swap(cur.unwrap());
                index = index_of_father;
            } else {
                break;
            }
        }
        self.validate();
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.elements.len() == 0 {
            return None
        }
        if self.elements.len() == 1 {
            let result = self.elements.remove(self.elements.len() - 1).into_inner();
            return Some((result.key, result.value))
        }
        self.elements[self.elements.len() - 1].swap(&self.elements[0]);
        let result = self.elements.remove(self.elements.len() - 1).into_inner();
        let mut index = 0;
        let mut new_index;
        while child_index_l(index) < self.elements.len() {
            let cur = self.elements.get(index);
            let left_child_index = child_index_l(index);
            let right_child_index = child_index_r(index);
            if let Some(right_child) = self.elements.get(right_child_index) {
                let left_child_borrowed = self.elements.get(left_child_index).unwrap().borrow();
                let right_child_borrowed = right_child.borrow();
                if left_child_borrowed.key < right_child_borrowed.key {
                    let min = &left_child_borrowed.key;
                    new_index = left_child_index;
                    if min >= &cur.unwrap().borrow().key {
                        break;
                    }
                } else {
                    let min = &right_child_borrowed.key;
                    new_index = right_child_index;
                    if min >= &cur.unwrap().borrow_mut().key {
                        break;
                    }  
                };
            } else if let Some(left_child) = self.elements.get(left_child_index) {
                let left_child_borrowed = left_child.borrow();
                let min = &left_child_borrowed.key;
                new_index = left_child_index;
                if min >= &cur.unwrap().borrow().key {
                    break;
                }
            } else {
                break;
            }
            let new_child = self.elements.get(new_index).unwrap(); 
            cur.unwrap().swap(new_child);
            index = new_index; 

        }

        self.validate();
        Some((result.key, result.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    #[test]
    fn heap_push() {
        let mut heap: Heap<i32, i32> = Heap::new();
        heap.push(2, 1);
        heap.push(1, 1);
        heap.push(3, 1);
        heap.push(4, 1);
        heap.push(-1, 1);
        heap.push(9, 1);
    }
    #[test]
    fn heap_push_harder() {
        let mut heap: Heap<i32, i32> = Heap::new();
        for _ in 0..1000 {
            heap.push(random(), 1);
        }
    }
    #[test]
    fn pop_empty() {
        let mut heap: Heap<i32, i32> = Heap::new();
        assert_eq!(None, heap.pop());
    }
    #[test]
    fn pop_full() {
        let mut heap: Heap<i32, i32> = Heap::new();
        heap.push(2, 1);
        heap.push(1, 1);
        heap.push(3, 1);
        heap.push(4, 1);
        heap.push(-1, 1);
        heap.push(9, 1);
        assert_eq!(-1, heap.pop().unwrap().0);
        assert_eq!(1, heap.pop().unwrap().0);
        assert_eq!(2, heap.pop().unwrap().0);
        assert_eq!(3, heap.pop().unwrap().0);
        assert_eq!(4, heap.pop().unwrap().0);
        assert_eq!(9, heap.pop().unwrap().0);
        assert_eq!(None, heap.pop());
    }
}
