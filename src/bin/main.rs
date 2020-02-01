use a_star::Heap;
extern crate sdl2;
fn main() {
    let mut heap: Heap<i32, i32> = Heap::new();
    assert_eq!(None, heap.pop());
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
    let _sdl = sdl2::init().unwrap();
    println!("Hello, world!");
}
