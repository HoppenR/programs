//! Functions and structs for navigating through `UniInfo`
//!
//! This module contains functions for basic navigation around the `UniInfo`
//! structures indented to be used both for representing a graphical cursor on
//! screen as well as cursors created when iterating through the data.
//!
//! The less literal, "iterator" cursors can be used to compare with the main cursor
//! to see if the current object is the currently targeted one in an iteration.
//! This is why `Cursor` implements `Default` and `PartialEq`, as well as
//! setting indexes back to 0 after backing up one level.
//!
//! # Usage
//!
//! An example of the usage is:
//!
//! ```
//! fn my_func(uni: &UniInfo, course: &Course) {
//!     for (ix, sem) in uni.menu.iter().enumerate() {
//!         let cursor = Cursor {
//!             semester_ix: ix,
//!             level: CursorLevel::Semester,
//!             ..Default::default()
//!         };
//!         if uni.cursor == cursor {
//!             println!("This semester is currently targeted!");
//!         }
//!     }
//! }
//! ```

use std::cmp;

#[derive(Default, PartialEq, Eq)]
pub(crate) enum Level {
    #[default]
    Semester,
    Period,
    Course,
    Moment,
    Task,
}

/// A struct containing information regarding which indexes it points at/via,
/// and which level it currently is on.
#[derive(Default, PartialEq, Eq)]
pub(super) struct Cursor {
    pub(super) semester: usize,
    pub(super) period: usize,
    pub(super) course: usize,
    pub(super) moment: usize,
    pub(super) task: usize,
    pub(super) level: Level,
}

impl Cursor {
    /// Indents the cursor.
    pub(super) fn enter(&mut self) {
        match self.level {
            Level::Semester => self.level = Level::Period,
            Level::Period => self.level = Level::Course,
            Level::Course => self.level = Level::Moment,
            Level::Moment => self.level = Level::Task,
            Level::Task => {}
        }
    }

    /// Unindents the cursor and sets the old level to 0 so that it can more
    /// easily be compared to other cursors with those fields left uninitialized.
    pub(super) fn exit(&mut self) {
        match self.level {
            Level::Semester => {
                self.semester = 0;
            }
            Level::Period => {
                self.level = Level::Semester;
                self.period = 0;
            }
            Level::Course => {
                self.level = Level::Period;
                self.course = 0;
            }
            Level::Moment => {
                self.level = Level::Course;
                self.moment = 0;
            }
            Level::Task => {
                self.level = Level::Moment;
                self.task = 0;
            }
        }
    }

    /// Moves the cursor upward, unless already on the first object.
    pub(super) fn up(&mut self) {
        match self.level {
            Level::Semester => self.semester = self.semester.saturating_sub(1),
            Level::Period => self.period = self.period.saturating_sub(1),
            Level::Course => self.course = self.course.saturating_sub(1),
            Level::Moment => self.moment = self.moment.saturating_sub(1),
            Level::Task => self.task = self.task.saturating_sub(1),
        }
    }

    /// Moves the cursor downward, but prevents the cursor from reaching the `max` value.
    pub(super) fn down(&mut self, max: usize) {
        match self.level {
            Level::Semester => self.semester = cmp::min(self.semester + 1, max - 1),
            Level::Period => self.period = cmp::min(self.period + 1, max - 1),
            Level::Course => self.course = cmp::min(self.course + 1, max - 1),
            Level::Moment => self.moment = cmp::min(self.moment + 1, max - 1),
            Level::Task => self.task = cmp::min(self.task + 1, max - 1),
        }
    }
}
