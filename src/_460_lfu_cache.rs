use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

///
/// [460. LFU 缓存](https://leetcode.cn/problems/lfu-cache/)
///
struct LFUCache {
    /// 容量
    capacity: usize,
    /// 当前容量
    size: usize,
    /// key 数据
    cachs: HashMap<i32, Rc<RefCell<Node>>>,
    /// 当前最小频次
    min_frequency: usize,
    // 频次桶
    frequency_buckets: HashMap<usize, Rc<RefCell<FrequencyBucket>>>,
}
struct Node {
    key: i32,
    value: i32,
    /// 访问频次
    frequency: usize,
}
struct FrequencyBucket {
    buckets: Vec<Rc<RefCell<Node>>>,
}
impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key: key,
            value: value,
            frequency: 0,
        }))
    }
}
impl FrequencyBucket {
    fn new() -> Rc<RefCell<FrequencyBucket>> {
        Rc::new(RefCell::new(FrequencyBucket {
            buckets: Vec::new(),
        }))
    }
    fn len(&self) -> usize {
        self.buckets.len()
    }

    fn push(&mut self, node: Rc<RefCell<Node>>) {
        self.buckets.push(node);
    }
    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        // 题目中说明key是唯一的
        self.buckets.retain(|e| e.borrow().key != node.borrow().key);
    }
    fn remove_first(&mut self) -> Option<Rc<RefCell<Node>>> {
        // 题目中说明key是唯一的
        if self.buckets.len() == 0 {
            return None;
        }
        Some(self.buckets.remove(0))
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            size: 0,
            cachs: HashMap::new(),
            min_frequency: 0,
            frequency_buckets: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.cachs.get(&key) {
            None => -1,
            Some(node) => {
                let old_freq = node.borrow_mut().frequency;
                let new_freq = old_freq + 1;
                match self.frequency_buckets.get_mut(&old_freq) {
                    Some(bucket) => {
                        bucket.borrow_mut().remove(node.clone());
                        // if bucket.borrow().len() == 0 {
                        //     // 这里不用移除vec，更新min_frequency即可
                        // }
                    }
                    None => {}
                }
                let buk = self
                    .frequency_buckets
                    .entry(new_freq)
                    .or_insert(FrequencyBucket::new());
                buk.borrow_mut().push(node.clone());
                // 更新访问频次
                node.borrow_mut().frequency = new_freq;
                let v = node.borrow().value;
                self.update_min_frequency();
                // 返回数据
                v
            }
        }
    }
    fn update_min_frequency(&mut self) {
        self.min_frequency = *(self
            .frequency_buckets
            .iter()
            .filter(|(a, b)| b.borrow().len() > 0)
            .map(|(a, b)| a)
            .min()
            .unwrap_or(&1));
    }
    fn min_frequency(&self) -> usize {
        *(self
            .frequency_buckets
            .iter()
            .filter(|(a, b)| b.borrow().len() > 0)
            .map(|(a, b)| a)
            .min()
            .unwrap_or(&1))
    }
    fn put(&mut self, key: i32, value: i32) {
        if self.cachs.contains_key(&key) {
            // 存在则更新数据
            let node = self.cachs.get(&key).unwrap().clone();
            node.borrow_mut().value = value;
            let old_freq = node.borrow().frequency;
            let new_freq = old_freq + 1;
            match self.frequency_buckets.get_mut(&self.min_frequency) {
                Some(buk) => {
                    buk.borrow_mut().remove(node.clone());
                }
                None => {}
            }

            let buk = self
                .frequency_buckets
                .entry(new_freq)
                .or_insert(FrequencyBucket::new());
            buk.borrow_mut().push(node.clone());
            self.update_min_frequency();
            return;
        }

        let new_node = Node::new(key, value);
        let new_freq = 1;
        new_node.borrow_mut().frequency = new_freq;

        if self.size >= self.capacity {
            // 移除频次最小的
            match self.frequency_buckets.get_mut(&self.min_frequency) {
                Some(buk) => {
                    let rm = buk.borrow_mut().remove_first();
                    match rm {
                        Some(s) => {
                            let rmk = s.borrow().key;
                            self.cachs.remove(&rmk);
                        }
                        _ => {}
                    }
                }
                None => {}
            }

            self.size -= 1;
        }

        let buk = self
            .frequency_buckets
            .entry(new_freq)
            .or_insert(FrequencyBucket::new());
        buk.borrow_mut().push(new_node.clone());

        self.cachs.insert(key, new_node.clone());
        // 这个节点是新加入的，访问次数为1，一定是最小的
        self.min_frequency = new_freq;

        self.size += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        // cnt(x) = 键 x 的使用计数
        // cache=[] 将显示最后一次使用的顺序（最左边的元素是最近的）
        let mut lfu = LFUCache::new(2);

        lfu.put(1, 1); // cache=[1,_], cnt(1)=1
        lfu.put(2, 2); // cache=[2,1], cnt(2)=1, cnt(1)=1
        lfu.get(1); // 返回 1
        // cache=[1,2], cnt(2)=1, cnt(1)=2

        lfu.put(3, 3); // 去除键 2 ，因为 cnt(2)=1 ，使用计数最小
        // cache=[3,1], cnt(3)=1, cnt(1)=2

        lfu.get(2); // 返回 -1（未找到）
        lfu.get(3); // 返回 3
        // cache=[3,1], cnt(3)=2, cnt(1)=2

        lfu.put(4, 4); // 去除键 1 ，1 和 3 的 cnt 相同，但 1 最久未使用
        // cache=[4,3], cnt(4)=1, cnt(3)=2

        lfu.get(1); // 返回 -1（未找到）
        lfu.get(3); // 返回 3
        // cache=[3,4], cnt(4)=1, cnt(3)=3

        lfu.get(4); // 返回 4
        // cache=[3,4], cnt(4)=2, cnt(3)=3
    }
}
