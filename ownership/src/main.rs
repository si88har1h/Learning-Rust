// fn main() {
//     let s = String::from("Hello"); // s comes into scope here
//                                    // available here too
//     take_ownership(s);             // transfered to take_ownership function 
//                                    // s not available her anymore 
//     println!("{}",s);

//     let x = 5;  // comes into scope

//     print_numeral(x); // its a static data type so uses Copy trait underneath
//                       // still available  
// } // x goes out of scope here

// fn take_ownership(some_string : String){
//     println!("{}",some_string);
// } // scope ends and the string argument 'drops' and memory is freed

// fn print_numeral(num : i32){
//     println!("{}",num);
// }

// fn main() {
//     let s1 = gives_ownership();        // gives_ownership moves its return
//                                        // value into s1

//     let s2 = String::from("hello");    // s2 comes into scope

//     let s3 = takes_and_gives_back(s2); // s2 is moved into
//                                        // takes_and_gives_back, which also
//                                        // moves its return value into s3
//     println!("{}",s2);
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {       // gives_ownership will move its
//                                        // return value into the function
//                                        // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                        // some_string is returned and
//                                        // moves out to the calling
//                                        // function
// }

// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

fn main(){
    let team_1 : String = String::from("CSK");
    let team_2 : String = String::from("RCB");

    let team_1_runs : [u32;5] = [25,36,10,0,5];

    const WICKET_COUNT : u32 = 5;

    let (total_runs,run_rate) = calculate_runs_and_run_rate(team_1.clone(),team_2,team_1_runs);

    let summary = final_summary(team_1,team_1_runs,total_runs,WICKET_COUNT,run_rate);

    println!("{}",summary);
}

fn calculate_runs_and_run_rate(team1: String, team2: String, team_1_runs : [u32;5]) -> (u32,f64) {
    println!(">>> Innings: {0} vs {1}",team1,team2);
    let mut runs : u32 = 0;
    for run in team_1_runs {
        runs +=run;
    }
    let run_rate : f64 = runs as f64 / team_1_runs.len() as f64;

    (runs,run_rate)    
}

fn final_summary(team : String, team_runs : [u32;5], total_runs : u32, wickets : u32, run_rate : f64) -> String{
    let total_overs = team_runs.len();
    let s = format!("{team} posted {total_runs}/{wickets} in {total_overs} overs at RR {run_rate}");

    s
}