use super::vector::Vec2;
pub struct Map {
    pub height: i32,
    pub width: i32,
    pub stride:i32,
    pub values: Vec<u8>,
}

impl Map {
    pub fn dimensions(self) -> Vec2<i32> {
        let res = Vec2{x:self.width, y:self.height};
        res
    }

    pub fn index(map:&Map, v:(i32,i32)) -> usize {
        let res = v.1 * map.stride + v.0;
        res as usize
    }

    pub fn value(&self, v:(i32,i32)) -> &u8 {
        let index = Map::index(self, v);
        let val = self.values.get(index).unwrap();
        val
    }

    pub fn value_mut(&mut self, v:(i32,i32)) -> &mut u8 {
        let index = Map::index(self, v);
        let val = self.values.get_mut(index).unwrap();
        val
    }
}
