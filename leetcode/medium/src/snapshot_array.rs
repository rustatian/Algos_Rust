struct SnapshotArray {
    index_changes: Vec<Vec<[i32; 2]>>,
    curr_snap: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut v: Vec<Vec<[i32; 2]>> = vec![vec![[0, 0]]; length as usize];

        Self {
            index_changes: v,
            curr_snap: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.index_changes[index as usize].push([self.curr_snap, val]);
    }

    fn snap(&mut self) -> i32 {
        self.curr_snap += 1;
        self.curr_snap - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let val = &self.index_changes[index as usize];
        let res = val.binary_search(&[snap_id, i32::MAX]);

        match res {
            Ok(idx) => self.index_changes[snap_id as usize][idx][1],

            Err(idx) => self.index_changes[snap_id as usize][idx - 1][1],
        }
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
        sa.set(1, 13);
        assert_eq!(0, sa.snap());
        assert_eq!(13, sa.get(0, 0));
        assert_eq!(1, sa.snap());
    }
}
