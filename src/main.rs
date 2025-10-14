fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello, test!");
}

#[test]
fn test_variable(){
    let name = "Haikal Nuril";
    println!("Hello, {}", name);
}

#[test]
fn test_mutable_variable(){
    let mut name = "Haikal Nuril";
    println!("Hello, {}", name);
    name = "Uriel";
    println!("Hello, {}", name);
}

#[test]
fn shadowing_variable(){
    let name = "Haikal Nuril";
    println!("Hello, {}", name);
    let name = 10;
    println!("Hello, {}", name);
}

#[test]
fn explicit(){
    let age: i32 = 20;
    println!("Age: {}", age);
}

#[test]
fn number_conversion(){
    let age: i32 = 20;
    println!("Age: {}", age);
    let age2: i64 = age as i64;
    println!("Age: {}", age2);
    let age3: i128 = age2 as i128;
    println!("Age: {}", age3);

    //integer overflow
    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("e: {}", e); //e: 0 (because of overflow)
}

#[test]
fn augmented_assignment(){
    let mut age = 20;
    println!("Age: {}", age);
    age += 1;
    println!("Age: {}", age);
    age -= 1;
    println!("Age: {}", age);
    age *= 2;
    println!("Age: {}", age);
    age /= 2;
    println!("Age: {}", age);
    age %= 3;
    println!("Age: {}", age);
}

#[test]
fn char_type(){
    let letter = 'A';
    println!("Letter: {}", letter);
    let emoji = '😀';
    println!("Emoji: {}", emoji);
}

#[test]
fn tuple(){
    let person: (&str, i32) = ("Haikal Nuril", 20);
    println!("Name: {}, Age: {}", person.0, person.1);
    println!("{:?}", person);
    let (name, age) = person;
    println!("Name: {}, Age: {}", name, age);

    let mut person2 = ("Haikal Nuril", 20);
    person2.0 = "Uriel";
    person2.1 = 21;
    println!("Name: {}, Age: {}", person2.0, person2.1);
}

// unit tuple
fn unit(){
    println!("hello");
}

#[test]
fn test_unit(){
    let result: () = unit();
    println!("Result: {:?}", result);

    let test: () = ();
    println!("Test: {:?}", test);
}

#[test]
fn array(){
    let array: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", array);
    println!("Array[0]: {}", array[0]);
    println!("Array length: {}", array.len());

    let mut array: [i32; 3] = [1, 2, 3];
    array[0] = 10;
    println!("Array: {:?}", array);
}

const MAX: i32 = 100;

#[test]
fn constant(){
    const MIN: i32 = 0;
    println!("MAX: {}", MAX);
    println!("MIN: {}", MIN);
}

#[test]
fn variable_scope(){
    let name = "Haikal Nuril";
    println!("Hello, {}", name);
    {
        let name = "Uriel";
        println!("Hello, {}", name);
        let age = 20;
    }
    println!("Hello, {}", name);
    // println!("Age: {}", age); //error: age is not found
}

#[test]
fn stack_and_heap(){
    function_a();
    function_b();
}

fn function_a(){
    let a = 10;
    let b = String::from("haikal");
    println!("a: {}, b: {}", a, b);
}

fn function_b(){
    let a = 10;
    let b = String::from("ril");
    println!("a: {}, b: {}", a, b);
}

#[test]
fn string(){
    let name: &str = " Haikal Nuril  "; //stack
    let trim: &str = name.trim();

    println!("Name: {}", name);
    println!("Trim: {}", trim);

    let mut username = "Eko";
    username = "Budi";
    println!("Username: {}", username);
}

#[test]
fn string_type(){
    let mut name: String = String::from("Haikal Nuril"); //heap
    name.push_str(" Abiyit");
    println!("Name: {}", name);

    let budi = name.replace("Nuril", "Budi");
    println!("Budi: {}", budi);
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 20;

        println!("b: {}", b);
    }
    // println!("a: {}", b); cant access b here 
    println!("a: {}", a);
}

#[test]
fn ownership_movement() {
    let name = String::from("Haikal Nuril"); //why cant move? cause its store in heap. so its not implement copy trait data

    // ownership moved to name2
    let name2 = name;
    // println!("name: {}", name); cant access name here
    println!("name2: {}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("Haikal Nuril");
    let name2 = name1.clone(); // deep copy, this will make big performance issue if the data is too big cause if the first data 100 mb it will copy all the data to the new variable

    println!("name1: {}", name1);
    println!("name2: {}", name2);
}