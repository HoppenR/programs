//! An object that provides data for manipulating University Info data
//!
//! This module contains a struct representing University Info that is meant
//! to be serialized from JSON using `SerDe`. It is meant to fully represent the
//! state of a menu with the data, and provides convenient bindings meant for
//! accessing and manipulating it soloely through a cursor.
//!
//! # Usage
//!
//! Examples of the usage is:
//!
//! ```
//! fn delete_parent_entry(uni: &mut UniInfo) {
//!     uni.cursor_exit();
//!     uni.delete_entry();
//! }
//! ```
//!
//! ```
//! fn add_task(uni: &mut UniInfo) {
//!     assert!(uni.cursor_level() == CursorLevel::Course);
//!     uni.cursor_enter();
//!     uni.add_task("Do the dishes!".to_string());
//! }
//! ```
//!
//! Even though the functions are designed to be failsafe, they might however
//! not make sense when you try to modify an object that does not relate to the
//! cursors position. In which case the data changes might be unexpected.

pub(super) mod cursor;
mod print;

use cursor::{Cursor, CursorLevel};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize)]
pub(super) struct UniInfo {
    #[serde(default)]
    menu: Menu,
    #[serde(skip)]
    cursor: Cursor,
}
type Menu = Vec<Semester>;
type Semester = [Period; 2];
type Period = Vec<Course>;
#[derive(Deserialize, Serialize, Clone)]
struct Course {
    code: String,
    grade: Grade,
    moments: Moments,
    name: String,
}
#[derive(Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub(super) enum Grade {
    Completed(bool),
    Grade(usize),
    Ongoing,
}
type Moments = Vec<Moment>;
#[derive(Deserialize, Serialize, Clone)]
struct Moment {
    code: String,
    completed: bool,
    credits: f32,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tasks: Option<Tasks>,
}
type Tasks = BTreeMap<String, bool>;

/// A struct that represents university info, as well as data and bindings to
/// navigate a menu of its data members.
impl UniInfo {
    pub(super) fn cursor_level(&self) -> CursorLevel {
        self.cursor.level
    }

    /// Moves the cursor down, increasing the index up to the amount of
    /// entries on the current cursor level.
    pub(super) fn cursor_down(&mut self) {
        let max_value: usize = match self.cursor.level {
            CursorLevel::Semester => self.sel_menu_entries(),
            CursorLevel::Period => self.sel_semester_entries(),
            CursorLevel::Course => self.sel_period_entries(),
            CursorLevel::Moment => self.sel_course_entries(),
            CursorLevel::Task => self.sel_moment_entries(),
        };
        self.cursor.down(max_value);
    }

    /// Moves the cursor up, decreasing the index down until, and including, 0.
    pub(super) fn cursor_up(&mut self) {
        self.cursor.up();
    }

    /// Indents the cursor depending on if there are any objects that should
    /// be printable on the indented cursor level.
    pub(super) fn cursor_enter(&mut self) {
        let num_entries_next_level: usize = match self.cursor.level {
            CursorLevel::Semester => self.sel_semester_entries(),
            CursorLevel::Period => self.sel_period_entries(),
            CursorLevel::Course => match self.sel_course().map(|x| x.should_print_moments()) {
                Some(true) => self.sel_course_entries(),
                _ => 0,
            },
            CursorLevel::Moment => self.sel_moment_entries(),
            CursorLevel::Task => 0,
        };
        if num_entries_next_level > 0 {
            self.cursor.enter();
        }
    }

    /// Unindents the cursor.
    pub(super) fn cursor_exit(&mut self) {
        self.cursor.exit();
    }

    pub(super) fn add_semester(&mut self) {
        if let Some(menu) = self.sel_menu_mut() {
            menu.push([Vec::new(), Vec::new()])
        }
    }

    pub(super) fn add_course(&mut self, code: String, grade: Grade, name: String) {
        if let Some(period) = self.sel_period_mut() {
            period.push(Course {
                code,
                grade,
                moments: Vec::new(),
                name,
            })
        }
    }

    pub(super) fn add_moment(&mut self, code: String, credits: f32, description: String) {
        if let Some(course) = self.sel_course_mut() {
            course.moments.push(Moment {
                code,
                completed: false,
                credits,
                description,
                tasks: None,
            })
        }
    }

    pub(super) fn add_task(&mut self, name: String) {
        if let Some(moment) = self.sel_moment_mut() {
            if let Some(tasks) = moment.tasks.as_mut() {
                tasks.insert(name, false);
            } else {
                moment.tasks = Some(BTreeMap::from([(name, false)]));
            }
        }
    }

    /// Delete the currently targeted entry.
    pub(super) fn delete_entry(&mut self) {
        let (entries, cursorpos) = match self.cursor.level {
            CursorLevel::Semester => {
                let ix: usize = self.cursor.semester_ix;
                if let Some(menu) = self.sel_menu_mut() {
                    menu.remove(ix);
                }
                (self.sel_menu_entries(), ix)
            }
            CursorLevel::Period => (0, self.cursor.period_ix),
            CursorLevel::Course => {
                let ix: usize = self.cursor.course_ix;
                if let Some(period) = self.sel_period_mut() {
                    period.remove(ix);
                }
                (self.sel_period_entries(), ix)
            }
            CursorLevel::Moment => {
                let ix: usize = self.cursor.moment_ix;
                if let Some(course) = self.sel_course_mut() {
                    course.moments.remove(ix);
                }
                (self.sel_course_entries(), ix)
            }
            CursorLevel::Task => {
                if let Some(key) = self.sel_task().map(|x| x.0.clone()) {
                    if let Some(moment) = self.sel_moment_mut() {
                        if let Some(tasks) = moment.tasks.as_mut() {
                            tasks.retain(|n, _| *n != key);
                            if tasks.is_empty() {
                                moment.tasks = None;
                            }
                        }
                    }
                }
                (self.sel_moment_entries(), self.cursor.task_ix)
            }
        };
        if entries == 0 {
            self.cursor_exit();
        } else if cursorpos == entries {
            self.cursor_up();
        }
    }

    /// Sets the `grade` of the currently targeted `course` to `new_grade`.
    pub(super) fn set_selected_course(&mut self, new_grade: Grade) {
        if let Some(course) = self.sel_course_mut() {
            course.grade = new_grade;
        }
    }

    /// Toggles the completion of the currently selected moment on/off.
    pub(super) fn toggle_selected_moment(&mut self) {
        if let Some(moment) = self.sel_moment_mut() {
            moment.completed ^= true;
        }
    }

    /// Toggles the completion of the currently selected task on/off.
    pub(super) fn toggle_selected_task(&mut self) {
        if let Some((_, completed)) = self.sel_task_mut() {
            *completed ^= true;
        }
    }

    fn sel_menu_entries(&self) -> usize {
        match self.sel_menu() {
            Some(menu) => menu.len(),
            _ => 0,
        }
    }

    fn sel_menu_mut(&mut self) -> Option<&mut Menu> {
        Some(&mut self.menu)
    }

    fn sel_menu(&self) -> Option<&Menu> {
        Some(&self.menu)
    }

    fn sel_semester_entries(&self) -> usize {
        match self.sel_semester() {
            Some(semester) => semester.len(),
            _ => 0,
        }
    }

    fn sel_semester_mut(&mut self) -> Option<&mut Semester> {
        let ix: usize = self.cursor.semester_ix;
        self.sel_menu_mut()?.get_mut(ix)
    }

    fn sel_semester(&self) -> Option<&Semester> {
        let ix: usize = self.cursor.semester_ix;
        self.sel_menu()?.get(ix)
    }

    fn sel_period_entries(&self) -> usize {
        match self.sel_period() {
            Some(period) => period.len(),
            _ => 0,
        }
    }

    fn sel_period_mut(&mut self) -> Option<&mut Period> {
        let ix: usize = self.cursor.period_ix;
        self.sel_semester_mut()?.get_mut(ix)
    }

    fn sel_period(&self) -> Option<&Period> {
        let ix: usize = self.cursor.period_ix;
        self.sel_semester()?.get(ix)
    }

    fn sel_course_entries(&self) -> usize {
        match self.sel_course() {
            Some(course) => course.moments.len(),
            _ => 0,
        }
    }

    fn sel_course_mut(&mut self) -> Option<&mut Course> {
        let ix: usize = self.cursor.course_ix;
        self.sel_period_mut()?.get_mut(ix)
    }

    fn sel_course(&self) -> Option<&Course> {
        let ix: usize = self.cursor.course_ix;
        self.sel_period()?.get(ix)
    }

    fn sel_moment_entries(&self) -> usize {
        match self.sel_moment().and_then(|x| x.tasks.as_ref()) {
            Some(tasks) => tasks.len(),
            _ => 0,
        }
    }

    fn sel_moment_mut(&mut self) -> Option<&mut Moment> {
        let ix: usize = self.cursor.moment_ix;
        self.sel_course_mut()?.moments.get_mut(ix)
    }

    fn sel_moment(&self) -> Option<&Moment> {
        let ix: usize = self.cursor.moment_ix;
        self.sel_course()?.moments.get(ix)
    }

    fn sel_task(&self) -> Option<(&String, &bool)> {
        let ix: usize = self.cursor.task_ix;
        self.sel_moment()?.tasks.as_ref()?.iter().nth(ix)
    }

    fn sel_task_mut(&mut self) -> Option<(&String, &mut bool)> {
        let ix: usize = self.cursor.task_ix;
        self.sel_moment_mut()?.tasks.as_mut()?.iter_mut().nth(ix)
    }
}

impl Course {
    /// Sums the accrued credits.
    fn sum_credits(&self) -> f32 {
        return self
            .moments
            .iter()
            .filter_map(|v| if v.completed { Some(v.credits) } else { None })
            .sum();
    }

    /// Sums the maximum posssible.
    fn max_credits(&self) -> f32 {
        return self.moments.iter().map(|v| v.credits).sum();
    }

    /// Returns whether the child moments should be visible on screen.
    fn should_print_moments(&self) -> bool {
        match self.grade {
            Grade::Ongoing => true,
            Grade::Completed(passed) => !passed,
            Grade::Grade(_) => false,
        }
    }
}
