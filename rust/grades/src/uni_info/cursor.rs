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

#[derive(Clone, Copy, Default, PartialEq)]
pub(crate) enum CursorLevel {
    #[default]
    Semester,
    Period,
    Course,
    Moment,
    Task,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub(super) struct Cursor {
    pub(super) semester_ix: usize,
    pub(super) period_ix: usize,
    pub(super) course_ix: usize,
    pub(super) moment_ix: usize,
    pub(super) task_ix: usize,
    pub(super) level: CursorLevel,
}

impl Cursor {
    /// Indents the cursor.
    pub(super) fn enter(&mut self) {
        match self.level {
            CursorLevel::Semester => self.level = CursorLevel::Period,
            CursorLevel::Period => self.level = CursorLevel::Course,
            CursorLevel::Course => self.level = CursorLevel::Moment,
            CursorLevel::Moment => self.level = CursorLevel::Task,
            CursorLevel::Task => {}
        }
    }

    /// Unindents the cursor and sets the old level to 0 for future comparisons.
    pub(super) fn exit(&mut self) {
        match self.level {
            CursorLevel::Semester => {
                self.semester_ix = 0;
            }
            CursorLevel::Period => {
                self.level = CursorLevel::Semester;
                self.period_ix = 0;
            }
            CursorLevel::Course => {
                self.level = CursorLevel::Period;
                self.course_ix = 0;
            }
            CursorLevel::Moment => {
                self.level = CursorLevel::Course;
                self.moment_ix = 0;
            }
            CursorLevel::Task => {
                self.level = CursorLevel::Moment;
                self.task_ix = 0;
            }
        }
    }

    /// Moves the cursor upward, unless already on the first object.
    pub(super) fn up(&mut self) {
        match self.level {
            CursorLevel::Semester => self.semester_ix = self.semester_ix.saturating_sub(1),
            CursorLevel::Period => self.period_ix = self.period_ix.saturating_sub(1),
            CursorLevel::Course => self.course_ix = self.course_ix.saturating_sub(1),
            CursorLevel::Moment => self.moment_ix = self.moment_ix.saturating_sub(1),
            CursorLevel::Task => self.task_ix = self.task_ix.saturating_sub(1),
        }
    }

    /// Moves the cursor downward, but prevents the cursor from reaching the `max` value.
    pub(super) fn down(&mut self, max: usize) {
        match self.level {
            CursorLevel::Semester => self.semester_ix = cmp::min(self.semester_ix + 1, max - 1),
            CursorLevel::Period => self.period_ix = cmp::min(self.period_ix + 1, max - 1),
            CursorLevel::Course => self.course_ix = cmp::min(self.course_ix + 1, max - 1),
            CursorLevel::Moment => self.moment_ix = cmp::min(self.moment_ix + 1, max - 1),
            CursorLevel::Task => self.task_ix = cmp::min(self.task_ix + 1, max - 1),
        }
    }
}
