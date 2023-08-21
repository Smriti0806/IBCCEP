use std::io;
struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}
impl Student 
{
    fn get_details(&self) 
    {
        println!("Student ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("Phone: {}", self.phno);
    }
}
fn main() 
{
    let mut students: Vec<Student> = Vec::new();
    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phno: String::from("123-456-7890"),
        id: 1,
    });
    students.push(Student {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phno: String::from("987-654-3210"),
        id: 2,
    });
    students.push(Student {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        phno: String::from("555-555-5555"),
        id: 3,
    });
    students.push(Student {
        name: String::from("David"),
        email: String::from("david@example.com"),
        phno: String::from("111-222-3333"),
        id: 4,
    });
    students.push(Student {
        name: String::from("Eve"),
        email: String::from("eve@example.com"),
        phno: String::from("444-444-4444"),
        id: 5,
    });
    println!("Enter student ID:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let student_id_to_find: u32 = input.trim().parse().expect("Invalid input");
    match students.iter().find(|student| student.id == student_id_to_find) {
        Some(student) => {
            student.get_details();
        }
        None => {
            println!("Student not found.");
        }
    }
}
