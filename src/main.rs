use std::{ io, thread, time::Duration, collections::HashMap, sync::{ Arc, Mutex } };
use rand::Rng;

fn main() {
    println!(" ------------------------------------");
    println!("|                                    |");
    println!("|   Welcome to Go Horsey, Go Run!    |");
    println!("|                                    |");
    println!(" ------------------------------------");
    println!();

    let origin: i32 = 1;
    let bound: i32 = 5 + 1;

    println!("Select your Horsey ({} - {}): ", origin, bound - 1);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let horsey: i32 = input.trim().parse::<i32>().expect("Panic.");
    let mut is_in_bound: bool = false;

    for i in origin..bound {
        if horsey == i {
            is_in_bound = true;
            break;
        }
    }

    if is_in_bound {
        println!("Your Horsey is: {}", input);

        let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut race: HashMap<i32, &str> = HashMap::new();

        let condition: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let finished: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

        for i in origin..bound {
            let mut scores: HashMap<i32, i32> = map.clone();
            let mut races: HashMap<i32, &str> = race.clone();

            let mut cond: Arc<Mutex<bool>> = condition.clone();
            let mut fin: Arc<Mutex<i32>> = finished.clone();

            threads.push(thread::spawn(move || {
                for j in 1..11 {
                    let rng: i32 = rand::thread_rng().gen_range(1..3);

                    let val = scores.get(&i).copied().unwrap_or(0);
                    let dash = if val >= 10 { 12 } else { val + rng };

                    let mut winner: std::sync::MutexGuard<'_, bool> = cond.lock().unwrap();
                    let mut position: std::sync::MutexGuard<'_, i32> = fin.lock().unwrap();

                    let mut line: String = "".to_string();
                    for l in 0..10 {
                        if l == dash - 1 {
                            line.push_str("*");
                        } else if dash >= 10 && l == 9 {
                            line.push_str("*");
                        } else {
                            line.push_str("-");
                        }
                    }
                    if dash <= 11 {
                        if dash == 10 || dash == 11 {
                            *position += 1;
                            println!("Horse {}: {} | Finished at position {}!", i, line, *position);
                        } else {
                            println!("Horse {}: {}", i, line);
                        }
                    }

                    if !*winner && dash >= 10 {
                        *winner = true;
                        
                        if i == horsey {
                            println!("Horse {} has won! Your Horsey's a winner winner chicken dinner!", i);
                        } else {
                            println!("Horse {} has won! Your Horsey's kind of a loser.", i);
                        }
                    }
                    scores.insert(i, dash);

                    thread::sleep(Duration::from_millis(100));
                }
            }));
        }
        for handle in threads {
            handle.join().unwrap();
        }
    } else {
        println!("Invalid input. Try again.");
    }
}

// fn horse_run(scores: HashMap<i32, i32>, k: i32) {
    // println!("Hello from Thread {}!: {}", k, scores.get(&k).copied().unwrap_or(0));
// }