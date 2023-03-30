use serde::Deserialize;
use std::cmp;
use std::mem::discriminant;

#[derive(Deserialize)]
pub enum CursorLevel {
    Semester,
    Period,
    Course,
    Moment,
    Task,
}

#[derive(Deserialize)]
pub struct Cursor {
    pub semester_ix: usize,
    pub period_ix: usize,
    pub course_ix: usize,
    pub moment_ix: usize,
    pub task_ix: usize,
    pub level: CursorLevel,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            semester_ix: 0,
            period_ix: 0,
            course_ix: 0,
            moment_ix: 0,
            task_ix: 0,
            level: CursorLevel::Semester,
        }
    }
}

impl Cursor {
    pub fn enter(self: &mut Self) {
        match self.level {
            CursorLevel::Semester => self.level = CursorLevel::Period,
            CursorLevel::Period => self.level = CursorLevel::Course,
            CursorLevel::Course => self.level = CursorLevel::Moment,
            CursorLevel::Moment => self.level = CursorLevel::Task,
            CursorLevel::Task => self.level = CursorLevel::Task,
        }
    }

    pub fn exit(self: &mut Self) {
        match self.level {
            CursorLevel::Semester => {
                self.level = CursorLevel::Semester;
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

    pub fn decrease(self: &mut Self) {
        match self.level {
            CursorLevel::Semester => self.semester_ix = self.semester_ix.saturating_sub(1),
            CursorLevel::Period => self.period_ix = self.period_ix.saturating_sub(1),
            CursorLevel::Course => self.course_ix = self.course_ix.saturating_sub(1),
            CursorLevel::Moment => self.moment_ix = self.moment_ix.saturating_sub(1),
            CursorLevel::Task => self.task_ix = self.task_ix.saturating_sub(1),
        }
    }

    pub fn increase(self: &mut Self, max: usize) {
        match self.level {
            CursorLevel::Semester => self.semester_ix = cmp::min(self.semester_ix + 1, max),
            CursorLevel::Period => self.period_ix = cmp::min(self.period_ix + 1, max),
            CursorLevel::Course => self.course_ix = cmp::min(self.course_ix + 1, max),
            CursorLevel::Moment => self.moment_ix = cmp::min(self.moment_ix + 1, max),
            CursorLevel::Task => self.task_ix = cmp::min(self.task_ix + 1, max),
        }
    }

    pub fn matches(self: &Self, level: CursorLevel, indexes: &Vec<usize>) -> bool {
        if discriminant(&self.level) == discriminant(&level) {
            return self.equals(indexes);
        }
        return false;
    }

    pub fn equals(self: &Self, indexes: &Vec<usize>) -> bool {
        match indexes[..] {
            [s] => s == self.semester_ix,
            [s, p] => p == self.period_ix && self.equals(&vec![s]),
            [s, p, c] => c == self.course_ix && self.equals(&vec![s, p]),
            [s, p, c, m] => m == self.moment_ix && self.equals(&vec![s, p, c]),
            [s, p, c, m, t] => t == self.task_ix && self.equals(&vec![s, p, c, m]),
            _ => false,
        }
    }
}
