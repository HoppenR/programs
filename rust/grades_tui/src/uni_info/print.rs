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
use crate::uni_info::{Course, Grade, Level, Moment, UniInfo};
use std::fmt::{self, Display, Formatter};

const INDENT_WIDTH: usize = 4;
const ELLIPSIS: char = '\u{2026}';
const CHECKMARK: char = '\u{2713}';
const CROSS: char = '\u{2717}';
const BARS: char = '\u{2016}';
const ARROW: &str = "\u{2192}";
const DOT: char = '\u{2022}';

/// Helper struct to implement `Display` for a tuple when iterating over `Tasks`.
struct PrintableTask {
    name: String,
    completed: bool,
}

/// Writes and formats a `uni_info` object using their `Display` implementations.
/// The leading indentation is given in the `start` parameter.
fn write_entry<T>(f: &mut Formatter<'_>, entry: &T, targeted: bool, indents: usize) -> fmt::Result
where
    T: Display,
{
    write!(
        f,
        "{indicator}{lead:width$}{entry}{ERASE_TO_LINE_END}\n\r",
        indicator = if targeted { ARROW } else { "" },
        lead = "",
        width = indents * INDENT_WIDTH,
    )
}

/// Writes and formats a string header in the menu.
/// The leading indentation is given in the `start` parameter.
fn write_header(
    f: &mut Formatter<'_>,
    title: &str,
    index: usize,
    targeted: bool,
    indents: usize,
) -> fmt::Result {
    write!(
        f,
        "{indicator}{lead:width$}{DOT} {title} {index}:{ERASE_TO_LINE_END}\n\r",
        indicator = if targeted { ARROW } else { "" },
        lead = "",
        width = indents * INDENT_WIDTH,
    )
}

/// Writes and formats the average grade of the `courses` parameter,
/// as well as its current and total credits.
/// The leading indentation is given in the `start` parameter.
fn write_progress(f: &mut Formatter<'_>, courses: &Vec<&Course>, indents: usize) -> fmt::Result {
    let mut accrued_creds: f32 = 0.0;
    let mut total_creds: f32 = 0.0;
    let mut grades: Vec<f32> = Vec::new();
    let mut total_grade_items: f32 = 0.0;
    for course in courses {
        total_creds += course.max_credits();
        accrued_creds += course.sum_credits();
        match course.grade {
            Grade::Completed(_) | Grade::Ongoing => {}
            Grade::Grade(grade) => match grade {
                (3..=5) => {
                    grades.push(f32::from(grade));
                    total_grade_items += 1.0;
                }
                _ => return Err(fmt::Error),
            },
            Grade::Traditional(grade) => match grade {
                ('A'..='E') => {
                    grades.push(f32::from(grade as u8 - b'A').mul_add(-0.5, 5.0));
                    total_grade_items += 1.0;
                }
                _ => return Err(fmt::Error),
            },
        }
    }
    let average: f32 = grades.iter().sum::<f32>() / total_grade_items;
    write!(
        f,
        "{lead:width$}{avg_color}{average:.3}{RST}avg {BARS} \
        {cred_color}{accrued_creds:.1}/{total_creds:.1}{RST} ECTS{ERASE_TO_LINE_END}\n\r",
        lead = "",
        width = indents * INDENT_WIDTH,
        avg_color = if f32::is_nan(average) { RED } else { CYN },
        cred_color = if accrued_creds == 0.0 { RED } else { CYN },
    )
}

impl Display for UniInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut all_courses: Vec<&Course> = Vec::new();
        let mut semester_courses: Vec<&Course> = Vec::new();
        let mut period_courses: Vec<&Course> = Vec::new();
        write!(
            f,
            "{BLD}{RED}Averages include gradings 3..5 and A..E (B=4.5, D=3.5){RST}{ERASE_TO_LINE_END}\n\r",
        )?;
        for (sem_ix, sem) in self.menu.iter().enumerate() {
            let cursor = Cursor {
                semester: sem_ix,
                level: Level::Semester,
                ..Default::default()
            };
            write_header(f, "Semester", sem_ix + 1, self.cursor == cursor, 0)?;
            for (period_ix, period) in sem.iter().enumerate() {
                let cursor = Cursor {
                    period: period_ix,
                    level: Level::Period,
                    ..cursor
                };
                write_header(f, "Period", period_ix + 1, self.cursor == cursor, 1)?;
                for (course_ix, course) in period.iter().enumerate() {
                    period_courses.push(course);
                    let cursor = Cursor {
                        course: course_ix,
                        level: Level::Course,
                        ..cursor
                    };
                    write_entry(f, course, self.cursor == cursor, 2)?;
                    if !course.should_print_moments() {
                        continue;
                    }
                    for (moment_ix, moment) in course.moments.iter().enumerate() {
                        let cursor = Cursor {
                            moment: moment_ix,
                            level: Level::Moment,
                            ..cursor
                        };
                        write_entry(f, moment, self.cursor == cursor, 3)?;
                        if let Some(tasks) = &moment.tasks {
                            for (task_ix, task) in tasks.iter().map(PrintableTask::from).enumerate()
                            {
                                let cursor = Cursor {
                                    task: task_ix,
                                    level: Level::Task,
                                    ..cursor
                                };
                                write_entry(f, &task, self.cursor == cursor, 5)?;
                            }
                        }
                    }
                }
                write_progress(f, &period_courses, 2)?;
                semester_courses.append(&mut period_courses);
            }
            write_progress(f, &semester_courses, 1)?;
            all_courses.append(&mut semester_courses);
        }
        write_progress(f, &all_courses, 0)?;
        write!(f, "{ERASE_TO_DISP_END}")
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (color, symbol): (&str, char) = match self.grade {
            Grade::Completed(passed) => {
                if passed {
                    Ok((GRN, CHECKMARK))
                } else {
                    Ok((RED, CROSS))
                }
            }
            Grade::Grade(grade) => match grade {
                (3..=5) => {
                    let grade_ch: char = (grade + b'0') as char;
                    Ok((GRN, grade_ch))
                }
                _ => Err(fmt::Error),
            },
            Grade::Traditional(grade) => match grade {
                ('A'..='E') => Ok((GRN, grade)),
                _ => Err(fmt::Error),
            },
            Grade::Ongoing => Ok((BLU, ELLIPSIS)),
        }?;
        write!(
            f,
            "[{color}{symbol}{RST}] {UDL}{code}{RST} {BLD}{BLU}{name}{RST} {credits:.1} ECTS",
            code = self.code,
            name = self.name,
            credits = self.max_credits(),
        )
    }
}

impl Display for Moment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (color, status): (&str, char) = match self.grade {
            Grade::Completed(completed) => {
                if completed {
                    Ok((GRN, 'G'))
                } else {
                    Ok((RED, 'U'))
                }
            }
            Grade::Grade(grade) => match grade {
                (3..=5) => {
                    let grade_ch: char = (grade + b'0') as char;
                    Ok((GRN, grade_ch))
                }
                _ => Err(fmt::Error),
            },
            Grade::Traditional(grade) => match grade {
                ('A'..='E') => Ok((GRN, grade)),
                _ => Err(fmt::Error),
            },
            Grade::Ongoing => Ok((RED, ' ')),
        }?;
        write!(
            f,
            "[{color}{status}{RST}] {marker}[{code}] {YLW}{CUR}{description}{RST} {credits:.1} ECTS",
            marker = if color == GRN { STK } else { "" },
            code = self.code,
            credits = self.credits,
            description = self.description,
        )
    }
}

impl Display for PrintableTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
        Self {
            name: value.0.clone(),
            completed: *value.1,
        }
    }
}
