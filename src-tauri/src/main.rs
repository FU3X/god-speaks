// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn generate_context() -> &'static str {
  let quote: [&str; 20] = [
      r##"I am the Lord your God."##,
      r##"You shall have no other gods before me."##,
      r##"You shall not make for yourself an idol."##,
      r##"You shall not misuse the name of the Lord your God."##,
      r##"Remember the Sabbath day by keeping it holy."##,
      r##"Honor your father and your mother."##,
      r##"You shall not murder."##,
      r##"You shall not commit adultery."##,
      r##"You shall not steal."##,
      r##"You shall not give false testimony against your neighbor."##,
      r##"You shall not covet your neighbor's house."##,
      r##"You shall not covet your neighbor's wife."##,
      r##"I am the Lord, and there is no other; apart from me there is no God."##,
      r##"I am the Alpha and the Omega, the First and the Last, the Beginning and the End."##,
      r##"I am who I am."##,
      r##"I am the way and the truth and the life. No one comes to the Father except through me."##,
      r##"I am the light of the world. Whoever follows me will never walk in darkness, but will have the light of life."##,
      r##"I am the bread of life. Whoever comes to me will never go hungry, and whoever believes in me will never be thirsty."##,
      r##"I am the good shepherd. The good shepherd lays down his life for the sheep."##,
      r##"I am the resurrection and the life. The one who believes in me will live, even though they die; and whoever lives by believing in me will never die."##
  ];

  let mut rng = rand::thread_rng();
  let ran_quote: &str = quote[rng.gen_range(0..quote.len())];
  ran_quote
}