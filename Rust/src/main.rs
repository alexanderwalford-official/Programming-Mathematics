#![allow(non_snake_case)]

use std::fmt::Pointer;
use std::io;
use std::io::*;
use std::ptr::null;

fn main() {
    // You can uncomment specific functions:
    // shapes();
    // averages();
    // factorials(56.0);
    // problem_1();
}

fn shapes () {

    // example 1:
    // Task: find the area of a circle

    // solution: pi times by radius squared

    let pi: f64 = 3.141592;
    let radius: f64 = 1.2345; // radius of circle
    let cirarea: f64 = pi * radius * radius; // multiply pi by radius squared
    println!("Area: {}", cirarea);


    // example 2:
    // Task: find the hypotenuse of a right-angled triangle (the missing value of a side)

    // solution: a squared plus b squared equals c squared

    let a: f64 = 1.2345; // side a
    let b: f64 = 2.3456; // side b
    let mut c: f64; // side c (no value, mutable)
    c = a * a + b * b; // a squared plus b squared
    c = c * c; // c squared (final answer)
    println!("Side C: {}", c);


    // example 3:
    // Task: Find the area of triangle

    // solution: height multiplied by width (base), then devide by 2

    let height: f64 = 1.2345;
    let width: f64 = 1.2345;
    let triarea: f64 = height * width / 2.0;
    println!("Area: {}", triarea);


    // example 4:
    // Task: Find the missing angle in a triangle

    // solution: angle a plus angle b, subtract from 180

    let angleA: f64 = 57.65;
    let angleB: f64 = 23.35;
    let angleC: f64 = angleA + angleB - 180.0;
    println!("Angle C: {}", angleC);


    /*
        Secion 2 of shapes; trigonometry
    */


    // example 5:
    // Task: Determine wheather to use sine, cosine or tangent depending on the input variables
    // to find the missing value in a triangle. Let's assume that the angle is located in the top
    // right of a right angled, equalatrial triangle.

    // solution: Sin * opposite / hypotonouse, Cos * adjacent / hypotonouse, Tan * Opposite / adjacent

    let SideA: f64 = 8.0;
    let SideB: f64 = 10.0;
    let SideC: f64 = 14.0;
    let AnglePlacementPos: u32 = 1;
    let mut Answer: f64 = 0.0;

    if AnglePlacementPos == 0 {
        // use Cos
        Answer = SideB / SideC;
        Answer = Answer.cos();
    }
    else if AnglePlacementPos == 1 {
        // use Tan
        Answer = SideB / SideA;
        Answer = Answer.tan();
    }
    else if AnglePlacementPos == 2 {
        // use Sin
        Answer = SideA / SideC;
        Answer = Answer.sin();
    }
    println!("Trig Answer: {}", Answer);    
}

/*
    Averages
    In this section, we'll cover the mean, median and mode averages
    and how you should implement them.
*/

fn averages () {
    // Task: Calculate the mean, mode and median from an array of integers
    // solution:
    // mean = all nums added together devided by amount of nums,
    // median = middle number after list is sorted in linear order,
    // mode = the largest amount of a specific number

    // declare array
    let mut a_x: [i64; 12] = [42, 36, 93, 23, 46, 89, 30, 42, 74, 35, 26, 42];
    let mut last_value = 0;

    let t_6sum: i64;

    // mean
    let xx: i64 = a_x.iter().sum();
    t_6sum = xx / a_x.len() as i64;
    println!("Mean: {}", t_6sum);

    // mode
    // tip: I think that there may be a major flaw with this; does it check values before the current index? probably not
    // how would we fix this?
    let mut a_x_m: [i64; 12] = [0,0,0,0,0,0,0,0,0,0,0,0]; // count for each a_x value
    let pointer: i64 = 0;
    for i in a_x { 
        // if current index pointer value equals the last array value
        if i == a_x[pointer as usize - 1] || i !=0 {
            // increment the value at the current pointer in the counter value array
            a_x_m[pointer as usize] = a_x_m[pointer as usize] + 1;
        }
        // set the value in array 2
        last_value = i;
    }
    
    // sort the array, lowest to largest then the last value is the highest
    a_x_m.sort();
    println!("Mode: {}", a_x_m[12]);

    // median
    a_x_m.sort();
    println!("Median: {}", a_x_m[a_x_m.len() / 2]);
}


    /*
    
        Factorials
        In this section, I'll be using my own custom, simple algorithm
        to find the prime factors of an input cariable.

        The primary focus of this exercise to to consolidate my understanding
        of how to use mathematics in Rust and to illustrate the best possible
        sollution for optimisation.

    */


static mut i_counter: i64 = 0;
static mut i_answer: [i64; 20] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]; // supports up to 20 factorials
static mut i_OriginalNum: f64 = 0.0;

fn factorials (input: f64) {

    /*
        Standard Mathematics:

        To find the prime factorials of a number, all we must do is split
        the number into a multiple of 10(i) and single units. 

        Example:

        Find the prime factorials of 28:

        20              8

        2 x 10          2 x 4
        1 x 5           1 x 2

        Prime factorials: 1, 5, 1, 2


        Computing:

        Computers do not understand the concept of prime factorials, thereore we
        must first define what they are in terms of computer code. 
    */


    unsafe {
        // save the original number
        if i_counter == 0 {
            i_OriginalNum = input;
        }
        if i_counter < 19 {
            let tempnum: f64 = input / 2.0;
            if tempnum.fract() == 0.0 {
                // is a factorial, add it to the array
                i_answer[i_counter as usize] = (tempnum * 100.0).round() as i64;
                i_counter = i_counter + 1;
            }
            else {
                i_counter = i_counter + 1; 
                // is not a factorial, pass back through
                factorials(tempnum);
            } 
        }
        else {
            // maximum array lenth reached!
            println!("Maximum amount of factorials reached!");
            println!("Recorded factorials of {} : ", i_OriginalNum);
            println!("{:?}", i_answer);        
        }
    }
}



fn sqrt () {
    /*
        Square Root Using Bakhshali Method

        Standard Mathematics:
        x = 600
        a = 125348 - 600*2(squared) / 2 x 600 = -195.543
        b = 600 + (-195) = 404.456
        x = 404.456 - (-195.543)*2 (squared) / 2 x 404.456 = 357

        Computing:
    */

    println!("Please enter your value for x:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Illegal character type entered!");

    io::stdin().read_line(&mut input);

    println!("Please enter your value for s:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input)
        .expect("Illegal character type entered!");

    io::stdin().read_line(&mut input2);
    
    // Bakhshali Method
    let s: i64 = input2.parse().unwrap();
    let mut x: i64 = input.parse().unwrap();
    let a: i64 = s - x * x / 2 * x;
    let b: i64 = x + (-a);
    x = b - (-a) * (-a) / 2 * b;
    
    println!(" {}", x);

    // more iterations of this method = more accuracy 

}



    /*
    
        These next secions contain maths problems from the euler archives
        which can be found at: https://projecteuler.net/archives
    
    */


static mut i_problem1_counter: i64 = 0;
static mut i_problem1_answer: i64 = 0;

fn problem_1 () {
    unsafe {
        // Multiples of 3 or 5
        // Task: If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
        // Find the sum of all the multiples of 3 or 5 below 1000.

        // Solution: multiply 3 by an incremental value in a loop until it reaches 1000 multiples

        let iMax: i64 = 1000;
        let iMultiple: i64 = 3;

        if i_problem1_counter < iMax {
            
            let i: i64 = iMultiple * i_problem1_counter;
            println!("{} X {} = {}", iMultiple, i_problem1_counter, i);
            i_problem1_answer = i_problem1_answer + i;
            i_problem1_counter = i_problem1_counter + 1;
            problem_1();
        }
        else {
            println!("The sum of all multiples of {} up to {} is {}", iMultiple, iMax, i_problem1_answer);
        }
    }
}

// set the starting number
static startingnum: i64 = 1;

fn problem_2 (mut cf: usize, mut arr: [i64; 100]) {
    
    // Fibonacci numbers
    // Task: Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
    // 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

    // print all fibonacci numbers up to 4 million
    
    // soltion: 
    arr[0] = startingnum; // set the first value of the array to the static variable

    if (arr[arr.len()] < 400000 && cf != 0) {
        arr[cf] = arr[cf] - 1 + arr[cf]; // add the last and current pointer values
        cf = cf + 1; // increment counter
        problem_2(cf, arr); // recall method, passing the counter and array
    }
    else {
        println!("The Fibonacci numbers are:");
        for i in arr {
            println!("{}", i);
        }
    }
}