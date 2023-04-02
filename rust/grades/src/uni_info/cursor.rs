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
    pub(super) fn enter(&mut self) {
        match self.level {
            CursorLevel::Semester => self.level = CursorLevel::Period,
            CursorLevel::Period => self.level = CursorLevel::Course,
            CursorLevel::Course => self.level = CursorLevel::Moment,
            CursorLevel::Moment => self.level = CursorLevel::Task,
            CursorLevel::Task => unreachable!(),
        }
    }

    pub(super) fn exit(&mut self) {
        match self.level {
            CursorLevel::Semester => unreachable!(),
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

    pub(super) fn decrease(&mut self) {
        match self.level {
            CursorLevel::Semester => self.semester_ix = self.semester_ix.saturating_sub(1),
            CursorLevel::Period => self.period_ix = self.period_ix.saturating_sub(1),
            CursorLevel::Course => self.course_ix = self.course_ix.saturating_sub(1),
            CursorLevel::Moment => self.moment_ix = self.moment_ix.saturating_sub(1),
            CursorLevel::Task => self.task_ix = self.task_ix.saturating_sub(1),
        }
    }

    pub(super) fn increase(&mut self, max: usize) {
        match self.level {
            CursorLevel::Semester => self.semester_ix = cmp::min(self.semester_ix + 1, max - 1),
            CursorLevel::Period => self.period_ix = cmp::min(self.period_ix + 1, max - 1),
            CursorLevel::Course => self.course_ix = cmp::min(self.course_ix + 1, max - 1),
            CursorLevel::Moment => self.moment_ix = cmp::min(self.moment_ix + 1, max - 1),
            CursorLevel::Task => self.task_ix = cmp::min(self.task_ix + 1, max - 1),
        }
    }
}
