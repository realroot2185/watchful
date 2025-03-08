use core::cell::RefCell;
use core::ops::Add;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Ticker};

pub struct Clock {
    time: Mutex<ThreadModeRawMutex, RefCell<time::PrimitiveDateTime>>,
    //alarm: (u8, u8),
    //alarm_on: bool,
}

impl Clock {
    pub const fn new() -> Self {
        Self {
            time: Mutex::new(RefCell::new(time::PrimitiveDateTime::MIN)),
            /*alarm: (7, 30),
            alarm_on: false,*/
        }
    }

    pub fn set(&self, time: time::PrimitiveDateTime) {
        self.time.lock(|f| *f.borrow_mut() = time)
    }

    pub fn get(&self) -> time::PrimitiveDateTime {
        self.time.lock(|f| f.borrow().clone())
    }
/*
	pub fn set_alarm(&mut self, alarm: (u8, u8)) {
        self.alarm = alarm;
    }

    pub fn get_alarm(&self) -> (u8, u8) {
        self.alarm
    }

    pub fn set_alarm_on(&mut self, on: bool) {
        self.alarm_on = on;
    }

    pub fn get_alarm_on(&self) -> bool {
        self.alarm_on
    }*/
    
    fn add(&self, duration: time::Duration) {
        self.time.lock(|f| {
            let mut val = f.borrow_mut();
            *val = val.add(duration);
        })
    }
}

#[embassy_executor::task]
pub async fn clock(clock: &'static Clock) {
    const TICK: Duration = Duration::from_secs(1);
    let mut ticker = Ticker::every(TICK);
    loop {
        ticker.next().await;
        clock.add(time::Duration::seconds(1));
    }
}
