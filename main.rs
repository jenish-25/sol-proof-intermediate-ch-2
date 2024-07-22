struct Employee{
    name:String,
    salary:u32,
    id:u32,
    employee_type:EmployeeType,
}
enum EmployeeType{
    JuniorDev,
    SeniorDev,
}

impl Employee{
    fn new(name:String,id:u32,employee_type:EmployeeType)->Employee{
        Employee{
        name,
        salary:0,
        id,
        employee_type,
        }
    }

    fn asisgn_salary(&mut self){
        self.salary=match self.employee_type{
            EmployeeType::JuniorDev => 50000,
            EmployeeType::SeniorDev => 60000, 
        }
    }
}
fn main(){
    let mut junior=Employee::new("dhruv".to_string(),1,EmployeeType::JuniorDev);
    junior.asisgn_salary();

    let mut senior=Employee::new("jenish".to_string(),2,EmployeeType::SeniorDev);
    senior.asisgn_salary();

    let mut newemp=Employee::new("deep".to_string(),3,EmployeeType::JuniorDev);
    newemp.asisgn_salary();

    println!("Employee: {}, ID: {}, Type: Junior Engineer, Salary: {}", junior.name, junior.id, junior.salary);
    println!("Employee: {}, ID: {}, Type: Senior Engineer, Salary: {}", senior.name, senior.id, senior.salary);
    println!("Employee: {}, ID: {}, Type: Senior Engineer, Salary: {}", newemp.name, newemp.id, newemp.salary);

}