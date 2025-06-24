use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::collections::HashMap;

const ENCODING_CHAR: char = '@';

fn time_phase(instant: &mut Instant, message: &str) {
	println!("{}\t\t{} s", message, instant.elapsed().as_secs_f32());
	*instant = Instant::now();
}

fn is_good_for_encoding(word: &str, popularity: u32) -> bool {
	!word.chars().all(|x| x.is_numeric()) && popularity > 1
}

fn count_popularity(contents: &String) -> HashMap<String, u32> {
	let mut result: HashMap<String, u32> = HashMap::new();
	for line in contents.lines() {
		for word in line.split(' ') {
			match result.get_mut(word) {
				Some(count) => {
					*count += 1
				},
				None => {
					result.insert(word.to_string(), 1);
				}
			}
		}
	}
	result
}

fn encode_text(contents: &String) -> String {
	let mut result = String::new();
	result.push(ENCODING_CHAR);

	let dictionary = count_popularity(&contents).iter()
		.filter(|(x, popularity)| is_good_for_encoding(x, **popularity))
		.enumerate().map(|(id, (word, _))| {
			if id != 0 {
				result.push(' ');
			}
			result.push_str(word);
			(word.clone(), id as u32)
		})
		.collect::<HashMap<String, u32>>();

	result.push(ENCODING_CHAR);

	for (i, line) in contents.lines().enumerate() {
		if i != 0 {
			result.push_str("\n");
		}
		let mut result_line = String::new();

		for (i, word) in line.split(' ').enumerate() {
			// println!("word: {}", word);
			if i != 0 {
				result_line.push(' ');
			}
			match dictionary.get(word) {
				Some(id) => {
					result_line.push(ENCODING_CHAR);
					result_line.push_str(&id.to_string());
				},
				None => {
					if !word.is_empty() && word.chars().next().unwrap() == ENCODING_CHAR {
						result_line.push(ENCODING_CHAR);
					}
					result_line.push_str(word);
				}
			}
		}

		result.push_str(&result_line);
	}

	result
}

#[derive(Debug, Clone)]
struct DecodingError;

fn decode_text(contents: &String) -> Result<String, DecodingError> {
	// print!("123\n\n");
	let mut split = contents.split(ENCODING_CHAR);
	split.next();
	let Some(dictionary_raw) = split.next() else {
		return Err(DecodingError);
	};

	let dictionary: Vec<String> = dictionary_raw.split_whitespace().map(|word| word.to_string()).collect();

	Ok(split.collect::<Vec<&str>>().join(&ENCODING_CHAR.to_string()).lines().map(|line| {
		line.split(' ').map(|word| {
			// println!("split: /{}/", word);
			let mut chars = word.chars();
			if !word.is_empty() && chars.next().unwrap() == ENCODING_CHAR {
				let parse_candidate = chars.collect::<String>();
				match parse_candidate.parse::<usize>() {
					Ok(index) => {
						// println!("case 2137");
						dictionary.get(index).map_or("".to_string(), |x| x.clone())
					},
					Err(_) => {
						parse_candidate
					}
				}
			} else {
				word.to_string()
			}
		}).collect::<Vec<String>>().join(" ")
	}).collect::<Vec<String>>().join("\n"))
}

fn main() {
	let file_path = std::env::args().nth(1).expect("no file path given").to_string();
	println!("{}", file_path);

	let mut instant = Instant::now();
	let mut file = File::open(file_path.clone()).expect("file not found");
	time_phase(&mut instant, "text file opened in ");
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	let already_encoded = contents.chars().next().unwrap_or_default() == ENCODING_CHAR;

	contents = contents.lines().collect::<Vec<&str>>().join("\n");

	time_phase(&mut instant, "converted to string in ");
	
	let manipulated = if !already_encoded {
		let encoded = encode_text(&contents);
		time_phase(&mut instant, "encoded in ");

		encoded
	} else {
		let decoded_result = decode_text(&contents);
		time_phase(&mut instant, "decoded in ");

		match decoded_result {
			Ok(decoded) => decoded,
			Err(_) => {
				println!("Decoding failed");
				String::new()
			}
		}
	};

	if let Some(save_path) = std::env::args().nth(2) {
		let _ = std::fs::write(save_path, manipulated.clone());
	}

	// check
	if !already_encoded {
		let decoded_result = decode_text(&manipulated);
		time_phase(&mut instant, "decoded in ");

		match decoded_result {
			Ok(decoded) => {
				let decoded_lines = decoded.lines().collect::<Vec<&str>>();
				for (i, line) in contents.lines().enumerate() {
					assert_eq!(line, decoded_lines[i]);
				}
				println!("ENCODING CORRECT");
			},
			Err(_) => {
				println!("Decoding for assert failed");
			}
		}
	}

	let compression_ratio = 1.0 - ((manipulated.len() as f32) / (contents.len() as f32));
	println!("{} Compression ratio: {}\n", file_path, (compression_ratio * 100.0).round() / 100.0);
}