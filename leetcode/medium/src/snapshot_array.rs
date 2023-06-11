use std::collections::HashMap;

#[derive(Clone)]
struct changes {
    idx: i32,
    elem: i32,
}

struct SnapshotArray {
    array: Vec<i32>,
    changes: Vec<changes>,
    idxs: HashMap<usize, Vec<changes>>,
    curr_snap: usize,
    prev_snap: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut arr = vec![0; length as usize];
        let mut changes = Vec::new();
        let mut hm = HashMap::new();
        Self {
            changes: changes,
            array: arr,
            idxs: hm,
            curr_snap: 0,
            prev_snap: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.array[index as usize] = val;

        let change = changes {
            idx: index,
            elem: val,
        };

        self.changes.push(change);
    }

    fn snap(&mut self) -> i32 {
        self.idxs.insert(self.curr_snap, self.changes.clone());
        self.changes.clear();


        self.prev_snap = self.curr_snap;
        self.curr_snap += 1;

        self.prev_snap as i32
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        if self.idxs.is_empty() {
            return self.array[index as usize];
        }

        let changes = self.idxs.get(&(snap_id as usize)).unwrap();
        for change in changes {
            if change.idx == index {
                return change.elem;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::SnapshotArray;

    #[test]
    fn test() {
        let mut sa = SnapshotArray::new(3);
        sa.set(0, 4);
        sa.set(0, 16);
        sa.set(0, 13);
        assert_eq!(0, sa.snap());
        assert_eq!(13, sa.get(0, 0));
        assert_eq!(1, sa.snap());
    }
}
