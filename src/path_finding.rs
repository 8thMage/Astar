use crate::map::Map;
use crate::math::vector::Vec2;
use crate::data_structures::heap_hash::HeapHash;

struct Node {
    father: Option<std::rc::Rc<Node>>,
    position: Vec2<i32>,
    real_distance: i32,
}

fn heuristic(start_point: Vec2<i32>, end_point: Vec2<i32>) -> f32 {
    let diff = start_point - end_point;
    let res = diff.x.abs() + diff.y.abs();
    res as f32
}

// std::collections::BTreeSet

pub fn path_find(map: &Map, start_point: Vec2<i32>, end_point: Vec2<i32>) -> Vec<Vec2<i32>>{

    let mut hash = std::collections::HashMap::new();
    let start: Node = Node {
        position: start_point,
        father: None,
        real_distance: 0,
    };
    let value = heuristic(start_point, end_point);
    let neighbors_delta = vec!(Vec2{x:0,y:1},Vec2{x:1,y:0},Vec2{x:-1,y:0},Vec2{x:0,y:-1});
    let path = 
    {
        let mut heap: HeapHash<f32, Vec2<i32>, Node> = HeapHash::new();
        heap.push(value, start_point, start);
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
                let h = heuristic(new_position, end_point);
                let value = new_real_distance as f32 + h;
                let new_node = Node{
                    position:new_position,
                    real_distance:new_real_distance,
                    father: Some((&boxed).clone()),
                };
                if new_position == end_point {
                    result = Some(new_node);
                    break 'whileHeapNotEmpty;
                }
                if let Some(old_dist) = hash.get(&new_position) {
                    if new_real_distance < *old_dist {
                        heap.push(value, new_position, new_node);
                    }
                } else if let Some((old_value, _old_node)) = heap.get(&new_position) {
                    if value < *old_value {
                        heap.push(value, new_position, new_node);
                    }
                } else {
                    heap.push(value, new_position, new_node);
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
