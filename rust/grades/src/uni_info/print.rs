//! Printing functions for the `UniInfo` objects
//!
//! This module contains the `Display` implementations for each of the
//! objects in `uni_info`, as well as a helper struct `PrintableTask`.
//! This helper struct implements the formatting trait for a key-pair tuple
//! when iterating over a `BTreeMap` such as `Tasks`.
//!
//! The prints are designed with a terminal output in mind, always ending each
//! line with a terminal escape sequence that clears the line in the terminal.
//! After `UniInfo` is done printing it also tells the terminal to clear the rest
//! of the terminal contents from the cursor to the end of the screen.
//!
//! # Usage
//!
//! An example of the usage is:
//!
//! ```
//! fn my_func(uni: &UniInfo, course: &Course) {
//!     let terminal_output = format!("{course}");
//!     write!(std::io::stdout(), "{uni}").unwrap();
//! }
//! ```
//!
//! For invalid data it will simply tell the formatter to stop and return an uncategorized error.

use crate::ui::term::attributes::{BLD, BLU, CUR, CYN, GRN, RED, RST, STK, UDL, YLW};
use crate::ui::term::{ERASE_TO_DISP_END, ERASE_TO_LINE_END};
use crate::uni_info::cursor::Cursor;
use crate::uni_info::{Course, CursorLevel, Grade, Moment, UniInfo};
use std::fmt::{self, Display, Formatter};

/// Helper struct to implement Display for a tuple when iterating over `Tasks`.
struct PrintableTask {
    name: String,
    completed: bool,
}

const INDENT: &str = "    ";

fn indent(indent_level: usize) -> String {
    INDENT.repeat(indent_level)
}

/// Writes and formats a `uni_info` object using their `Display` implementations.
/// The leading indentation is given in the `start` parameter.
fn write_entry<T>(f: &mut Formatter, entry: &T, targeted: bool, start: &str) -> fmt::Result
where
    T: Display,
{
    write!(
        f,
        "{indicator}{start}{entry}{end}\n\r",
        indicator = if targeted { "→" } else { "" },
        end = ERASE_TO_LINE_END,
    )
}

/// Writes and formats a string header in the menu.
/// The leading indentation is given in the `start` parameter.
fn write_header(
    f: &mut Formatter,
    title: &str,
    index: usize,
    targeted: bool,
    start: &str,
) -> fmt::Result {
    write!(
        f,
        "{indicator}{start}• {title} {index}:{end}\n\r",
        indicator = if targeted { "→" } else { "" },
        end = ERASE_TO_LINE_END,
    )
}

/// Writes and formats the average grade of the `courses` parameter,
/// as well as its current and total credits.
/// The leading indentation is given in the `start` parameter.
fn write_progress(f: &mut Formatter, courses: &Vec<Course>, start: &str) -> fmt::Result {
    let mut accrued_creds: f32 = 0.0;
    let mut total_creds: f32 = 0.0;
    let mut grades: Vec<usize> = Vec::new();
    for course in courses {
        total_creds += course.max_credits();
        accrued_creds += course.sum_credits();
        match course.grade {
            Grade::Completed(_) => {}
            Grade::Ongoing => {}
            Grade::Grade(grade) => match grade {
                (3..=5) => {
                    grades.push(grade);
                }
                _ => return Err(fmt::Error),
            },
        }
    }
    let average: f32 = grades.iter().sum::<usize>() as f32 / grades.len() as f32;
    write!(
        f,
        "{start}{avg_color}{average:.3}{RST}avg ‖ \
        {cred_color}{accrued_creds:.1}/{total_creds:.1}{RST}hp{end}\n\r",
        cred_color = if accrued_creds == 0.0 { RED } else { CYN },
        avg_color = if f32::is_nan(average) { RED } else { CYN },
        end = ERASE_TO_LINE_END,
    )
}

impl Display for UniInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut all_courses: Vec<Course> = Vec::new();
        let mut semester_courses: Vec<Course> = Vec::new();
        let mut period_courses: Vec<Course> = Vec::new();
        write!(
            f,
            "{BLD}{RED}Averages only include courses graded 3-5{RST}{ERASE_TO_DISP_END}\n\r",
        )?;
        for (sem_ix, sem) in self.menu.iter().enumerate() {
            let cursor = Cursor {
                semester_ix: sem_ix,
                level: CursorLevel::Semester,
                ..Default::default()
            };
            write_header(f, "Semester", sem_ix + 1, self.cursor == cursor, &indent(0))?;
            for (period_ix, period) in sem.iter().enumerate() {
                let cursor = Cursor {
                    period_ix,
                    level: CursorLevel::Period,
                    ..cursor
                };
                write_header(
                    f,
                    "Period",
                    period_ix + 1,
                    self.cursor == cursor,
                    &indent(1),
                )?;
                for (course_ix, course) in period.iter().enumerate() {
                    period_courses.push(course.clone());
                    let cursor = Cursor {
                        course_ix,
                        level: CursorLevel::Course,
                        ..cursor
                    };
                    write_entry(f, course, self.cursor == cursor, &indent(2))?;
                    if !course.should_print_moments() {
                        continue;
                    }
                    for (moment_ix, moment) in course.moments.iter().enumerate() {
                        let cursor = Cursor {
                            moment_ix,
                            level: CursorLevel::Moment,
                            ..cursor
                        };
                        write_entry(f, moment, self.cursor == cursor, &indent(3))?;
                        if let Some(tasks) = &moment.tasks {
                            for (task_ix, task) in tasks.iter().map(PrintableTask::from).enumerate()
                            {
                                let cursor = Cursor {
                                    task_ix,
                                    level: CursorLevel::Task,
                                    ..cursor
                                };
                                write_entry(f, &task, self.cursor == cursor, &indent(4))?;
                            }
                        }
                    }
                }
                write_progress(f, &period_courses, &indent(2))?;
                semester_courses.append(&mut period_courses);
            }
            write_progress(f, &semester_courses, &indent(1))?;
            all_courses.append(&mut semester_courses);
        }
        write_progress(f, &all_courses, &indent(0))?;
        write!(f, "{ERASE_TO_DISP_END}")
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (color, symbol): (&str, char) = match self.grade {
            Grade::Completed(passed) => {
                if passed {
                    Ok((GRN, '✓'))
                } else {
                    Ok((RED, '✗'))
                }
            }
            Grade::Grade(grade) => match grade {
                (3..=5) => {
                    let grade_ch: char = (u8::try_from(grade).unwrap() + b'0') as char;
                    Ok((GRN, grade_ch))
                }
                _ => Err(fmt::Error),
            },
            Grade::Ongoing => Ok((BLU, '…')),
        }?;
        write!(
            f,
            "[{color}{symbol}{RST}] {UDL}{code}{RST} {BLD}{BLU}{name}{RST} {credits:.1}hp",
            code = self.code,
            name = self.name,
            credits = self.max_credits(),
        )
    }
}

impl Display for Moment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{marker}[{code}] {YLW}{CUR}{description}{RST} {credits:.1}hp",
            marker = if self.completed { STK } else { "" },
            code = self.code,
            credits = self.credits,
            description = self.description,
        )
    }
}

impl Display for PrintableTask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{marker}{task_name}{RST}",
            marker = if self.completed { STK } else { "" },
            task_name = self.name,
        )
    }
}

/// Helper function to convert a key-value pair from a `Tasks` object.
impl From<(&String, &bool)> for PrintableTask {
    fn from(value: (&String, &bool)) -> Self {
        PrintableTask {
            name: value.0.clone(),
            completed: *value.1,
        }
    }
}
