pub mod metrics {
    pub struct Metrics {
        pub cyclomatic_complexity: usize,
        pub loop_depth: usize,
        pub arithmetic_operations: usize,
        pub string_operations: usize,
    }

    impl Metrics {
        pub fn new() -> Self {
            Metrics {
                cyclomatic_complexity: 3,
                loop_depth: 1,
                arithmetic_operations: 3,
                string_operations: 1,
            }
        }

        pub fn average(&self) -> usize {
            let total = self.cyclomatic_complexity
                + self.loop_depth
                + self.arithmetic_operations
                + self.string_operations;
            total / 4
        }
    }
}