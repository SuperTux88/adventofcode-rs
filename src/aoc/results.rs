use std::time::Duration;

pub struct Results {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

pub struct BenchResults {
    pub parsing: Duration,
    pub part1: Option<Duration>,
    pub part2: Option<Duration>,
    pub total: Duration,
}
