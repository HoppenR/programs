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

use cursor::{Cursor, Level};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The top level object, containing uni data and a cursor to manipulate that data.
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
#[derive(Deserialize, Serialize)]
struct Course {
    code: String,
    grade: Grade,
    moments: Moments,
    name: String,
}
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum Grade {
    Completed(bool),
    Grade(u8),
    Ongoing,
}
type Moments = Vec<Moment>;
#[derive(Deserialize, Serialize)]
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
    /// Returns the enum `Level` representing which level the cursor is on.
    pub(super) const fn cursor_level(&self) -> &Level {
        &self.cursor.level
    }

    /// Moves the cursor down, increasing the index up to the amount of
    /// entries on the current cursor level.
    pub(super) fn cursor_down(&mut self) {
        let max_value: usize = match self.cursor.level {
            Level::Semester => self.sel_menu_entries(),
            Level::Period => self.sel_semester_entries(),
            Level::Course => self.sel_period_entries(),
            Level::Moment => self.sel_course_entries(),
            Level::Task => self.sel_moment_entries(),
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
            Level::Semester => self.sel_semester_entries(),
            Level::Period => self.sel_period_entries(),
            Level::Course => match self.sel_course().map(Course::should_print_moments) {
                Some(true) => self.sel_course_entries(),
                _ => 0,
            },
            Level::Moment => self.sel_moment_entries(),
            Level::Task => 0,
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
        self.sel_menu_mut().push([Vec::new(), Vec::new()]);
    }

    pub(super) fn add_course(&mut self, code: String, grade: Grade, name: String) {
        if let Some(period) = self.sel_period_mut() {
            period.push(Course {
                code,
                grade,
                moments: Vec::new(),
                name,
            });
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
            });
        }
    }

    pub(super) fn add_task(&mut self, name: String) {
        if let Some(moment) = self.sel_moment_mut() {
            if let Some(tasks) = moment.tasks.as_mut() {
                let _ = tasks.insert(name, false);
            } else {
                moment.tasks = Some(BTreeMap::from([(name, false)]));
            }
        }
    }

    /// Delete the currently targeted entry.
    pub(super) fn delete_entry(&mut self) {
        let (entries, cursorpos) = match self.cursor.level {
            Level::Semester => {
                let ix: usize = self.cursor.semester;
                drop(self.sel_menu_mut().remove(ix));
                (self.sel_menu_entries(), ix)
            }
            Level::Period => (0, self.cursor.period),
            Level::Course => {
                let ix: usize = self.cursor.course;
                if let Some(period) = self.sel_period_mut() {
                    drop(period.remove(ix));
                }
                (self.sel_period_entries(), ix)
            }
            Level::Moment => {
                let ix: usize = self.cursor.moment;
                if let Some(course) = self.sel_course_mut() {
                    drop(course.moments.remove(ix));
                }
                (self.sel_course_entries(), ix)
            }
            Level::Task => {
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
                (self.sel_moment_entries(), self.cursor.task)
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
        self.sel_menu().len()
    }

    fn sel_menu_mut(&mut self) -> &mut Menu {
        &mut self.menu
    }

    const fn sel_menu(&self) -> &Menu {
        &self.menu
    }

    fn sel_semester_entries(&self) -> usize {
        self.sel_semester().map_or(0, |semester| semester.len())
    }

    fn sel_semester_mut(&mut self) -> Option<&mut Semester> {
        let ix: usize = self.cursor.semester;
        self.sel_menu_mut().get_mut(ix)
    }

    fn sel_semester(&self) -> Option<&Semester> {
        let ix: usize = self.cursor.semester;
        self.sel_menu().get(ix)
    }

    fn sel_period_entries(&self) -> usize {
        self.sel_period().map_or(0, std::vec::Vec::len)
    }

    fn sel_period_mut(&mut self) -> Option<&mut Period> {
        let ix: usize = self.cursor.period;
        self.sel_semester_mut()?.get_mut(ix)
    }

    fn sel_period(&self) -> Option<&Period> {
        let ix: usize = self.cursor.period;
        self.sel_semester()?.get(ix)
    }

    fn sel_course_entries(&self) -> usize {
        self.sel_course().map_or(0, |course| course.moments.len())
    }

    fn sel_course_mut(&mut self) -> Option<&mut Course> {
        let ix: usize = self.cursor.course;
        self.sel_period_mut()?.get_mut(ix)
    }

    fn sel_course(&self) -> Option<&Course> {
        let ix: usize = self.cursor.course;
        self.sel_period()?.get(ix)
    }

    fn sel_moment_entries(&self) -> usize {
        self.sel_moment()
            .and_then(|x| x.tasks.as_ref())
            .map_or(0, std::collections::BTreeMap::len)
    }

    fn sel_moment_mut(&mut self) -> Option<&mut Moment> {
        let ix: usize = self.cursor.moment;
        self.sel_course_mut()?.moments.get_mut(ix)
    }

    fn sel_moment(&self) -> Option<&Moment> {
        let ix: usize = self.cursor.moment;
        self.sel_course()?.moments.get(ix)
    }

    fn sel_task(&self) -> Option<(&String, &bool)> {
        let ix: usize = self.cursor.task;
        self.sel_moment()?.tasks.as_ref()?.iter().nth(ix)
    }

    fn sel_task_mut(&mut self) -> Option<(&String, &mut bool)> {
        let ix: usize = self.cursor.task;
        self.sel_moment_mut()?.tasks.as_mut()?.iter_mut().nth(ix)
    }
}

impl Course {
    /// Sums the accrued credits.
    fn sum_credits(&self) -> f32 {
        return self
            .moments
            .iter()
            .filter_map(|v| v.completed.then_some(v.credits))
            .sum();
    }

    /// Sums the maximum posssible.
    fn max_credits(&self) -> f32 {
        return self.moments.iter().map(|v| v.credits).sum();
    }

    /// Returns whether the child moments should be visible on screen.
    const fn should_print_moments(&self) -> bool {
        match self.grade {
            Grade::Ongoing => true,
            Grade::Completed(passed) => !passed,
            Grade::Grade(_) => false,
        }
    }
}