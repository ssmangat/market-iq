use market_iq::{analyze_ticker, prompt_ticker};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    loop {
        // Ask the user for the ticker symbol
        println!("Please enter the ticker symbol (e.g., 'ZGRO-T.TO'), or 'exit' to quit:");
        let symbol = prompt_ticker();

        if symbol.to_lowercase() == "exit" {
            println!("Exiting the program.");
            break;
        }

        // Ask for the number of days to analyze (e.g., 365 for one year)
        println!("Enter the number of days to analyze (e.g., 365):");
        let mut days_back_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut days_back_input).unwrap();
        let days_back: i64 = days_back_input.trim().parse().unwrap_or(365); // Default to 365 if invalid input

        // Ask for the number of last days to analyze (e.g., 5 or 3)
        println!("Enter the number of last days to analyze (e.g., 5):");
        let mut last_n_days_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut last_n_days_input).unwrap();
        let last_n_days: usize = last_n_days_input.trim().parse().unwrap_or(5); // Default to 5 if invalid input

        // Perform the analysis
        match analyze_ticker(&symbol, days_back, last_n_days).await {
            Ok(_) => {
                println!("Analysis completed successfully for symbol: {}", symbol);
            }
            Err(err) => {
                eprintln!("Error during analysis: {}", err);
            }
        }
    }
}
