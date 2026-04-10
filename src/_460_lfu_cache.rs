use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

///
/// [460. LFU 缓存](https://leetcode.cn/problems/lfu-cache/)
///
/// LFU 的核心思想
/// - 基于访问频率：LFU 会为每个缓存项维护一个访问计数器（frequency），每次访问（get 或 put）该项时，计数器就 +1。
/// - 淘汰规则：当缓存已满，需要插入新数据时，优先移除访问频率最低（使用次数最少）的项。
/// - 频率相同时的处理：如果多个项的频率相同，通常会进一步结合 LRU（Least Recently Used，最近最少使用） 策略，
/// 即在同频率的项中，移除最久未被访问的那一个（或最早进入的）。
struct LFUCache {
    capacity: i32,
    curr: i32,
    data: BTreeMap<i32, LList>, // count: list
    memo: HashMap<i32, LPtr>,   // key: node
}

type LPtr = Rc<RefCell<LNode>>;

struct LNode {
    key: i32,
    value: i32,
    count: i32,
    prev: Option<LPtr>,
    next: Option<LPtr>,
}

impl LNode {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            count: 1,
            prev: None,
            next: None,
        }
    }
}

struct LList {
    // root最旧, root -> prev最新;
    root: Option<LPtr>,
}

impl LList {
    fn new() -> Self {
        Self { root: None }
    }

    fn new_with_value(key: i32, value: i32) -> Self {
        let node = LNode::new(key, value);
        let root = Rc::new(RefCell::new(node));
        {
            let mut m = root.borrow_mut();
            m.prev = Some(Rc::clone(&root));
            m.next = Some(Rc::clone(&root));
        }
        Self { root: Some(root) }
    }

    fn new_with_node(node: LPtr) -> Self {
        {
            let mut m = node.borrow_mut();
            m.prev = Some(Rc::clone(&node));
            m.next = Some(Rc::clone(&node));
        }
        Self { root: Some(node) }
    }

    fn ptr_eq(left: &LPtr, right: &LPtr) -> bool {
        Rc::<RefCell<LNode>>::ptr_eq(left, right)
    }

    fn move_out(&mut self, node: LPtr) -> bool {
        // 将node从self中移除; 需要确保node在自身内部;
        if Self::ptr_eq(&node, node.borrow().prev.as_ref().unwrap()) {
            // 唯一节点;
            {
                let mut m = node.borrow_mut();
                m.next.take();
                m.prev.take();
            }
            // 删除root节点;
            self.root.take();
            true
        } else {
            // 不唯一的节点, 需要执行移动;
            let (prev, next) = {
                let mut m = node.borrow_mut();
                (m.prev.take().unwrap(), m.next.take().unwrap())
            };
            prev.borrow_mut().next = Some(Rc::clone(&next));
            next.borrow_mut().prev = Some(prev);
            // 检查是否与root相等; 相等则更新root为下一个节点;
            let root_ref = self.root.as_ref().unwrap();
            if Self::ptr_eq(&node, root_ref) {
                self.root.replace(next);
            }
            false
        }
    }

    fn move_in(&mut self, node: LPtr) {
        // 将node移入self; 需要确保node不在自身内;
        if self.root.is_none() {
            let prev = Rc::clone(&node);
            let next = Rc::clone(&node);
            {
                let mut m = node.borrow_mut();
                m.prev = Some(prev);
                m.next = Some(next);
            }
            self.root = Some(node);
        } else {
            // node最新, 接到root -> prev;
            let root = Rc::clone(self.root.as_ref().unwrap());
            let last = Rc::clone(root.borrow().prev.as_ref().unwrap());
            last.borrow_mut().next = Some(Rc::clone(&node));
            root.borrow_mut().prev = Some(Rc::clone(&node));
            let mut m = node.borrow_mut();
            m.prev = Some(last);
            m.next = Some(root);
        }
    }

    fn pop(&mut self) -> LPtr {
        // 取出最旧的一个节点;
        let node = Rc::clone(self.root.as_ref().unwrap());
        self.move_out(Rc::clone(&node));
        node
    }

    fn push(&mut self, key: i32, value: i32) {
        let node = LNode::new(key, value);
        let root = Rc::new(RefCell::new(node));
        self.move_in(root);
    }
}

impl Drop for LList {
    fn drop(&mut self) {
        if let Some(node) = self.root.take() {
            node.borrow_mut().prev.take();
            let mut node_next = node.borrow_mut().next.take();
            while let Some(next) = node_next {
                next.borrow_mut().prev.take();
                node_next = next.borrow_mut().next.take();
            }
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            curr: 0,
            data: BTreeMap::new(),
            memo: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.memo.get(&key) {
            let node = Rc::clone(node);
            let (count, value) = {
                let mut m = node.borrow_mut();
                m.count += 1;
                (m.count, m.value)
            };
            {
                let last = self.data.get_mut(&(count - 1)).unwrap();
                let empty = last.move_out(Rc::clone(&node));
                if empty {
                    self.data.remove(&(count - 1));
                }
            }
            if let Some(mut list) = self.data.get_mut(&count) {
                list.move_in(node);
            } else {
                let list = LList::new_with_node(node);
                self.data.insert(count, list);
            }
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.get(key) >= 0 {
            let node = Rc::clone(self.memo.get(&key).unwrap());
            node.borrow_mut().value = value;
            return;
        }
        if self.curr < self.capacity {
            // 新增
            self.curr += 1;
            let node = Rc::new(RefCell::new(LNode::new(key, value)));
            if let Some(mut list) = self.data.get_mut(&1) {
                list.move_in(Rc::clone(&node));
            } else {
                let list = LList::new_with_node(Rc::clone(&node));
                self.data.insert(1, list);
            }
            self.memo.insert(key, node);
        } else {
            // 替换
            // 1. 删除旧node
            let (node, empty, count) = {
                let mut entry = self.data.first_entry().unwrap();
                let mut list = entry.get_mut();
                let node = list.pop();
                let empty = list.root.is_none();
                (node, empty, *entry.key())
            };
            if empty {
                self.data.remove(&count);
            }
            let old_key = node.borrow().key;
            self.memo.remove(&old_key);
            // 添加新key;
            {
                let mut m = node.borrow_mut();
                m.count = 1;
                m.key = key;
                m.value = value;
            }
            if let Some(mut list) = self.data.get_mut(&1) {
                list.move_in(Rc::clone(&node));
            } else {
                let list = LList::new_with_node(Rc::clone(&node));
                self.data.insert(1, list);
            }
            self.memo.insert(key, node);
        }
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
