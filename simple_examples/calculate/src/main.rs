fn sum(a: f32, b: f32) -> f32 {
    a + b
}


fn multiplication(a: f32, b: f32) -> f32 {
    a * b
}

fn division(a: f32, b:f32) -> f32 {
    a / b
}

fn subtracion(a: f32, b: f32) -> f32 {
    a - b
}

fn main() {
   let mut num1: f32 = 10.0;
   let mut num2: f32 = 20.0;

   let mut result = sum(num1, num2);

   println!("{:?}", result);

   num1 = 5.1;
   num2 = 10.9;

   result = multiplication(num1, num2);
   println!("{}", result);


   result = division(num2, num1);
   println!("{:?}", result);

   result = subtracion(num2, num1);
   println!("{:?}", result);


}
