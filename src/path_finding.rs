use crate::map::Map;
use crate::math::vector::Vec2;
use crate::data_structures::heap::Heap;
extern crate ahash;
struct Node {
    father: Option<std::rc::Rc<Node>>,
    position: Vec2<i32>,
    real_distance: i32,
}

fn heuristic(start_point: Vec2<i32>, mid_point: Vec2<i32>, end_point: Vec2<i32>) -> f32 {
    let diff = mid_point - end_point;
    let line_diff = end_point - start_point;

    let l1 = diff.x.abs() + diff.y.abs();
    let cross = (mid_point.x * line_diff.y - mid_point.y * line_diff.x).abs();
    l1 as f32 + 0.001 * cross as f32
}

// std::collections::BTreeSet

pub fn path_find(map: &Map, start_point: Vec2<i32>, end_point: Vec2<i32>) -> Vec<Vec2<i32>>{

    let mut hash : ahash::AHashMap<Vec2<i32>, i32> = ahash::AHashMap::new();
    let start: Node = Node {
        position: start_point,
        father: None,
        real_distance: 0,
    };
    let value = heuristic(start_point, start_point, end_point);
    let neighbors_delta = vec!(Vec2{x:0,y:1},Vec2{x:1,y:0},Vec2{x:-1,y:0},Vec2{x:0,y:-1});
    let path = 
    {
        let mut heap: Heap<f32, Node> = Heap::new();
        heap.push(value, start);
        let mut result = None;
        'whileHeapNotEmpty : while let Some(popped) = heap.pop() {
            let node = popped.1;
            let position = node.position;
            let real_distance = node.real_distance;
            if let Some(&hash_pos) = hash.get(&position) {
                if  real_distance < hash_pos {
                    continue;
                }
            }
            let boxed = std::rc::Rc::new(node);
            for &delta in &neighbors_delta {
                let new_position = position + delta;
                if new_position.x >= map.width || new_position.y >= map.height || new_position.x < 0 || new_position.y < 0 {
                    continue;
                } 
                if *map.value((new_position.x, new_position.y)) != 3 {
                    continue;
                }
                let new_real_distance = real_distance + 1;
                let h = heuristic(start_point, new_position, end_point);
                let curr_value = h + (new_real_distance as f32);
                let new_node = Node{
                    position:new_position,
                    real_distance:new_real_distance,
                    father: Some((&boxed).clone()),
                };
                if new_position == end_point {
                    result = Some(new_node);
                    break 'whileHeapNotEmpty;
                }
                if let Some(&old_dist) = hash.get(&new_position) {
                    if new_real_distance < old_dist {
                        heap.push(curr_value, new_node);
                    }
                } else {
                    heap.push(curr_value, new_node);
                }
            }
            
            if let Some(hash_pos) = hash.get_mut(&position) {
                *hash_pos = real_distance.max(*hash_pos);
            } else {
                hash.insert(position, real_distance);
            }
        }
        result
    };
    
    let mut result_vector = vec!();
    let mut iter_path = path;
    while let Some(p) = iter_path {
        result_vector.insert(0, p.position);
        if p.father.is_none() {
            break;
        } else {
            let option = std::rc::Rc::try_unwrap(p.father.unwrap());
            iter_path = option.ok();
        }
    }
    result_vector
}
