use std::fmt::{Display, Formatter};
use chrono::{DateTime, Local, Duration, Timelike};
use crate::app::app_timer::Status::{Finished, Ready, Relaxing, Working};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Status{
    Ready,
    Working,
    Finished,
    Relaxing
}

pub struct Timer {
    // seconds
    working_time: u32,
    rest_time: u32,
    pub time: u32,
    status: Status,
    countdown: bool,
    pub text: String,

    started_at: DateTime<Local>,
    finished_at: DateTime<Local>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            working_time: 0,
            rest_time: 0,
            time: 0,
            status: Ready,
            countdown: false,
            text: "".to_string(),

            started_at: Local::now(),
            finished_at: Local::now(),
        }
    }

    pub fn setup(&mut self, worktime: u32, resttime: u32) {
        self.working_time = worktime;
        self.rest_time = resttime;
        self.status = Ready;
    }

    pub fn processing(&mut self) {
        match self.status {
            Ready => {
                self.time = self.working_time;
                self.text = "Ready".to_string();
            }
            Working => {
                self.time =  self.finished_at.time().num_seconds_from_midnight() - Local::now().time().num_seconds_from_midnight();
                self.text = "Working".to_string();
                if Local::now() >= self.finished_at {
                    self.status = Finished;
                }
            }
            Finished => {
                self.time = 0;
                self.text = "Finished".to_string();
            }
            Relaxing => {
                self.time = self.finished_at.time().num_seconds_from_midnight() - Local::now().time().num_seconds_from_midnight();
                self.text = "Relax".to_string();
                if Local::now() >= self.finished_at {
                    self.status = Ready;
                }
            }
        }
    }

    pub fn next(&mut self, worktime: u32, resttime: u32) {
        self.working_time = worktime;
        self.rest_time = resttime;

        match self.status {
            Ready => {
                self.status = Working;
                self.started_at = Local::now();
                self.finished_at = self.started_at + Duration::seconds(self.working_time as i64);
            }
            Working => {
                self.status = Finished;
                self.started_at = Local::now();
                self.finished_at = self.started_at;
            }
            Finished => {
                self.status = Relaxing;
                self.started_at = Local::now();
                self.finished_at = self.started_at + Duration::seconds(self.rest_time as i64);
            }
            Relaxing => {
                self.status = Ready;
                self.started_at = Local::now();
                self.finished_at = self.started_at;
            }
        }
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {:?}, {})", self.working_time, self.rest_time, self.status, self.countdown)
    }
}

#[test]
fn test_timer() {
    let mut timer = Timer::new();
    timer.setup(20 * 60, 5 * 60);

    timer.processing();
    timer.next(20 * 60, 5 * 60);
    timer.processing();
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("After One seconds: timer={}", timer.time);

    timer.next(10, 5);
    timer.processing();
    println!("{}", timer.text);
    assert_eq!(timer.status, Finished);

    timer.next(10, 5);
    timer.processing();
    assert_eq!(timer.status, Relaxing);
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("After One seconds: timer={}", timer.time);
}