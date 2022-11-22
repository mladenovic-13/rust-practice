use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    lastname: String,
    grades: Vec<u32>,
}
impl Student {
    fn get_final_grade(&self) -> f32 {
        self.grades.iter().sum::<u32>() as f32 / self.grades.len() as f32
    }
}

fn main() {
    println!("Welcome to students service!");

    let mut students: Vec<Student> = Vec::new();
    let mut students_count = String::new();
    println!("How much students you want to enter?");
    io::stdin()
        .read_line(&mut students_count)
        .expect("Failed to read the line!");

    let students_count: u32 = match students_count.trim().parse() {
        Ok(input) => input,
        Err(_) => {
            println!("You must enter a valid number!");
            std::process::exit(1)
        }
    };

    for i in 0..students_count {
        let mut name = String::new();
        let mut lastname = String::new();
        let mut grades: Vec<u32> = Vec::new();
        let mut grades_count = String::new();

        println!("Please enter student's name: ");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read the line!");

        println!("Please enter student's lastname: ");
        io::stdin()
            .read_line(&mut lastname)
            .expect("Failed to read the line!");

        println!("How much grades you want to enter?");
        io::stdin()
            .read_line(&mut grades_count)
            .expect("Failed to read the line!");

        let grades_count: u32 = match grades_count.trim().parse() {
            Ok(count) => count,
            Err(_) => {
                println!("You must enter valid number!");
                break;
            }
        };

        for i in 0..grades_count {
            println!("Enter {}. grade:", i + 1);
            let mut grade = String::new();
            io::stdin()
                .read_line(&mut grade)
                .expect("Failed to read the line!");
            match grade.trim().parse() {
                Ok(input) => grades.push(input),
                Err(_) => {
                    println!("You must enter valid number!");
                    break;
                }
            };
        }

        let student = Student {
            name,
            lastname,
            grades,
        };

        students.push(student);
    }

    println!("This is the list of students:");
    students
        .iter()
        // .enumerate()
        .for_each(|student| {
            println!(
                "Student:{}{}{:#?}Final grade: {}",
                student.name,
                student.lastname,
                student.grades,
                student.get_final_grade()
            )
        })
}
