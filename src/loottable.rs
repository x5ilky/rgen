// ported from https://gist.github.com/x5ilky/c8d851257e6c0c73fc781b14ab683dcb

use rand::Rng;

pub struct LootTable<T> {
    table: Vec<(T, u32)>
}
impl<T> LootTable<T> {
    pub fn new(table: Vec<(T, u32)>) -> Self {
        Self {
            table
        }
    }

    pub fn push(&mut self, v: T, w: u32) {
        self.table.push((v, w));
    }

    pub fn total(&self) -> u32 {
        let mut j = 0;
        for (_, count) in &self.table {
            j += count;
        }
        j
    }
    pub fn random(&self) -> &T {
        let r = rand::thread_rng().gen_range(0..self.total() as u32);
        let mut counter = 0;
        for (item, count) in &self.table {
            counter += count;
            if counter > r {
                return item;
            }
        }      
        unreachable!()
    }
}