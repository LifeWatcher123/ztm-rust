// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct StudentLocker {
    student_name: String,
    locker_num: Option<i32>,
}

fn main() {
    let student_locker = StudentLocker {
        student_name: "Maria".to_owned(),
        locker_num: None,
    };

    match student_locker.locker_num {
        Some(locker_num) => {
            println!(
                "Student Name: {:?} | Locker: {:?}",
                student_locker.student_name, locker_num
            )
        }
        None => {
            println!(
                "Student Name: {:?} | Locker: None",
                student_locker.student_name
            )
        }
    }
}
