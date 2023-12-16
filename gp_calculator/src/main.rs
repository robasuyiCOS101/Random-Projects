use std::io;
fn main()
{
    let mut subjects:Vec<String> = Vec::new();
    let mut course_unit:Vec<i32> = Vec::new(); 
    let mut grade:Vec<String> = Vec::new();
    println!("welcome to the GP calculator");
    println!("How many subjects are you offering?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let number_of_subjects:i32 = input1.trim().parse().expect("Failed to read input");

    for count in 0..number_of_subjects
    {
        println!("What is the name of the subject {} ?", count+1);
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_subjects:String = input2.trim().to_string();
        subjects.push(new_subjects);

        println!("What is the course unit for subject {} ?", count+1);
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let new_course_unit:i32 = input3.trim().parse().expect("Failed to read input");
        course_unit.push(new_course_unit);


        println!("what was the grade for subject {}", count+1);
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input"); 
        let new_grade:String = input4.trim().to_string();
        grade.push(new_grade);
    }

        let mut subject_score = 0;
        for i in 0..number_of_subjects as usize
        {

        let grade = &grade[i];
        let course_unit = course_unit[i];
        
            match grade.as_str(){
                "a" => subject_score +=5 * course_unit,
                "b" => subject_score +=4 * course_unit,
                "c" => subject_score +=3 * course_unit,
                "d" => subject_score +=2 * course_unit,
                "f" => subject_score +=1 * course_unit,
                _ => println!("Input the correct Grade ranging from a - f"),


            }
        }
         
        let course_unit_sum:i32 = course_unit.iter().sum();
        let gp_formulae:f64 = subject_score as f64 / course_unit_sum as f64;
        println!("Your GP is {:.2}",gp_formulae);

        if gp_formulae >= 4.5{
            println!("Congratulations you are a First class student!");
        }else if gp_formulae = 3.5 && gp_formulae >= 4.49{
            println!("Congratulations you are a second class upper student ");
        } else if gp_formulae <= 2.5 && gp_formulae >= 3.49{
            println!("Congratulations you are a second class lower student");
        }else if gp_formulae = 1.50 && gp_formulae >= 2.49{
            println!("Congratulations you are a third class student ");
        }else {
            println!("You have failed ");
        }

        

}