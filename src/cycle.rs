type Turn = u32;

#[derive(Debug, Clone)]
pub struct Cycle {
    values: Vec<usize>,
    current: usize,
    is_reversed: bool,
    turn: Turn,
}

impl Iterator for Cycle {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // update current, depending on direction
        let c = self.current as isize;
        let next = match self.is_reversed {
            true => c - 1,
            false => c + 1,
        };
        let n_values = self.values.len() as isize;
        let current = next.rem_euclid(n_values); // calculate the least non-negative remainder
        assert!(current >= 0);
        self.current = current as usize;

        // select item
        let item = *self.values.get(self.current).expect("out-of-bounds index");

        // update turn
        self.turn += 1;

        // return item
        Some(item)
    }
}

impl Cycle {
    /// Create generator that cycles over values from range 0..`n`.
    pub fn new(n_values: usize) -> Self {
        let values = (0..n_values).collect();
        // initialized with forward direction and start position so that next value will
        // be the first value
        Self {
            values,
            current: n_values - 1,
            is_reversed: false,
            turn: 0,
        }
    }

    /// Reverse cycle.
    pub fn reverse(&mut self) {
        // if we reverse in the first turn, we need to change the starting position
        // to the first value, so that the next value will be the last value
        if self.turn == 0 {
            assert!(!self.is_reversed);
            self.current = 0;
        }

        // reverse direction, if reversed already, reverse back
        self.is_reversed = match self.is_reversed {
            true => false,
            false => true,
        }
    }

    /// Get turn number.
    pub fn turn(&self) -> Turn {
        self.turn
    }
}

#[cfg(test)]
mod tests {
    use super::*; // bring private functions into scope

    #[test]
    fn test_cycle_iter_values_next_reverse_next_3() {
        let mut cycle = Cycle::new(3);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 2);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
    }

    #[test]
    fn test_cycle_iter_values_next_reverse_next_5() {
        let mut cycle = Cycle::new(5);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 4);
        assert_eq!(cycle.next().unwrap(), 3);
        assert_eq!(cycle.next().unwrap(), 2);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
    }

    #[test]
    fn test_cycle_iter_values_reverse_next() {
        let mut cycle = Cycle::new(3);
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 2);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
    }

    #[test]
    fn test_cycle_iter_values_next_reverse_reverse_next() {
        let mut cycle = Cycle::new(3);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        cycle.reverse();
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
    }

    #[test]
    fn test_cycle_iter_values_next() {
        let mut cycle = Cycle::new(3);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
    }
}
