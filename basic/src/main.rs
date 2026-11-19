use std::net::UdpSocket;

fn main() {
    println!("Yo Yo Yo");

    let socket = UdpSocket::bind("0.0.0.0:9007").expect("bind should succeed");
    let mut message = [0u8; 255];
    loop {
        let (amt, src) = match socket.recv_from(&mut message) {
            Ok(result) => result,
            Err(_) => {
                println!("error");
                continue;
            }
        };
        println!("Received {} bytes from {}", amt, src);
        handle_message(amt);
    }
}

fn handle_message(data: usize) {
    println!("Received {} bytes", data);
}

// use std::io;
// use rand::RngExt;

// fn main() {
//     println!("gues the game");
//     println!("choose number between 1 to 100");
//     println!("You have total 7 attempts to guess the number");

//     let num = rand::rng().random_range(1..=100);
//     let mut attempt: u32 = 0;
//     let total_attempt:u32 = 7 - 1;

//     loop {
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).unwrap();
//         let user_guess = match guess.trim().parse::<u32>() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Enter Integar you shit");
//                 continue;
//             }
//         };

//         if user_guess < 1 || user_guess > 100 {
//             println!("Enter the number between 1 to 100 you shit");
//         };

//         attempt += 1;
//         if attempt > total_attempt {
//             println!("You have used all your attempts!");
//             println!("You loose skibidi dumb:)");
//             break;
//         }
//         println!("attempted left {}", total_attempt - attempt + 1);

//         if user_guess > num {
//             println!("high");
//             continue;
//         } else if user_guess < num {
//             println!("low");
//             continue;
//         } else {
//             println!("you mew~~~");
//             break;
//         }
//     }
// }
