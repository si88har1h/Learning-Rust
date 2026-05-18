fn main() {
    const ACTIVITY: [bool; 14] = [
        true, true, false, true, true, true, true, true, false, true, true, false, true, true,
    ];

    const DAYS: usize = 14;
    let mut total_commits = 0;
    let mut current_streak = 0;
    let mut longest_streak = 0;

    for day in 0..DAYS {
        if ACTIVITY[day] {
            current_streak += 1;
            total_commits += 1;
        } else {
            current_streak = 0;
        }
        if longest_streak < current_streak {longest_streak = current_streak};

    }

    let completion_rate :f64 = (total_commits as f64 / DAYS as f64) * 100.0;

    println!("Total Commits : {}", total_commits);
    println!("CurrentStreak : {}", current_streak);
    println!("Longest Streak : {}", longest_streak);
    println!("Completion Rate% : {:.2}", (completion_rate));
    print_heatmap(ACTIVITY);
}

fn print_heatmap(activity : [bool;14]) {
    let mut counter = 0;

    while counter < 14 {
       let icon = if activity[counter] { '█'} else { '░'};
       counter+=1;
       print!("{}",icon);
    }
}