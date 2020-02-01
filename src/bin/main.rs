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
    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context
        .video()
        .unwrap()
        .window("astar", 100, 100)
        .resizable()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main:loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                // ...
                _ => {}
            }
        }
    }
    println!("Hello, world!");
}
