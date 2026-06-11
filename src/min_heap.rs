pub struct Heap<V: Ord> {
    values: Vec<V>,
}

impl<V: Ord> Heap<V> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn add(&mut self, value: V) {
        self.values.push(value);
        if self.values.len() == 1 {
            return;
        }
        self.upheap(self.values.len() - 1);
    }

    fn upheap(&mut self, index: usize) {
        if index == 0 {
            return;
        }
        let parent_index = Self::get_parent_index(index);
        if self.values[parent_index] <= self.values[index] {
            return;
        }
        self.values.swap(index, parent_index);
        self.upheap(parent_index);
    }

    pub fn remove(&mut self) -> Option<V> {
        let len = self.values.len();
        if len == 0 {
            return None;
        }
        self.values.swap(0, len - 1);
        let value = self.values.pop().unwrap();
        self.downheap(0);

        Some(value)
    }

    fn downheap(&mut self, index: usize) {
        let left_child_index = 2 * index + 1;
        let right_child_index = 2 * index + 2;
        if left_child_index >= self.values.len() {
            return;
        }
        let min_child_index = if right_child_index < self.values.len()
            && self.values[left_child_index] > self.values[right_child_index]
        {
            right_child_index
        } else {
            left_child_index
        };

        if self.values[min_child_index] >= self.values[index] {
            return;
        }

        self.values.swap(min_child_index, index);
        self.downheap(min_child_index);
    }

    pub fn peek(&self) -> Option<&V> {
        if self.values.is_empty() {
            return None;
        }

        self.values.get(0)
    }

    fn get_parent_index(index: usize) -> usize {
        if index % 2 == 0 {
            return (index - 2) / 2;
        }

        (index - 1) / 2
    }
}
