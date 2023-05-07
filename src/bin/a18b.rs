// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Position {
    Maintenance,
    Marketing,
    Managers,
    Line,
    Kitchen,
    Assembly,
}

enum Status {
    Employed,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn building_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Employee terminated!".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Managers => Ok(()),
        _ => Err("Invalid position".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = building_access(employee)?;
    println!("Access granted!");
    Ok(())
}

fn main() {
    let employee_1 = Employee {
        position: Position::Assembly,
        status: Status::Employed,
    };
    let employee_2 = Employee {
        position: Position::Managers,
        status: Status::Terminated,
    };

    match print_access(&employee_2) {
        Err(e) => println!("{:?} Access denied!", e),
        _ => (),
    }
}
