use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::io;

struct Employee {
    name: String,
    department: String,
}

impl Display for Employee {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "Employee: {}, Department: {}",
            self.name, self.department
        )
    }
}

/** This function will verify that an employee exists via their name only */
fn exists(name: &str, registry: &HashMap<&str, Vec<Employee>>) -> bool {
    for department in registry.values() {
        for employee in department {
            if employee.name == name {
                return true;
            }
        }
    }
    false
}

/** This function will receive a hash map, which will be our main registry and print all employees in the registry */
fn view_history(registry: &HashMap<&str, Vec<Employee>>) {
    println!("======Registry======");
    for department in registry.values() {
        for employee in department {
            println!("{}", employee);
        }
    }
}

/** In this example we'll do the 3rd exercise of creating a hash map with vectors that represent departments that will have employees in them */
/** We'll choose one option, either registering a new employee in a dept, or viewing the entire registry of employees */
fn main() {
    println!("Hello! Welcome to the IT Registry");
    let mut registry: HashMap<&str, Vec<Employee>> = HashMap::new();
    registry.insert("engineering", vec![]);
    registry.insert("management", vec![]);
    let mut operations = 0;
    loop {
        operations += 1;
        println!("Operations: {}", operations);
        println!("What would you like to do?");
        println!("1. Register new employee");
        println!("2. View Employee History");
        println!("3. Do nothing, Exit");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Invalid option. Try again later.");
        let number = answer.trim().parse().unwrap();
        println!("Number: {}", number);
        match number {
            // TODO: Fix match pattern when entering an option number
            1 => {
                println!("Write the name of the new employee:");
                let mut new_employee = String::new();
                io::stdin()
                    .read_line(&mut new_employee)
                    .expect("Invalid name. Try again later.");
                while exists(&new_employee[..], &registry) {
                    println!("Employee already registered, Try a different employee.");
                    new_employee.clear();
                    io::stdin()
                        .read_line(&mut new_employee)
                        .expect("Invalid name. Try again later.");
                }
                println!("Which dept would you like {} to go?", new_employee);
                println!("1. Engineering");
                println!("2. Management");
                println!("3. Do nothing, Exit");
                let mut selected_dept = String::new();
                io::stdin()
                    .read_line(&mut selected_dept)
                    .expect("Invalid dept. Try again later.");
                let dept_option: i32 = selected_dept.trim().parse().unwrap();
                match dept_option {
                    1 => {
                        let engineering = registry.entry("engineering").or_insert(vec![]);
                        engineering.push(Employee {
                            name: String::from(new_employee),
                            department: String::from("engineering"),
                        });
                    }
                    2 => {
                        let management = registry.entry("management").or_insert(vec![]);
                        management.push(Employee {
                            name: String::from(new_employee),
                            department: String::from("management"),
                        });
                    }
                    3 => break,
                    _ => (),
                }
            }
            2 => view_history(&registry),
            3 => break,
            val => {
                println!("Selected option: {}", answer);
                println!("Selected option equals \"1\": {}", val == 1);
            }
        }
    }
}
