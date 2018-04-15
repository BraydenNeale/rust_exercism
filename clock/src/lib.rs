// use std::fmt;

// pub struct Clock {
// 	hours: i32,
// 	mins: i32,
// }

// impl Clock {
// 	pub fn new(hours: i32, mins: i32) -> Clock {
// 		Clock {
// 			hours,
// 			mins,
// 		}
//     }

// 	pub fn add_minutes(&self, mins: i32) -> Clock {
// 		Clock::new(self.hours, self.mins + mins)
// 	}

// 	pub fn normalize(&self) -> Clock {
// 		if self.mins < 0 {
// 			return self.negative_normalize();
// 		}

// 		let mut hours = self.hours.abs();
// 		let mut mins = self.mins.abs();

// 		hours += mins / 60;
// 		mins %= 60;
// 		hours %= 24;

// 		if self.hours < 0 {
// 			hours = 24 - hours;
// 		}

// 		Clock::new(hours, mins)
// 	}

// 	fn negative_normalize(&self) -> Clock {
// 		let mut mins = self.mins.abs();
// 		let mut hours = self.hours;
// 		let mut total = hours * 60 - mins;
// 		let neg = total < 0;

// 		total = total.abs();
// 		hours = (total / 60) % 24;
// 		mins = total % 60;

// 		if neg {
// 			hours += if total % 60 == 0 { 0 } else { 1 };
// 			mins = 60 - mins;
// 			hours = 24 - hours;
// 		}

// 		Clock::new(hours, mins)
// 	}
// }

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     	let norm = self.normalize();
//         write!(f, "{:02}:{:02}", norm.hours, norm.mins)
//     }
// }

// impl fmt::Debug for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Clock {{ hours: {}, mins: {} }}", self.hours, self.mins)
//     }
// }

// impl std::cmp::PartialEq for Clock {
// 	fn eq(&self, other: &Clock) -> bool {
// 		let snorm = self.normalize();
// 		let onorm = other.normalize();
//         snorm.hours == onorm.hours && snorm.mins == onorm.mins
//     }

//     fn ne(&self, other: &Clock) -> bool {
//     	!self.eq(other)
//     }
// }

// this guys code is insane
use std::fmt::{Display, Formatter, Result};
/// `T_MIN` is the total minutes in a day.
static T_MIN: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    min: u32
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.min / 60, self.min % 60)    
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> String {
        format!("{}", clock)
    }
}

impl Clock {
    /// Creates a new `Clock`.
    pub fn new(h: i32, m: i32) -> Clock {
        Clock {
            min: ((((h * 60 + m) % T_MIN) + T_MIN) % T_MIN) as u32
        }
    }

    /// Returns a new Clock with `minutes` added.
    pub fn add_minutes(&self, minutes: i32) -> Clock {
        Clock::new(0, self.min as i32 + minutes)
    }
}