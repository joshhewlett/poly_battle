use std::collections::HashMap;
use std::time::{Duration, SystemTime};

pub enum TimeUnit {
    NANO,
    MILLIS,
    SECOND,
}

pub struct BenchmarkUnit {
    name: String,
    time_unit: TimeUnit,
    start: SystemTime,
    elapsed: Option<Duration>,
}

impl BenchmarkUnit {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            time_unit: TimeUnit::MILLIS,
            start: SystemTime::now(),
            elapsed: None,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn end(&mut self) {
        self.elapsed = Some(self.start.elapsed().expect("Failed to get elapsed time for unit"));
    }
}

impl std::fmt::Display for BenchmarkUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let elapsed_str = self.elapsed
            .map(|elapsed| {
                let micros = elapsed.as_micros();
                let millis = micros / 1000;
                let fractional_digits = micros % 1000;

                format!("{}.{}ms", millis, fractional_digits)
            })
            .unwrap_or("Unit still executing...".to_string());

        write!(f, "{}: {}", self.name, elapsed_str)
    }
}

pub struct Benchmark {
    start: SystemTime,
    benchmarks: Vec<BenchmarkUnit>,
    elapsed: Option<Duration>,
}

impl Benchmark {
    pub fn new() -> Self {
        Self {
            start: SystemTime::now(),
            benchmarks: Vec::new(),
            elapsed: None,
        }
    }

    pub fn new_benchmark_unit(&mut self, name: &str) {
        self.benchmarks.push(BenchmarkUnit::new(name));
    }

    pub fn end_benchmark_unit(&mut self, name: &str) -> Result<(), String> {
        let mut unit = self.benchmarks.iter_mut()
            .find(|unit| unit.name == name)
            .ok_or("Benchmark unit not found".to_string())?;

        unit.end();

        Ok(())
    }

    pub fn end(&mut self) -> Option<Duration> {
        self.elapsed = Some(self.start.elapsed().expect("Failed to get elapsed time"));

        self.elapsed
    }
}

impl std::fmt::Display for Benchmark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let elapsed_str = self.elapsed
            .map(|elapsed| {
                let micros = elapsed.as_micros();
                let millis = micros / 1000;
                let fractional_digits = micros % 1000;

                format!("{}.{}ms", millis, fractional_digits)
            })
            .unwrap_or("Benchmark still executing...".to_string());

        let mut output = format!("Total time: {}", elapsed_str);
        self.benchmarks.iter()
            .for_each(|(unit)| output.push_str(format!("\n    {}", unit).as_str()));

        write!(f, "{}", output)
    }
}