use std::time::Duration;

pub struct PartResult {
    pub result: String,
    pub duration: Duration,
}

pub struct Results {
    #[allow(dead_code)]
    part1: Option<String>,
    #[allow(dead_code)]
    part2: Option<String>,
}

impl Results {
    pub fn from(results: (&Option<PartResult>, &Option<PartResult>)) -> Self {
        Self {
            part1: results.0.as_ref().map(|r| r.result.clone()),
            part2: results.1.as_ref().map(|r| r.result.clone()),
        }
    }
}

pub struct BenchResults {
    pub parsing: Duration,
    pub part1: Option<Duration>,
    pub part2: Option<Duration>,
    pub total: Duration,
}
