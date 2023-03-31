use super::{cursor::Cursor, Course, CursorLevel, Grade, Moment, UniInfo};
use std::fmt::Display;

const RST: &str = "\x1b[0m";
const BLD: &str = "\x1b[1m";
const CUR: &str = "\x1b[3m";
const UDL: &str = "\x1b[4m";
const STK: &str = "\x1b[9m";
const RED: &str = "\x1b[91m";
const GRN: &str = "\x1b[92m";
const YLW: &str = "\x1b[93m";
const BLU: &str = "\x1b[94m";
const CYN: &str = "\x1b[96m";

const EOL: &str = "\x1b[K\n\r";
const INDENT: &str = "    ";

fn indent(indent_level: usize) -> String {
    INDENT.repeat(indent_level)
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (color, symbol): (&str, char) = match self.grade {
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
        write!(
            f,
            "[{color}{symbol}{RST}] {UDL}{code}{RST} \
            {BLD}{BLU}{name}{RST} {credits:.1}hp{EOL}",
            code = self.code,
            name = self.name,
            credits = self.max_credits(),
        )
    }
}

impl Display for Moment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{marker}[{code}] \
            {YLW}{CUR}{description}{RST} {credits:.1}hp{EOL}",
            marker = if self.completed { STK } else { "" },
            code = self.code,
            credits = self.credits,
            description = self.description,
        )
    }
}

fn print_header(
    f: &mut std::fmt::Formatter<'_>,
    name: &str,
    index: usize,
    indent_level: usize,
) -> std::fmt::Result {
    write!(
        f,
        "{leading}• {name} {index}:{EOL}",
        leading = indent(indent_level),
    )
}

fn print_progress(
    f: &mut std::fmt::Formatter<'_>,
    courses: &Vec<Course>,
    indent_level: usize,
) -> std::fmt::Result {
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
    write!(
        f,
        "{leading}{avg_color}{average:.3}{RST}avg ‖ \
        {cred_color}{accrued_creds:.1}/{total_creds:.1}{RST}hp{EOL}",
        leading = indent(indent_level),
        cred_color = if accrued_creds > 0.0 { CYN } else { RED },
        avg_color = if !f32::is_nan(average) { CYN } else { RED },
    )
}

impl Display for UniInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all_courses: Vec<Course> = Vec::new();
        let mut semester_courses: Vec<Course> = Vec::new();
        let mut period_courses: Vec<Course> = Vec::new();
        write!(
            f,
            "{BLD}{RED}Averages only include courses graded 3-5{RST}{EOL}"
        )?;
        for sem in &self.menu {
            if self.cursor == Cursor::new(sem.index - 1, 0, 0, 0, 0, CursorLevel::Semester) {
                write!(f, "→")?;
            }
            print_header(f, "Semester", sem.index, 0)?;
            for (per_ix, period) in sem.periods.iter().enumerate() {
                if self.cursor == Cursor::new(sem.index - 1, per_ix, 0, 0, 0, CursorLevel::Period) {
                    write!(f, "→")?;
                }
                print_header(f, "Period", per_ix + 1, 1)?;
                for (cor_ix, course) in period.iter().enumerate() {
                    if self.cursor
                        == Cursor::new(sem.index - 1, per_ix, cor_ix, 0, 0, CursorLevel::Course)
                    {
                        write!(f, "→")?;
                    }
                    write!(f, "{leading}{course}", leading = indent(2))?;
                    if matches!(course.grade, Grade::Ongoing) {
                        for (mom_ix, moment) in course.moments.iter().enumerate() {
                            if self.cursor
                                == Cursor::new(
                                    sem.index - 1,
                                    per_ix,
                                    cor_ix,
                                    mom_ix,
                                    0,
                                    CursorLevel::Moment,
                                )
                            {
                                write!(f, "→")?;
                            }
                            write!(f, "{leading}{moment}", leading = indent(3))?;
                            if let Some(tasks) = &moment.tasks {
                                for (tsk_ix, (task_name, completed)) in tasks.iter().enumerate() {
                                    if self.cursor
                                        == Cursor::new(
                                            sem.index - 1,
                                            per_ix,
                                            cor_ix,
                                            mom_ix,
                                            tsk_ix,
                                            CursorLevel::Task,
                                        )
                                    {
                                        write!(f, "→")?;
                                    }
                                    write!(
                                        f,
                                        "{leading}{marker}[{task_name}]{RST}{EOL}",
                                        leading = indent(4),
                                        marker = if *completed { STK } else { "" },
                                    )?;
                                }
                            }
                        }
                    }
                    period_courses.push(course.clone());
                }
                print_progress(f, &period_courses, 2)?;
                semester_courses.append(&mut period_courses);
            }
            print_progress(f, &semester_courses, 1)?;
            all_courses.append(&mut semester_courses);
            write!(f, "{EOL}")?;
        }
        print_progress(f, &all_courses, 0)
    }
}
