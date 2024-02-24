pub type Turn = u32;

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
        let next = match self.is_reversed {
            true => self.current as i8 - 1,
            false => self.current as i8 + 1,
        };
        let n = self.values.len() as i8;
        let current = next.rem_euclid(n);
        self.current = current as usize;

        // select item
        let item = self.values[self.current];

        // update turn
        self.turn += 1;

        // return item
        Some(item)
    }
}

impl Cycle {
    /// Create generator that cycles over values from range 0..`n`.
    pub fn new(n: usize) -> Self {
        let values = (0..n).collect();
        Self {
            values,
            current: n - 1,
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

        self.is_reversed = match self.is_reversed {
            true => false,
            false => true,
        }
    }

    pub fn get_turn(&self) -> Turn {
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
