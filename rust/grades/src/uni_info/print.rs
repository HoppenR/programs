use super::{cursor::Cursor, Course, CursorLevel, Grade, Moment, PrintableTask, UniInfo};
use crate::ui::term::{
    BLD, BLU, CUR, CYN, ERASE_TO_DISP_END, ERASE_TO_LINE_END, GRN, RED, RST, STK, UDL, YLW,
};
use std::fmt::{self, Display, Formatter};

const INDENT: &str = "    ";

fn indent(indent_level: usize) -> String {
    INDENT.repeat(indent_level)
}

fn write_entry<T>(f: &mut Formatter<'_>, entry: T, targeted: bool, start: String) -> fmt::Result
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

fn write_header(
    f: &mut Formatter<'_>,
    title: &str,
    index: usize,
    targeted: bool,
    start: String,
) -> fmt::Result {
    write!(
        f,
        "{indicator}{start}• {title} {index}:{end}\n\r",
        indicator = if targeted { "→" } else { "" },
        end = ERASE_TO_LINE_END,
    )
}

fn write_progress(f: &mut Formatter<'_>, courses: &Vec<Course>, start: String) -> fmt::Result {
    let mut accrued_creds: f32 = 0.0;
    let mut total_creds: f32 = 0.0;
    let mut grades: Vec<usize> = Vec::new();
    for course in courses {
        total_creds += course.max_credits();
        accrued_creds += course.sum_credits();
        match course.grade {
            Grade::Completed(passed) => match passed {
                true => {}
                false => {}
            },
            Grade::Grade(grade) => match grade {
                (3..=5) => {
                    grades.push(grade);
                }
                _ => return Err(fmt::Error),
            },
            Grade::Ongoing => {}
        }
    }
    let average: f32 = grades.iter().sum::<usize>() as f32 / grades.len() as f32;
    write!(
        f,
        "{start}{avg_color}{average:.3}{RST}avg ‖ \
        {cred_color}{accrued_creds:.1}/{total_creds:.1}{RST}hp{end}\n\r",
        cred_color = if accrued_creds > 0.0 { CYN } else { RED },
        avg_color = if !f32::is_nan(average) { CYN } else { RED },
        end = ERASE_TO_LINE_END,
    )
}

impl Display for UniInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut all_courses: Vec<Course> = Vec::new();
        let mut semester_courses: Vec<Course> = Vec::new();
        let mut period_courses: Vec<Course> = Vec::new();
        write!(
            f,
            "{BLD}{RED}Averages only include courses graded 3-5{RST}{end}\n\r",
            end = ERASE_TO_LINE_END,
        )?;
        for (sem_ix, sem) in self.menu.iter().enumerate() {
            let cursor = Cursor {
                semester_ix: sem_ix,
                level: CursorLevel::Semester,
                ..Default::default()
            };
            write_header(f, "Semester", sem_ix + 1, self.cursor == cursor, indent(0))?;
            for (period_ix, period) in sem.iter().enumerate() {
                let cursor = Cursor {
                    period_ix,
                    level: CursorLevel::Period,
                    ..cursor
                };
                write_header(f, "Period", period_ix + 1, self.cursor == cursor, indent(1))?;
                for (course_ix, course) in period.iter().enumerate() {
                    period_courses.push(course.clone());
                    let cursor = Cursor {
                        course_ix,
                        level: CursorLevel::Course,
                        ..cursor
                    };
                    write_entry(f, course, self.cursor == cursor, indent(2))?;
                    if !course.should_print_moments() {
                        continue;
                    }
                    for (moment_ix, moment) in course.moments.iter().enumerate() {
                        let cursor = Cursor {
                            moment_ix,
                            level: CursorLevel::Moment,
                            ..cursor
                        };
                        write_entry(f, moment, self.cursor == cursor, indent(3))?;
                        if let Some(tasks) = &moment.tasks {
                            for (task_ix, task) in tasks.iter().map(PrintableTask::from).enumerate()
                            {
                                let cursor = Cursor {
                                    task_ix,
                                    level: CursorLevel::Task,
                                    ..cursor
                                };
                                write_entry(f, task, self.cursor == cursor, indent(4))?;
                            }
                        }
                    }
                }
                write_progress(f, &period_courses, indent(2))?;
                semester_courses.append(&mut period_courses);
            }
            write_progress(f, &semester_courses, indent(1))?;
            all_courses.append(&mut semester_courses);
        }
        write_progress(f, &all_courses, indent(0))?;
        write!(f, "{end}", end = ERASE_TO_DISP_END)
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (color, symbol): (&str, char) = match self.grade {
            Grade::Completed(passed) => match passed {
                true => Ok((GRN, '✓')),
                false => Ok((RED, '✗')),
            },
            Grade::Grade(grade) => match grade {
                (3..=5) => Ok((GRN, char::from_digit(grade as u32, 10).unwrap())),
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{marker}{task_name}{RST}",
            marker = if self.completed { STK } else { "" },
            task_name = self.name,
        )
    }
}
