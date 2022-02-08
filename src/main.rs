use std::io;

fn main()
{
    loop{
        println!("__________________");
        println!("enter a number to convert to fehraneit");
        let mut celsuis=String::new();
         io::stdin()
        .read_line(&mut celsuis)
        .expect("failed to read line");
        if celsuis.trim() == "q"
        {
            break;
        }
        let celsuis:f32 = match celsuis.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{println!("please type a number");continue}
        };
        let fehranheit:f32 = (celsuis * (9.0/5.0))+32.0;
        println!("the celsius is {} and the fehranheit is {}",celsuis,fehranheit);
        println!("you can press q for quite");
    }
}