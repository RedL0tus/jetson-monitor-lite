use log::debug;

use systemstat::{Platform, System};

use std::default::Default;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

macro_rules! run_command {
    ($command:expr, $($arg:expr), +) => {
        {
            let mut output = Command::new($command)
                $(
                    .arg($arg)
                )*
                .output()
                .expect("Unable to execute command.")
                .stdout;
            output.pop();
            output
        }
    };
}

#[derive(Clone, Default)]
pub struct LoadStorage {
    pub capacity: usize,
    pub inner: Vec<f32>,
}

pub struct InfoBundle {
    system: System,
    pub hostname: String,
    pub temperature: String,
    pub fan_level: String,
    pub loadavg: String,
    pub cpu_load: LoadStorage,
}

impl LoadStorage {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            inner: vec![0.0; capacity],
        }
    }

    pub fn push(&mut self, value: f32) {
        self.inner.drain(0..1);
        self.inner.push(value);
    }
}

impl InfoBundle {
    pub fn new() -> Self {
        let system = System::new();
        Self {
            system: system,
            hostname: String::from_utf8(run_command!("uname", "-n")).unwrap(),
            temperature: "0".to_string(),
            fan_level: "0.0°C".to_string(),
            loadavg: "0.0".to_string(),
            cpu_load: LoadStorage::new(17),
        }
    }

    pub fn update(&mut self) {
        self.temperature = format!("{:.1}°C", self.system.cpu_temp().unwrap());
        self.fan_level =
            String::from_utf8(run_command!("cat", "/sys/devices/pwm-fan/target_pwm")).unwrap();
        self.loadavg = self.system.load_average().unwrap().five.to_string();
        let measurement = self.system.cpu_load_aggregate().unwrap();
        sleep(Duration::from_secs(1));
        let num = 1.0 - measurement.done().unwrap().idle;
        debug!("Current CPU load: {}", num);
        self.cpu_load.push(num);
    }
}
