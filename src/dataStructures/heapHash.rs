use std::collections::HashMap;
struct Element<K: PartialOrd, HashK, V>
where
    HashK: std::cmp::Eq + std::hash::Hash + Copy,
{
    pub key: K,
    pub hash_key: HashK,
    pub value: V,
}
pub struct HeapHash<K: PartialOrd, HashK: std::cmp::Eq + std::hash::Hash + Copy, V> {
    elements: Vec<Element<K, HashK, V>>,
    hash: HashMap<HashK, usize>,
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

impl<K: PartialOrd, HashK: std::cmp::Eq + std::hash::Hash + Copy, V> HeapHash<K, HashK, V> {
    pub fn new() -> HeapHash<K, HashK, V> {
        let elements = vec![];
        let hashMap = HashMap::new();
        HeapHash {
            elements,
            hash: hashMap,
        }
    }
    fn validate(&self) {
        return;
        for (index, elem) in self.elements.iter().enumerate() {
            let left_son = self.elements.get(child_index_l(index));
            if let Some(left_son) = left_son {
                assert!(left_son.key >= elem.key);
            }
            let right_son = self.elements.get(child_index_r(index));
            if let Some(right_son) = right_son {
                assert!(right_son.key >= elem.key);
            }
            let hash_key = &elem.hash_key;
            assert!(
                self.hash.get(hash_key) == Some(&index),
                "index {} hash_key {}",
                index,
                self.hash.get(hash_key).unwrap()
            );
        }
    }
    fn swap(&mut self, index0: usize, index1: usize) {
        *self.hash.get_mut(&self.elements[index0].hash_key).unwrap() = index1;
        *self.hash.get_mut(&self.elements[index1].hash_key).unwrap() = index0;
        self.elements.swap(index0, index1);
    }
    pub fn push(&mut self, key: K, hash_key: HashK, value: V) {
        let length = self.elements.len();
        if self.hash.contains_key(&hash_key) {
            self.update(*self.hash.get(&hash_key).unwrap(), key, value);
            return;
        }
        self.hash.insert(hash_key, length);
        self.elements.push(Element {
            key,
            hash_key,
            value: value,
        });
        let mut index = self.elements.len() - 1;
        while index != 0 {
            let cur = &self.elements[index];
            let index_of_father = father_index(index);
            let father = &self.elements[index_of_father];
            if father.key > cur.key {
                self.swap(index, index_of_father);
                index = index_of_father;
            } else {
                break;
            }
        }
        self.validate();
    }

    fn update(&mut self, index:usize, new_key:K, new_value:V) {
        self.elements[index].value = new_value;
        self.elements[index].key = new_key;
        let mut iter_index = index;
        while iter_index != 0 && self.elements[father_index(iter_index)].key >= self.elements[iter_index].key {
            self.swap(iter_index, father_index(iter_index));
            iter_index = father_index(iter_index);
        }

        while child_index_l(iter_index) < self.elements.len() {
            let cur = self.elements.get(iter_index).unwrap();
            let left_child_index = child_index_l(iter_index);
            let right_child_index = child_index_r(iter_index);
            let mut min = &cur.key;
            let mut new_index = iter_index;
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
                self.swap(iter_index, new_index);
                iter_index = new_index
            } else {
                break;
            }
        }     
        self.validate();   
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.elements.len() == 0 {
            return None;
        }
        if self.elements.len() == 1 {
            self.hash.remove(&self.elements[self.elements.len() - 1].hash_key);
            let Element {
                key,
                hash_key: _hash_key,
                value,
            } = self.elements.pop()?;
            return Some((key, value));
        }
        let len = self.elements.len();
        self.swap(0, len - 1);
        self.hash.remove(&self.elements[self.elements.len() - 1].hash_key);
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
                self.swap(index, new_index);
                index = new_index;
            } else {
                break;
            }
        }
        self.validate();
        Some((result.key, result.value))
    }

    pub fn get(&self, hash_key: &HashK) -> Option<(&K, &V)> {
        let &index = self.hash.get(hash_key)?;
        let result = self.elements.get(index)?;
        Some((&result.key, &result.value))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    #[test]
    fn heap_push() {
        let mut heap: HeapHash<i32, i32, i32> = HeapHash::new();
        heap.push(2, 0, 1);
        heap.push(1, 1, 1);
        heap.push(3, 2, 1);
        heap.push(4, 3, 1);
        heap.push(-1, 4, 1);
        heap.push(9, 5, 1);
    }
    #[test]
    fn heap_push_harder() {
        let mut heap: HeapHash<i32, i32, i32> = HeapHash::new();
        for _ in 0..1000 {
            heap.push(random(), random(), 1);
        }
    }
    #[test]
    fn pop_empty() {
        let mut heap: HeapHash<i32, i32, i32> = HeapHash::new();
        assert_eq!(None, heap.pop());
    }
    #[test]
    fn pop_full() {
        let mut heap: HeapHash<i32, i32, i32> = HeapHash::new();
        heap.push(2, 1, 1);
        heap.push(1, 2, 1);
        heap.push(3, 3, 1);
        heap.push(4, 4, 1);
        heap.push(-1, 5, 1);
        heap.push(9, 6, 1);
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
        let mut heap: HeapHash<f32, i32, (i32, i32)> = HeapHash::new();
        heap.push(2., 1, (1, 1));
        heap.push(1., 2, (1, 1));
        heap.push(3., 3, (1, 1));
        heap.push(4., 4, (1, 1));
        heap.push(-1., 5, (1, 1));
        heap.push(9., 6, (1, 1));
        assert_eq!(-1., heap.pop().unwrap().0);
        assert_eq!(1., heap.pop().unwrap().0);
        assert_eq!(2., heap.pop().unwrap().0);
        assert_eq!(3., heap.pop().unwrap().0);
        assert_eq!(4., heap.pop().unwrap().0);
        assert_eq!(9., heap.pop().unwrap().0);
        assert_eq!(None, heap.pop());
    }
    #[test]
    fn push_and_pop_100_times() {
        let mut heap: HeapHash<i32, i32, i32> = HeapHash::new();
        for _ in 0..1 {
        for j in 0..100 {
            heap.push(j * 129 % 100, 125 * j % 100, j);
        }

        for j in 0..100 {
            heap.push(j * 129 % 100, 125 * j % 100, j);
        }
        for _ in 0..100 {
            heap.pop();
        }
        for j in 0..100 {
            heap.push(j * 129 % 100, 125 * j % 100, j);
        }

        for j in 0..100 {
            heap.push(j * 129 % 100, 125 * j % 100, j);
        }
        for _ in 0..100 {
            heap.pop();
        }
    }
    }
    #[test]
    fn push_father_swap() {
        let mut heap: HeapHash<i32, i32, i32> = HeapHash::new();
        heap.push(2, 0, 0);
        heap.push(1, 1, 1);
        heap.push(0, 0, 1);
    }
}
