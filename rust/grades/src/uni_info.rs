mod cursor;
mod print;

use cursor::{Cursor, CursorLevel};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

type Tasks = BTreeMap<String, bool>;

#[derive(Deserialize, Clone)]
struct Moment {
    code: String,
    completed: bool,
    credits: f32,
    description: String,
    tasks: Option<Tasks>,
}

type Moments = BTreeSet<Moment>;

#[derive(Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[serde(untagged)]
enum Grade {
    Completed(bool),
    Grade(usize),
    Ongoing,
}

#[derive(Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Course {
    code: String,
    grade: Grade,
    moments: Moments,
    name: String,
}

type Period = BTreeSet<Course>;

#[derive(Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Semester {
    index: usize,
    periods: [Period; 2],
}

type Menu = BTreeSet<Semester>;

#[derive(Deserialize)]
pub(super) struct UniInfo {
    menu: Menu,
    #[serde(default)]
    cursor: Cursor,
}

impl Eq for Moment {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Moment {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl PartialOrd for Moment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Moment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.code.cmp(&other.code)
    }
}

impl Course {
    fn sum_credits(&self) -> f32 {
        return self
            .moments
            .iter()
            .filter_map(|v| if v.completed { Some(v.credits) } else { None })
            .sum();
    }

    fn max_credits(&self) -> f32 {
        return self.moments.iter().map(|v| v.credits).sum();
    }
}

impl UniInfo {
    pub(super) fn cursor_increase(&mut self) {
        let max_value: usize = match self.cursor.level {
            CursorLevel::Semester => self.sel_menu_entries(),
            CursorLevel::Period => self.sel_semester_entries(),
            CursorLevel::Course => self.sel_period_entries(),
            CursorLevel::Moment => self.sel_course_entries(),
            CursorLevel::Task => self.sel_moment_entries(),
        };
        self.cursor.increase(max_value);
    }

    pub(super) fn cursor_decrease(&mut self) {
        self.cursor.decrease();
    }

    pub(super) fn cursor_enter(&mut self) {
        let num_entries_next_level: usize = match self.cursor.level {
            CursorLevel::Semester => self.sel_semester_entries(),
            CursorLevel::Period => self.sel_period_entries(),
            CursorLevel::Course => match self.sel_course().map(|x| x.grade == Grade::Ongoing) {
                Some(ongoing) if ongoing => self.sel_course_entries(),
                _ => 0,
            },
            CursorLevel::Moment => self.sel_moment_entries(),
            CursorLevel::Task => 0,
        };
        if num_entries_next_level > 0 {
            self.cursor.enter();
        }
    }

    pub(super) fn cursor_exit(&mut self) {
        match self.cursor.level {
            CursorLevel::Semester => {}
            _ => self.cursor.exit(),
        }
    }

    fn sel_menu_entries(&self) -> usize {
        match self.sel_menu() {
            Some(menu) => menu.len(),
            _ => 0,
        }
    }

    fn sel_semester_entries(&self) -> usize {
        match self.sel_semester() {
            Some(semester) => semester.periods.len(),
            _ => 0,
        }
    }

    fn sel_period_entries(&self) -> usize {
        match self.sel_period() {
            Some(period) => period.len(),
            _ => 0,
        }
    }

    fn sel_course_entries(&self) -> usize {
        match self.sel_course() {
            Some(course) => course.moments.len(),
            _ => 0,
        }
    }

    fn sel_moment_entries(&self) -> usize {
        match self.sel_moment().and_then(|x| x.tasks.clone()) {
            Some(tasks) => tasks.len(),
            _ => 0,
        }
    }

    fn sel_menu(&self) -> Option<&Menu> {
        Some(&self.menu)
    }

    fn sel_semester(&self) -> Option<&Semester> {
        self.menu.iter().nth(self.cursor.semester_ix)
    }

    fn sel_period(&self) -> Option<&Period> {
        self.sel_semester()
            .map(|v| &v.periods[self.cursor.period_ix])
    }

    fn sel_course(&self) -> Option<&Course> {
        self.sel_period()
            .and_then(|x| x.iter().nth(self.cursor.course_ix))
    }

    fn sel_moment(&self) -> Option<&Moment> {
        self.sel_course()
            .and_then(|x| x.moments.iter().nth(self.cursor.moment_ix))
    }

    // fn sel_task(&self) -> Option<&Tasks> {
    //     self.sel_moment()
    //         .and_then(|x| x.tasks.iter().nth(self.cursor.task_ix))
    // }
}
