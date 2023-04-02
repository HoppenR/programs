pub(super) mod cursor;
mod print;

use cursor::{Cursor, CursorLevel};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::BitXorAssign;

#[derive(Deserialize, Serialize)]
pub(super) struct UniInfo {
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
// Helper struct to implement Display for a tuple when iterating over `Tasks`
struct PrintableTask {
    name: String,
    completed: bool,
}

impl UniInfo {
    pub(super) fn cursor_level(&self) -> CursorLevel {
        self.cursor.level
    }

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

    pub(super) fn cursor_exit(&mut self) {
        match self.cursor.level {
            CursorLevel::Semester => {}
            _ => self.cursor.exit(),
        }
    }

    pub(super) fn set_selected_course(&mut self, new_grade: Grade) {
        if let Some(course) = self.sel_course_mut() {
            course.grade = new_grade;
        }
    }

    pub(super) fn toggle_selected_moment(&mut self) {
        if let Some(moment) = self.sel_moment_mut() {
            moment.completed.bitxor_assign(true);
            // if let Some(tasks) = moment.tasks.as_mut() {
            //     for (_, completion) in tasks {
            //         *completion = moment.completed;
            //     }
            // }
        }
    }

    pub(super) fn toggle_selected_task(&mut self) {
        if let Some((_, completed)) = self.sel_task_mut() {
            completed.bitxor_assign(true);
        }
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

    pub(super) fn delete_entry(&mut self) {
        match self.cursor.level {
            CursorLevel::Semester => {
                let ix: usize = self.cursor.semester_ix;
                if let Some(menu) = self.sel_menu_mut() {
                    menu.remove(ix);
                }
            }
            CursorLevel::Period => {}
            CursorLevel::Course => {
                let ix: usize = self.cursor.course_ix;
                if let Some(period) = self.sel_period_mut() {
                    period.remove(ix);
                }
            }
            CursorLevel::Moment => {
                let ix: usize = self.cursor.moment_ix;
                if let Some(course) = self.sel_course_mut() {
                    course.moments.remove(ix);
                }
            }
            CursorLevel::Task => {
                let ix: usize = self.cursor.task_ix;
                let key: String = match self
                    .sel_moment()
                    .and_then(|x| x.tasks.as_ref())
                    .and_then(|x| x.iter().nth(ix))
                    .map(|x| x.0.clone())
                {
                    Some(key_str) => key_str,
                    None => return,
                };

                if let Some(moment) = self.sel_moment_mut() {
                    if let Some(tasks) = moment.tasks.as_mut() {
                        tasks.retain(|n, _| *n != key);
                        if tasks.is_empty() {
                            moment.tasks = None;
                            self.cursor.level = CursorLevel::Moment;
                        }
                    }
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
            Some(semester) => semester.len(),
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
        match self.sel_moment().and_then(|x| x.tasks.as_ref()) {
            Some(tasks) => tasks.len(),
            _ => 0,
        }
    }

    fn sel_menu(&self) -> Option<&Menu> {
        Some(&self.menu)
    }

    fn sel_semester(&self) -> Option<&Semester> {
        self.sel_menu()?.get(self.cursor.semester_ix)
    }

    fn sel_period(&self) -> Option<&Period> {
        self.sel_semester()?.get(self.cursor.period_ix)
    }

    fn sel_course(&self) -> Option<&Course> {
        self.sel_period()?.get(self.cursor.course_ix)
    }

    fn sel_moment(&self) -> Option<&Moment> {
        self.sel_course()?.moments.get(self.cursor.moment_ix)
    }

    // fn sel_task(&self) -> Option<(&String, &bool)> {
    //     self.sel_moment()?
    //         .tasks
    //         .as_ref()?
    //         .iter()
    //         .nth(self.cursor.task_ix)
    // }

    fn sel_menu_mut(&mut self) -> Option<&mut Menu> {
        Some(&mut self.menu)
    }

    fn sel_semester_mut(&mut self) -> Option<&mut Semester> {
        let ix: usize = self.cursor.semester_ix;
        self.sel_menu_mut()?.get_mut(ix)
    }

    fn sel_period_mut(&mut self) -> Option<&mut Period> {
        let ix: usize = self.cursor.period_ix;
        self.sel_semester_mut()?.get_mut(ix)
    }

    fn sel_course_mut(&mut self) -> Option<&mut Course> {
        let ix: usize = self.cursor.course_ix;
        self.sel_period_mut()?.get_mut(ix)
    }

    fn sel_moment_mut(&mut self) -> Option<&mut Moment> {
        let ix: usize = self.cursor.moment_ix;
        self.sel_course_mut()?.moments.get_mut(ix)
    }

    fn sel_task_mut(&mut self) -> Option<(&String, &mut bool)> {
        let ix: usize = self.cursor.task_ix;
        self.sel_moment_mut()?.tasks.as_mut()?.iter_mut().nth(ix)
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

    fn should_print_moments(&self) -> bool {
        match self.grade {
            Grade::Ongoing => true,
            Grade::Completed(passed) => !passed,
            Grade::Grade(_) => false,
        }
    }
}

impl From<(&String, &bool)> for PrintableTask {
    fn from(value: (&String, &bool)) -> Self {
        PrintableTask {
            name: value.0.clone(),
            completed: *value.1,
        }
    }
}
