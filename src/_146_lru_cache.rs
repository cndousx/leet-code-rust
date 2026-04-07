use std::{cell::RefCell, collections::HashMap, rc::Rc};

///
/// [146. LRU缓存](https://leetcode.cn/problems/lru-cache/description/)
///
///
/// 思路：
///
/// 1. 双链表实现LRU缓存
/// 2. 使用HashMap存储数据，方便查找
/// 3. 双链表[`head`]节点为最近使用的节点，双链表[`tail`]节点为最久未使用的节点
/// 4. 当缓存达到阈值时，删除尾节点，然后插入新节点(头插法)
/// 5. 当获取节点时，将节点移动到头节点
struct LRUCache {
    /// 容量
    capacity: usize,
    /// caches大小
    size: usize,
    /// 缓存
    cachs: HashMap<i32, Rc<RefCell<Node>>>,
    /// 双链表头节点
    head: Rc<RefCell<Node>>,
    /// 双链表尾节点
    tail: Rc<RefCell<Node>>,
}

/// 链表节点
struct Node {
    key: i32,
    value: i32,
    // 前一个节点
    prev: Option<Rc<RefCell<Node>>>,
    // 后一个节点
    next: Option<Rc<RefCell<Node>>>,
}
impl Node {
    ///
    /// 创建哨兵节点
    ///
    /// 这里的key和value统一使用[`i32::MIN`]
    ///
    fn sentry() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key: i32::MIN,
            value: i32::MIN,
            prev: None,
            next: None,
        }))
    }
    ///
    /// 创建节点
    ///
    fn new(key: i32, value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Node::sentry();
        let tail = Node::sentry();
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());
        LRUCache {
            capacity: capacity as usize,
            size: 0,
            cachs: HashMap::new(),
            head: head,
            tail: tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.cachs.get(&key) {
            Some(node) => {
                // 1.将当前节点移动到头节点
                // 2.返回数据
                // head <---> A <---> node <---> C <---> tail
                // head <---> node <---> A <---> C <---> tail

                let mut node_borrow_mut = node.borrow_mut();
                let node_prev = node_borrow_mut.prev.take();
                let node_next = node_borrow_mut.next.take();
                // 重新连接相邻节点
                if let Some(prev) = &node_prev {
                    prev.borrow_mut().next = node_next.clone();
                }
                if let Some(next) = &node_next {
                    next.borrow_mut().prev = node_prev.clone();
                }

                let head_next = self.head.borrow().next.clone();
                // 设置双向连接
                node_borrow_mut.prev = Some(self.head.clone());
                node_borrow_mut.next = head_next.clone();

                if let Some(head_next_node) = &head_next {
                    head_next_node.borrow_mut().prev = Some(node.clone());
                }
                self.head.borrow_mut().next = Some(node.clone());

                node_borrow_mut.value
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // 如果key已经存在，更新值并移到头部
        if self.cachs.contains_key(&key) {
            let existing_node = self.cachs.get(&key).unwrap().clone();
            existing_node.borrow_mut().value = value;

            // 移动到头部
            self.remove_node(existing_node.clone());
            self.add_to_head(existing_node.clone());
            return;
        }

        // 如果缓存已满，删除尾节点
        if self.size >= self.capacity {
            let tail_prev = self.tail.borrow().prev.as_ref().unwrap().clone();
            let removed_key = tail_prev.borrow().key;

            // 从链表中移除
            let prev_node = tail_prev.borrow_mut().prev.take();
            if let Some(prev_node_rc) = &prev_node {
                prev_node_rc.borrow_mut().next = Some(self.tail.clone());
            }
            self.tail.borrow_mut().prev = prev_node;

            // 从哈希表中移除
            self.cachs.remove(&removed_key);
            self.size -= 1;
        }

        // 插入新节点
        let new_node = Node::new(key, value);
        self.add_to_head(new_node.clone());
        self.cachs.insert(key, new_node);
        self.size += 1;
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node>>) {
        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();

        if let Some(prev_node) = &prev {
            prev_node.borrow_mut().next = next.clone();
        }
        if let Some(next_node) = &next {
            next_node.borrow_mut().prev = prev.clone();
        }
    }

    fn add_to_head(&mut self, node: Rc<RefCell<Node>>) {
        let first = self.head.borrow().next.clone();

        node.borrow_mut().prev = Some(self.head.clone());
        node.borrow_mut().next = first.clone();

        if let Some(first_node) = &first {
            first_node.borrow_mut().prev = Some(node.clone());
        }

        self.head.borrow_mut().next = Some(node);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        let mut lru: LRUCache = LRUCache::new(2);
        lru.put(1, 1); // 缓存是 {1=1}
        lru.put(2, 2); // 缓存是 {1=1, 2=2}
        assert_eq!(lru.get(1), 1); // 返回 1
        lru.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
        assert_eq!(lru.get(2), -1); // 返回 -1 (未找到)
        lru.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
        assert_eq!(lru.get(1), -1); // 返回 -1 (未找到)
        assert_eq!(lru.get(3), 3); // 返回 3
        assert_eq!(lru.get(4), 4); // 返回 4
    }
}
