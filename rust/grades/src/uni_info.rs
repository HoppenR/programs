mod cursor;
mod print;

use cursor::{Cursor, CursorLevel};
use serde::Deserialize;
use std::borrow::BorrowMut;
use std::cmp;
use std::collections::BTreeMap;

type Tasks = BTreeMap<String, bool>;

#[derive(Deserialize, Clone)]
struct Moment {
    code: String,
    completed: bool,
    credits: f32,
    description: String,
    tasks: Option<Tasks>,
}

type Moments = Vec<Moment>;

#[derive(Deserialize, PartialEq, Clone)]
#[serde(untagged)]
enum Grade {
    Completed(bool),
    Grade(usize),
    Ongoing,
}

#[derive(Deserialize, Clone)]
struct Course {
    code: String,
    grade: Grade,
    moments: Moments,
    name: String,
}

type Period = Vec<Course>;

#[derive(Deserialize, Clone)]
struct Semester {
    index: usize,
    periods: [Period; 2],
}

type Menu = Vec<Semester>;

#[derive(Deserialize)]
pub(super) struct UniInfo {
    menu: Menu,
    #[serde(default)]
    cursor: Cursor,
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

    pub(super) fn edit_selection(&mut self) {
        match self.cursor.level {
            CursorLevel::Semester => {}
            CursorLevel::Period => {}
            CursorLevel::Course => {
                if let Some(course) = self.sel_course_mut() {
                    match course.grade.borrow_mut() {
                        Grade::Completed(completed) => *completed = !*completed,
                        Grade::Grade(grade) => *grade = cmp::max(3, (*grade + 1) % 6),
                        Grade::Ongoing => {}
                    }
                }
            }
            CursorLevel::Moment => {
                if let Some(moment) = self.sel_moment_mut() {
                    moment.completed = !moment.completed;
                }
            }
            CursorLevel::Task => {
                if let Some((_, completed)) = self.sel_task_mut() {
                    *completed = !*completed;
                }
            }
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
        self.sel_menu().and_then(|x| x.get(self.cursor.semester_ix))
    }

    fn sel_period(&self) -> Option<&Period> {
        self.sel_semester()
            .map(|v| &v.periods[self.cursor.period_ix])
    }

    fn sel_course(&self) -> Option<&Course> {
        self.sel_period().and_then(|x| x.get(self.cursor.course_ix))
    }

    fn sel_moment(&self) -> Option<&Moment> {
        self.sel_course()
            .and_then(|x| x.moments.get(self.cursor.moment_ix))
    }

    // fn sel_task(&self) -> Option<(&String, &bool)> {
    //     self.sel_moment()
    //         .and_then(|x| x.tasks.as_ref())
    //         .and_then(|x| x.iter().nth(self.cursor.task_ix))
    // }

    fn sel_menu_mut(&mut self) -> Option<&mut Menu> {
        Some(&mut self.menu)
    }

    fn sel_semester_mut(&mut self) -> Option<&mut Semester> {
        let ix = self.cursor.semester_ix;
        self.sel_menu_mut()?.get_mut(ix)
    }

    fn sel_period_mut(&mut self) -> Option<&mut Period> {
        let ix = self.cursor.period_ix;
        self.sel_semester_mut()?.periods.iter_mut().nth(ix)
    }

    fn sel_course_mut(&mut self) -> Option<&mut Course> {
        let ix = self.cursor.course_ix;
        self.sel_period_mut()?.get_mut(ix)
    }

    fn sel_moment_mut(&mut self) -> Option<&mut Moment> {
        let ix = self.cursor.moment_ix;
        self.sel_course_mut()?.moments.get_mut(ix)
    }

    fn sel_task_mut(&mut self) -> Option<(&String, &mut bool)> {
        let ix = self.cursor.task_ix;
        if let Some(moment) = self.sel_moment_mut() {
            if let Some(tasks) = &mut moment.tasks {
                return tasks.iter_mut().nth(ix);
            }
        }
        None
    }
}
