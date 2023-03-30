mod cursor;
use cursor::*;

use super::*;
use serde::Deserialize;
use std::collections::BTreeMap;

type Tasks = BTreeMap<String, bool>;

#[derive(Deserialize, Clone)]
struct Moment {
    completed: bool,
    credits: f32,
    description: String,
    tasks: Option<Tasks>,
}

type Moments = BTreeMap<String, Moment>;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum Grade {
    Completed(bool),
    Grade(usize),
    Ongoing,
}

#[derive(Deserialize, Clone)]
struct Course {
    grade: Grade,
    moments: Moments,
    name: String,
}

type Period = BTreeMap<String, Course>;
type Semester = [Period; 2];

#[derive(Deserialize)]
pub struct UniInfo {
    info: BTreeMap<usize, Semester>,
    #[serde(default)]
    pub cursor: Cursor,
}

impl Course {
    fn sum_credits(self: &Self) -> f32 {
        return self
            .moments
            .values()
            .filter_map(|v| if v.completed { Some(v.credits) } else { None })
            .sum();
    }

    fn max_credits(self: &Self) -> f32 {
        return self.moments.values().map(|v| v.credits).sum();
    }
}

impl UniInfo {
    pub fn increase_cursor(self: &mut Self) {
        let max_value: Option<usize> = match self.cursor.level {
            CursorLevel::Semester => Some(self.info.len() - 1),
            CursorLevel::Period => Some(1),
            CursorLevel::Course => self
                .info
                .values()
                .nth(self.cursor.semester_ix)
                .and_then(|v| Some(v[self.cursor.period_ix].len() - 1)),
            CursorLevel::Moment => self
                .info
                .values()
                .nth(self.cursor.semester_ix)
                .and_then(|v| {
                    v[self.cursor.period_ix]
                        .values()
                        .nth(self.cursor.course_ix)
                        .and_then(|x| Some(x.moments.len() - 1))
                }),
            CursorLevel::Task => self
                .info
                .values()
                .nth(self.cursor.semester_ix)
                .and_then(|v| {
                    v[self.cursor.period_ix]
                        .values()
                        .nth(self.cursor.course_ix)
                        .and_then(|x| {
                            x.moments
                                .values()
                                .nth(self.cursor.moment_ix)
                                .and_then(|w| w.tasks.as_ref().and_then(|y| Some(y.len() - 1)))
                        })
                }),
        };
        if let Some(max) = max_value {
            self.cursor.increase(max);
        }
    }

    pub fn decrease_cursor(self: &mut Self) {
        self.cursor.decrease();
    }
}

fn print_course(
    code: &str,
    course: &Course,
    cursor: &Cursor,
    location: &Vec<usize>,
    indent_level: usize,
) {
    let (color, symbol): (&str, char) = match course.grade {
        Grade::Completed(completed) => match completed {
            true => (GRN, '✓'),
            false => (RED, '✗'),
        },
        Grade::Grade(grade) => match grade {
            (3..=5) => (GRN, char::from_digit(grade as u32, 10).unwrap()),
            _ => unreachable!(),
        },
        Grade::Ongoing => (BLU, '…'),
    };
    print!(
        "{leading}[{color}{symbol}{RST}] {UDL}{code}{RST} \
        {BLD}{BLU}{name}{RST} {credits:.1}hp\n\r",
        leading = indent(indent_level),
        name = course.name,
        credits = course.max_credits(),
    );
    if matches!(course.grade, Grade::Ongoing) {
        print_moments(&course.moments, cursor, location, indent_level + 1);
    }
}

fn print_header(name: &str, index: usize, indent_level: usize) {
    print!(
        "{leading}• {name} {index}:\n\r",
        leading = indent(indent_level),
    );
}

fn print_moments(moments: &Moments, cursor: &Cursor, location: &Vec<usize>, indent_level: usize) {
    for (mom_idx, (moment_code, moment)) in moments.iter().enumerate() {
        let mut mom_loc = location.clone();
        mom_loc.push(mom_idx);
        if cursor.matches(CursorLevel::Moment, &mom_loc) {
            print!("→");
        }
        print!(
            "{leading}{marker}[{moment_code}] \
            {YLW}{CUR}{description}{RST} {credits:.1}hp\n\r",
            leading = indent(indent_level),
            marker = if moment.completed { STK } else { "" },
            credits = moment.credits,
            description = moment.description,
        );
        if !moment.completed {
            print_tasks(&moment.tasks, cursor, &mom_loc, indent_level + 1);
        }
    }
}

fn print_progress(courses: &Vec<Course>, indent_level: usize) {
    let mut accrued_creds: f32 = 0.0;
    let mut total_creds: f32 = 0.0;
    let mut grades: Vec<usize> = Vec::new();
    for course in courses {
        total_creds += course.max_credits();
        accrued_creds += course.sum_credits();
        match course.grade {
            Grade::Completed(completed) => match completed {
                true => {}
                false => {}
            },
            Grade::Grade(grade) => match grade {
                (3..=5) => {
                    grades.push(grade);
                }
                _ => unreachable!(),
            },
            Grade::Ongoing => {}
        }
    }
    let average: f32 = grades.iter().sum::<usize>() as f32 / grades.len() as f32;
    print!(
        "{leading}{avg_color}{average:.3}{RST}avg ‖ \
        {cred_color}{accrued_creds:.1}/{total_creds:.1}{RST}hp\n\r",
        leading = indent(indent_level),
        cred_color = if accrued_creds > 0.0 { CYN } else { RED },
        avg_color = if !f32::is_nan(average) { CYN } else { RED },
    );
}

fn print_tasks(tasks: &Option<Tasks>, cursor: &Cursor, location: &Vec<usize>, indent_level: usize) {
    if let Some(tasks) = tasks {
        for (tsk_idx, (task_name, completion)) in tasks.iter().enumerate() {
            let mut tsk_loc = location.clone();
            tsk_loc.push(tsk_idx);
            if cursor.matches(CursorLevel::Task, &tsk_loc) {
                print!("→");
            }
            print!(
                "{leading}{marker}{task_name}{RST}\n\r",
                leading = indent(indent_level),
                marker = if *completion { STK } else { "" },
            );
        }
    }
}

pub fn print_uni_info(uni_info: &UniInfo, cursor: &Cursor) {
    let mut all_courses: Vec<Course> = Vec::new();
    let mut semester_courses: Vec<Course> = Vec::new();
    let mut period_courses: Vec<Course> = Vec::new();
    print!("{BLD}{RED}Averages only include courses graded 3-5{RST}\n\r");
    for (sem_idx, semester) in &uni_info.info {
        if cursor.matches(CursorLevel::Semester, &vec![*sem_idx - 1]) {
            print!("→");
        }
        print_header("Semester", *sem_idx, 0);
        for (per_idx, period) in semester.iter().enumerate() {
            if cursor.matches(CursorLevel::Period, &vec![*sem_idx - 1, per_idx]) {
                print!("→");
            }
            print_header("Period", per_idx + 1, 1);
            for (cor_idx, (name, course)) in period.iter().enumerate() {
                if cursor.matches(CursorLevel::Course, &vec![*sem_idx - 1, per_idx, cor_idx]) {
                    print!("→");
                }
                print_course(
                    name,
                    course,
                    &uni_info.cursor,
                    &vec![*sem_idx - 1, per_idx, cor_idx],
                    2,
                );
                period_courses.push(course.clone());
            }
            print_progress(&period_courses, 2);
            semester_courses.append(&mut period_courses);
        }
        print_progress(&semester_courses, 1);
        all_courses.append(&mut semester_courses);
        print!("\n\r");
    }
    print_progress(&all_courses, 0);
}
