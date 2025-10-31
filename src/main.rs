use std::{array, result};
mod model;
mod third;

use model::user::User;

fn main() {
    let user = User{
        first_name: String::from("Eko"),
        last_name: String::from("Kaneddy"),
        username: String::from("eko"),
        email: String::from("eko@example.com"),
        age: 20
    };

    user.say_hello("Budi");
    println!("Hello, world!");

    model::user::second::say_bye();
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

#[test]
fn let_statement() {
    let value = 7;
    let result: &str;

    // in rust the if statement is an expression that can return a value so that create a let statement
    if value >= 8 {
        result = "good";
    } else {
        result = "bad";
    }

    // this is the same as above but more concise

    // let result: &str = if value >= 8 {
    //     "good"
    // } else {
    //     "bad"
    // };

    println!("result: {}", result);
}

#[test]
fn loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2; // return value from loop
        }
    };

    println!("result: {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut counter = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!(" {} x {} = {}", number, counter, number * counter);
            counter += 1;
            if counter > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 3] = ["Haikal", "Nuril", "Abiyit"];

    for value in array{
        println!("value: {}", value);
    }
}

#[test]
fn range_iteration() {
    let array:[i32; 5] = [1,2,3,4,5];

    let range = 0..5;

    for i in range{
        println!("array[{}]: {}", i, array[i]);
    }
}

#[test]
fn range_inclusive() {
    let array:[i32; 5] = [1,2,3,4,5];

    let range = 0..=4;

    for i in range{
        println!("array[{}]: {}", i, array[i]);
    }
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[test]
fn test_factorial_function_return_value() {
    let result = factorial_loop(5);
    println!("Factorial: {}", result);
}

fn factorial_recursive(n:i32) -> i32 {
    if n < 1 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return n * factorial_recursive(n - 1);
    }
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("Factorial: {}", result);
}


// ownership in function

fn print_number(number: i32){
    println!("number: {}", number)
}

fn hi(name: String) {
    println!("name {}", name)
}

#[test]
fn test_hi() {
    let number = 10; // if the function have parameter that type for saving in stack, it will copy the value in stack
    print_number(number);
    println!("number {}",number); // so we still can calling the variable in here

    let name = String::from("eko"); // if the function have parameter that type for saving in heap it will move the ownership from variable to the function
    hi(name)
    // println!("name {}", name) // so that why this cant be called because no more name in heap
}

//return value ownership

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("nuril");
    let last_name = String::from("abiyit");

    let name = full_name(first_name, last_name);
    println!("{}", name);
    // println!("{}", first_name); cant be executed because no more value in that variable
    // println!("{}", last_name);
}

// so how we can get back the ownership from the function

fn full_name_returner(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name_returner() {
    let first_name = String::from("nuril");
    let last_name = String::from("abiyit");

    let (first_name, last_name, name) = full_name_returner(first_name, last_name);
    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name); 
}


// References and Borrowing

fn references_full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn references_test() {
    let first_name = String::from("nuril");
    let last_name = String::from("abiyit");

    let full_name = references_full_name(&first_name, &last_name);
    println!("{}", full_name)
}

// rules borrowing
// 1. cant change value that we reference even it mutable (default)
// if want to change the value we can make it &mut in reference and just one mutable reference that allowed in one time

fn change_value(value: &mut String) {
    value.push_str("test")
}

#[test]
fn test_change_value() {
    let mut value = String::from("Nuril");
    change_value(&mut value);
    change_value(&mut value);
    println!("{}", value)

    //this is that not allowed with rust because it have 2 mutable reference in one cycle life
    // let valueBorrow1 = &mut value;
    // let valueBorrow2 = &mut value;

    // change_value(valueBorrow1);
    // change_value(valueBorrow2);
}

// Dangling Pointer Solution
fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("nuril");
    let last_name = String::from("abiyit");

    let full_name = get_full_name(&first_name, &last_name);
    println!("{}", full_name)
}

//slicing
#[test]
fn slice_reference() {
    let array: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);
}


//struct
struct Person {
    first_name: String,
    middle_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.age);
}

#[test]
fn struct_person() {
    let first_name = String::from("Haikal");
    let middle_name = String::from("Nuril");
    let person: Person = Person {
        first_name,
        middle_name,
        age: 21,
    };

    // println!("{}", first_name); this is error cause the first_name ownership was move into person.first_name
    print_person(&person);

    // Struct Update Syntax
    let person2: Person = Person{..person};

    print_person(&person2); 
    // but this have a problem if we just do this, cause it will transfer ownership from person to person2 if any field have types that save in heap
    
    // solution is
    let person3: Person = Person{
        first_name: person2.first_name.clone(),
        middle_name: person2.middle_name.clone(),
        age: person2.age.clone(),
    };

    print_person(&person3);
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name)
    }
}

#[test]
fn test_method() {
    let first_name = String::from("Haikal");
    let middle_name = String::from("Nuril");
    let person: Person = Person {
        first_name,
        middle_name,
        age: 21,
    };

    person.say_hello("agus");
}

//enum

enum Level {
    Regular,
    Premium,
    Platinum
}

#[test]
fn test_enum() {
    let level: Level = Level::Regular;

    match level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    Ewallet(String, String)
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount)
            }
            Payment::BankTransfer(bank, number ) => {
                println!("Paying with Bank Transfer {} {} amount {}", bank, number, amount)
            }
            Payment::Ewallet(wallet, number ) => {
                println!("Paying with Ewallet {} {} amount {}", wallet, number, amount)
            }
        }
    }
}
#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("12341234"));
    _payment1.pay(340000);
    let _payment2: Payment = Payment::BankTransfer(String::from("BCA"), String::from("12341234"));
    let _payment3: Payment = Payment::Ewallet(String::from("Gopay"), String::from("12341234"));
}

#[test]
fn test_match_value() {
    let name = "joko";

    match name {
        "Eko" => {
            println!("HELLO EKO");
        }
        other => {
            println!("Hello {}", other)
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 100;

    match value {
        75..=100 => {
            println!("Great");
        }
        50..=74 => {
            println!("Good");
        }
        25..=49 => {
            println!("Not Bad");
        }
        0..=24 => {
            println!("Bad");
        }
        other => {
            println!("invalid value {}", other);
        }
    }
}


#[test]
fn match_destructuring() {
    let person = Person {
        first_name: String::from("Uriel"),
        middle_name: String::from("Kun"),
        age: 21
    };
    
    match person {
        Person {first_name, middle_name, ..} => {
            println!("First Name: {}, Middle Name: {}", first_name, middle_name)
        }
    }
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age
}

#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("321321"),
        name: String::from("Eko"),
        age: 20
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

trait CanSayHello{
    fn hello(&self)-> String {
        String::from("Hello")
    }

    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}
trait CanSayGoodbye{
    fn say_good_bye(&self) -> String;
    fn say_good_bye_to(&self, name: &str) -> String;
}

impl CanSayHello for Person  {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.first_name)
    }
}

impl CanSayGoodbye for Person  {
    fn say_good_bye(&self) -> String {
        format!("Goodbye from {}", self.first_name)
    }

    fn say_good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {} from {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello())
}

fn say_hello_and_good_bye_trait(value: &(impl CanSayHello + CanSayGoodbye) ) {
    println!("{}", value.say_hello());
    println!("{}", value.say_good_bye());
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Eko"),
        middle_name: String::from("Eko"),
        age: 20
    };

    say_hello_trait(&person);
    say_hello_and_good_bye_trait(&person);

    let result = person.say_hello_to("Budi");
    println!("{}", result);

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Budi");
}

struct SimplePerson {
    name: String
}

impl CanSayGoodbye for SimplePerson{
    fn say_good_bye(&self) -> String {
        format!("Goodbye from {}", self.name)
    }

    fn say_good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, from {}", name, self.name)
    }
}

fn create_person(name:String) -> impl CanSayGoodbye {
    SimplePerson { name }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Eko"));
    println!("{}", person.say_good_bye())
}


// inharitence trait or Super trait
trait CanSay: CanSayHello + CanSayGoodbye {

}

struct SimpleMan {
    name: String,
}

impl CanSayHello for SimpleMan  {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello {}, my name is {}", name, self.name)
    }
}

impl CanSayGoodbye for SimpleMan  {
    fn say_good_bye(&self) -> String {
        format!("Goodbye from {}", self.name)
    }

    fn say_good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {} from {}", name, self.name)
    }
}

impl CanSay for SimpleMan {

}

struct Point <T> {
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> {
        x: 10,
        y: 20,
    };

    println!("Point integer: {}, {}", integer.x, integer.y);
}

enum Value <T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10);
    match value {
        Value::NONE => {
            println!("No Value")
        }
        Value::VALUE(v) => {
            println!("Value: {}", v)
        }
    }
}

struct Hi<T: CanSayGoodbye> {
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Eko"),
        },
    };

    println!("{}", hi.value.say_good_bye());
}

fn min <T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn test_generic_function() {
    let result = min(10, 20);
    println!("Min: {}", result);
}

impl <T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_method() {
    let point = Point{x: 10, y: 20};
    println!("x: {}, y: {}", point.get_x(), point.get_y());
}

// trait GetValue<T> {
//     fn get_value(&self) -> &T;
// }

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}

impl <T> GetValue<T> for Point<T> where T: PartialOrd {
    fn get_value(&self) -> &T {
        &self.x
    }
}

