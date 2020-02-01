use std::cmp::PartialOrd;
pub mod gl_render;
struct Element<K: PartialOrd, V> {
    pub key: K,
    pub value: V,
}
pub struct Heap<K: PartialOrd, V> {
    elements: Vec<Element<K,V>>,
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
                assert!(left_son.key >= elem.key);
            }
            let right_son = self.elements.get(child_index_r(index));
            if let Some(right_son) = right_son {
                assert!(right_son.key >= elem.key);
            }
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.elements.push(Element { key, value });
        let mut index = self.elements.len() - 1;
        while index != 0 {
            let cur = &self.elements[index];
            let index_of_father = father_index(index);
            let father = &self.elements[index_of_father];
            if father.key > cur.key {
                self.elements.swap(index, index_of_father);
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
            let Element{key, value} = self.elements.pop()?;
            return Some((key, value))
        }
        let len = self.elements.len();
        self.elements.swap(0, len - 1);
        let result = self.elements.remove(self.elements.len() - 1);
        let mut index = 0;
        let mut new_index = index;
        while child_index_l(index) < self.elements.len() {
            let cur = self.elements.get(index).unwrap();
            let left_child_index = child_index_l(index);
            let right_child_index = child_index_r(index);
            let mut min = &cur.key; 
            if let Some(right_child) = self.elements.get(right_child_index) {
                let left_child = self.elements.get(left_child_index).unwrap();
                if left_child.key < right_child.key {
                    new_index = left_child_index;
                    min = &left_child.key; 
                } else {
                    new_index = right_child_index;
                    min = &right_child.key; 
                };
            } else if let Some(left_child) = self.elements.get(left_child_index) {
                new_index = left_child_index;
                min = &left_child.key; 
            }
            if min < &cur.key {
                self.elements.swap(index, new_index);
                index = new_index;     
            } else {
                break;
            }
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
    #[test]
    fn pop_full_2() {
        let mut heap: Heap<f32, (i32, i32)> = Heap::new();
        heap.push(2., (1,1));
        heap.push(1., (1,1));
        heap.push(3., (1,1));
        heap.push(4., (1,1));
        heap.push(-1., (1,1));
        heap.push(9., (1,1));
        assert_eq!(-1., heap.pop().unwrap().0);
        assert_eq!(1., heap.pop().unwrap().0);
        assert_eq!(2., heap.pop().unwrap().0);
        assert_eq!(3., heap.pop().unwrap().0);
        assert_eq!(4., heap.pop().unwrap().0);
        assert_eq!(9., heap.pop().unwrap().0);
        assert_eq!(None, heap.pop());
    }
}