use create_type_spec_derive::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes counter base state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug, Default)]
pub struct CounterBase {
    /// counter value
    pub value: u128,
}

impl CounterBase {
    /// ## Description
    /// Increments current value by one
    pub fn increment(&mut self) {
        self.value += 1;
    }

    /// ## Description
    /// Decrements current value by one
    pub fn decrement(&mut self) {
        assert!(self.value > 0, "CounterBase: decrement overflow");
        self.value -= 1;
    }

    /// ## Description
    /// Resets current value to 0
    pub fn reset(&mut self) {
        self.value = 0;
    }

    /// ## Description
    /// Returns current counter value
    pub fn current(&self) -> u128 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_base() {
        let mut counter = CounterBase::default();
        assert_eq!(counter.current(), 0);

        counter.increment();
        assert_eq!(counter.current(), 1);

        counter.decrement();
        assert_eq!(counter.current(), 0);

        counter.increment();
        counter.reset();
        assert_eq!(counter.current(), 0);
    }

    #[test]
    #[should_panic(expected = "CounterBase: decrement overflow")]
    fn test_decrement_zero_value() {
        let mut counter = CounterBase::default();
        counter.decrement()
    }
}
