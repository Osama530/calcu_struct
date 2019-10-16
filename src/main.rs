use std :: io;
#[derive(Debug)]


struct Calculate { //struct for calculate
    val_1:f32, //1st value or numrator or base
    val_2:f32, //2nd value or denominator or power
}

impl Calculate { //implimentaion block for calulating functions
    //fnuction for addition
    fn add(&self)->f32{
        self.val_1+self.val_2
    }
    //function for subtraction
    fn sub(&self)->f32{
        self.val_1-self.val_2
    }
    //function for multiplication
    fn mul(&self)->f32{
        self.val_1*self.val_2
    }
    //function for division
    fn div(&self)->f32{
        self.val_1/self.val_2
    }
    //function for exponent
    fn exp(&self)->f32{
        let ans = self.val_1.powf(self.val_2);
        ans
    }
} 

fn main() {
        loop {  
        println!("1) addition");
        println!("2) subtraction");
        println!("3) multiplication");
        println!("4) division");
        println!("5) exponent");
        println!("0) quit");
        println!("Enter the fuction you wish to perform");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        let input:i32 =input.trim().parse().unwrap();
        //let input:String = input.trim().parse().unwrap();
        
    if input ==0 {
        break;
    }
    else {    
        println!("Enter 1st value: ");
        let mut num_1 = String::new();
        io::stdin().read_line(&mut num_1).ok();
        let num_1:f32 = num_1.trim().parse().unwrap();

        println!("Enter 1st value: ");
        let mut num_2 = String::new();
        io::stdin().read_line(&mut num_2).ok();
        let num_2:f32 = num_2.trim().parse().unwrap();
            
        match input {
                1=>println!("{}",Calculate{val_1:num_1,val_2:num_2}.add()),
                2=>println!("{}",Calculate{val_1:num_1,val_2:num_2}.sub()),
                3=>println!("{}",Calculate{val_1:num_1,val_2:num_2}.mul()),
                4=>println!("{}",Calculate{val_1:num_1,val_2:num_2}.div()),
                5=>println!("{}",Calculate{val_1:num_1,val_2:num_2}.exp()),
                _=>println!("Undefined function entered"),
            };  
        };
    };
}