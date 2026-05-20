fn main() {
   let mut balance : f64 = 10000.0;
   let expenses : [f64;6] = [500.0,1250.0,2356.0,50.0,995.0,5025.50];
   let label : [&str;6] = ["wallet recharge","date","therapy","cigarette","fast food","hangout"];

   for expense in expenses {
    let amount_spent = spend(&mut balance,expense);
    if !amount_spent{
        println!("insufficient funds!");
        break;
    }
    println!("Balance : {}",balance);
}

apply_cashback(&mut balance,2.0);
println!("Balance after cashback: {}",balance);
let statement = bank_statement(&expenses,&label);
println!("{}",statement);
}

fn spend(balance : &mut f64, amount:f64) -> bool {
    if *balance >= amount {
        *balance = *balance - amount;
        true
    }else {
        false
    }
}

fn apply_cashback(balance : &mut f64, pct: f64) {
    *balance = *balance + *balance * (pct/100.0);
}

fn bank_statement(expenses : &[f64;6],label : &[&str;6]) -> String{
    let mut total_spent : f64 = 0.0;
    let mut max_spent : f64 = 0.0;
    let mut max_index : usize = 0;

    for i in 0..expenses.len() {
        if expenses[i] > max_spent { max_spent = expenses[i]; max_index = i; }
        total_spent += expenses[i];
    }

    let average_spent = total_spent / (expenses.len() as f64);

    format!("Total spent amount is {total_spent}. Average Spend: {average_spent:.2} Biggest: ₹{max_spent:.2} for {}",label[max_index])
}
