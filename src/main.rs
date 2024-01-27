/* 
    Collatz Conjecture - develop by Dinnerb0ne, is a software to iterate the Collatz conjecture
    Copyright (C) 2024  Dinenrb0ne (Dinnerb0ne02)

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.    
    The Auther's email: tomma_2022@outlook.com

*/

use std::io;
// todo : use std::fs;
use std::time;

fn main() {

    print_info();
    // let mode_value = mode_confirm();

    let min_value = enter_minimum_value();
    let max_value = enter_maximum_value();

    if min_value < max_value && 0 < min_value {

    print!("\n[notification] The minimum value you entered: {1:?}\n[notification] The maximum value you entered: {0:?}\n",max_value,min_value);
    
    let start_time = time::Instant::now();

    // let output_test_2n_a = Computation::c_2n(&min_value);
    // let output_test_3np1_a = Computation::c_3np1(&max_value);

    print!("\n[notification] Starting now...\n\n");

    
        
    let mut incr_val = 0;
    let mut judg_val = 0;

    print!("[debug] judg_val = {0:?}\n",judg_val);
    print!("[debug] incr_val = {0:?}\n", incr_val);

    print!("\n\noutput:\n\n");
    
    while min_value + incr_val <= max_value {
        let mut median_num_cal = min_value + incr_val;
        print!("{0:?}", median_num_cal);
    
        while median_num_cal > 1 {
            let median_num_cal_static;
            judg_val = median_num_cal % 2;
            if judg_val == 0 {
                median_num_cal_static = Computation::c_2n(&median_num_cal);print!(", {0:?}", median_num_cal_static);
            } else {
                median_num_cal_static = Computation::c_3np1(&median_num_cal);print!(", {0:?}", median_num_cal_static);
            }
            median_num_cal = median_num_cal_static;
        }
        incr_val = incr_val + 1;
        print!(".\n\n");
    }

    print!("\n\n");    

    print!("[notification] Completed!\n");

    // print!("\n[test] output_test_2n_a = {0:?}\n[test] output_test_3np1_a = {1:?}\n",output_test_2n_a,output_test_3np1_a);

    let duration = start_time.elapsed();
    print!("[notification] Time spent: {1:?} (s)    {0:?} (ms)",duration.as_millis(),duration.as_secs());

    } else {
    let start_time = time::Instant::now();
    print!("\n[Error] Invalid value: {1:?} {0:?}\n",max_value,min_value);

    let duration = start_time.elapsed();
    print!("\n[notification] The time spent: {1:?} (s)    {0:?} (ms)",duration.as_millis(),duration.as_secs());}  

    press_any_key_to_exit();

}

fn enter_minimum_value() -> i32 {
    let mut min_number = String::new();
    print!("[input] Please enter the minimum value: \n");
    
    io::stdin()
        .read_line(&mut min_number)
        .expect("[Error] Failed to read line");

    let min_number: i32 = min_number
        .trim()
        .parse()
        .expect("[Error] Failed to parse number");

    min_number
}

fn enter_maximum_value() -> i32 {
    let mut max_number = String::new();
    print!("[input] Please enter the maximum value: \n");
    
    io::stdin()
        .read_line(&mut max_number)
        .expect("[Error] Failed to read line");

    let max_number: i32 = max_number
        .trim()
        .parse()
        .expect("[Error] Failed to parse number");

    // print!("You Entered: {} \n", max_number);

    max_number
}

fn press_any_key_to_exit(){
    print!("\n\n");
    let mut any_key = String::new();
    print!("Press any key to exit...\n");
    io::stdin()
        .read_line(&mut any_key)
        .expect("[Error] Failed to read line");
}

struct Computation {}

impl Computation{
fn c_2n(median_val: &i32) -> i32 {
    let median_val = median_val /2 ;
    median_val
}

fn c_3np1(median_val: &i32) -> i32 {
    let median_val = median_val *3 + 1 ;
    median_val
}
}

fn print_info(){
    print!(r"  ____      _ _       _       
 / ___|___ | | | __ _| |_ ____
| |   / _ \| | |/ _` | __|_  /
| |__| (_) | | | (_| | |_ / / 
 \____\___/|_|_|\__,_|\__/___|         v0.1 release \n");
    
    print!("Auther: Dinnerb0ne\n");
    print!("Email: tomma_2022@Outlook.com\n");
    print!("Github: Dinnerb0ne02 <tomma_2022@Outlook.com>\n");
    print!("Collatz conjecture is a software to iterate the Collatz conjecture\n\n");
}
// fn write_file(input: &String){}


/* 
fn mode_confirm() -> String {
    let mut mode_value = String::new();
    print!("Please enter the Mode value [default]: \n");
    // four mode : default, noFileOutput, noTerminalOutput, neither
    // only default mode is valid
    io::stdin()
        .read_line(&mut mode_value)
        .expect("Failed to read line");
    //

    mode_value
}
 */