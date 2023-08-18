use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum AgeGroup {
    Young,
    MiddleAged,
    Senior,
}

#[derive(Debug)]
struct Employee {
    employee_id: u32,
    employee_name: String,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    fn new(
        id: u32,
        name: String,
        email: String,
        age: u32,
        phone: String,
    ) -> Self {
        Employee {
            employee_id: id,
            employee_name: name,
            email,
            age,
            phone_number: phone,
        }
    }

    fn age_group(&self) -> AgeGroup {
        if self.age <= 30 {
            AgeGroup::Young
        } else if self.age <= 60 {
            AgeGroup::MiddleAged
        } else {
            AgeGroup::Senior
        }
    }
}

fn main() {
    let mut employees: HashMap<u32, Employee> = HashMap::new();

    loop {
        println!("Enter employee details:");
        println!("Employee ID:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let employee_id: u32 = input.trim().parse().expect("Invalid input");

        if employee_id == 0 {
            break;
        }

        println!("Employee Name:");
        let mut employee_name = String::new();
        io::stdin()
            .read_line(&mut employee_name)
            .expect("Failed to read line");

        println!("Email:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read line");

        println!("Age:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let age: u32 = input.trim().parse().expect("Invalid input");

        println!("Phone Number:");
        let mut phone_number = String::new();
        io::stdin()
            .read_line(&mut phone_number)
            .expect("Failed to read line");

        let employee =
            Employee::new(employee_id, employee_name, email, age, phone_number);
        employees.insert(employee_id, employee);
    }

    println!("Enter employee ID to search:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let target_id: u32 = input.trim().parse().expect("Invalid input");

    if let Some(employee) = employees.get(&target_id) {
        println!("Employee found: {:?}", employee);
        println!("Age Group: {:?}", employee.age_group());
    } else {
        println!("Employee not found.");
    }
}
