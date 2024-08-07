 
pub mod operations;
fn main(){
    loop {
        //taking operation 
        println!("Calculator");
        println!("Enter the operation (+,-,*,/): ");
        let opr_input = &mut String::from("");
        std::io::stdin().read_line(opr_input).unwrap();
        let opr = opr_input.replace("\n", "");
        
        //taking num1
        println!("Enter Num1: ");
        let num1_input = &mut String::from("");
        std::io::stdin().read_line(num1_input).unwrap();
        let num1 = num1_input.replace("\n", "").parse::<f32>().unwrap();
    
        //taking num2
        println!("Enter Num2: ");
        let num2_input = &mut String::from("");
        std::io::stdin().read_line(num2_input).unwrap();
        let num2 = num2_input.replace("\n", "").parse::<f32>().unwrap();


        if opr == "+"{
            let result: f32 = operations::add(num1, num2);
            println!("{}",result)
        }
        else if opr == "-"{
            let result: f32 = operations::sub(num1, num2);
            println!("{}",result)
        }
        else if opr == "*"{
            let result = operations::xpl(num1, num2);
            println!("{}",result)
        }
        else if opr == "/"{
            let result = operations::div(num1, num2);
            println!("{}",result)
        }
        else{
            println!("Unknow Error Occured, Please check your input")
        }


    }
   

}