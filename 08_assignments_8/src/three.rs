use crate::io::read_input;
use std::collections::HashMap;

pub fn execute() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let option = read_input::read_number_input(
            "Please choose an option:\n
        1. Add Employee to Department\n
        2. Retrieve Employees in Department\n
        3. Retrieve All Employees\n
        4. Quit",
        );

        println!("You chose option: {}", option);

        match option {
            1 => add_employee_to_department(&mut employees),
            2 => retrieve_employees_from_department(&employees),
            3 => retrieve_all_employees(&employees),
            _ => break,
        }
    }
}

fn add_employee_to_department(employees: &mut HashMap<String, Vec<String>>) {
    let (employee, department) = read_input::read_string_tuple(
        "Please enter an employee using the pattern: '<employee name' to <department>':",
        "to",
    );

    if employees.contains_key(&department) {
        if let Some(department_employees) = employees.get_mut(&department) {
            department_employees.push(employee);
            department_employees.sort();
        }
    } else {
        let department_employees: Vec<String> = vec![employee];
        employees.insert(department, department_employees);
    }
}

fn retrieve_employees_from_department(employees: &HashMap<String, Vec<String>>) {
    let department = read_input::read_string_input("Please enter the department name:");

    match employees.get(&department) {
        Some(department_employees) => {
            for employee in department_employees {
                println!("{}", employee);
            }
        }
        None => {
            println!(
                "There is no department named {}. These are the available values:",
                department
            );
            for department in employees.keys() {
                println!("{}", department);
            }
        }
    }

    // if employees.contains_key(&department) {
    //     println!("Department: {}", department);
    //     if let Some(department_employees) = employees.get(&department) {
    //         for employee in department_employees {
    //             println!("{}", employee);
    //         }
    //     }
    // } else {
    //     println!("There is no department named {}", department);
    // }
}

fn retrieve_all_employees(employees: &HashMap<String, Vec<String>>) {
    for (department, department_employees) in employees {
        println!("Department: {}", department);
        for employee in department_employees {
            println!("{}", employee);
        }
    }
}
