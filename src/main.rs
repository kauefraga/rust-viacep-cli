#![deny(warnings)]

use termion::{clear, color};
use std::io;

fn clear() {
  print!("{}", clear::All);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  clear();

  println!("HELLO");
  println!("Enter a cep: ");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess).expect("failed to readline");

  let url = format!("https://viacep.com.br/ws/{}/json/", guess);

  let body = reqwest::get(url)
    .await?
    .text()
    .await?;

  // eprintln!("Response: {:?} {}", res.version(), res.status());
  // eprintln!("Headers: {:#?}\n", res.headers());

  // let body = res.text().await?;

  println!("{}Location info is {}", color::Fg(color::Green), body);

  Ok(())
}
