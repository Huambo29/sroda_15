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
		for word in line.split_whitespace() {
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

fn encode_text(contents: String) -> String {
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

	for line in contents.lines() {
		let mut result_line = String::new();

		let words = line.split_whitespace();
		for (i, word) in words.enumerate() {
			if i != 0 {
				result_line.push(' ');
			}
			match dictionary.get(word) {
				Some(id) => {
					result_line.push(ENCODING_CHAR);
					result_line.push_str(&id.to_string());
				},
				None => {
					result_line.push_str(word);
				}
			}
		}

		result.push_str(&result_line);
		result.push_str("\n")
	}

	result
}

#[derive(Debug, Clone)]
struct DecodingError;

fn decode_text(contents: String) -> Result<String, DecodingError> {
	print!("123\n\n");
	let mut split = contents.split(ENCODING_CHAR);
	split.next();
	let Some(dictionary_raw) = split.next() else {
		return Err(DecodingError);
	};

	let mut dictionary: Vec<String> = dictionary_raw.split_whitespace().map(|word| word.to_string()).collect();

	for line in split.map(|x| {
		if x.chars().all(|c| c.is_numeric()) {
			dictionary.get(x.parse()?)
		} else {
			x.to_string()
		}
	}).collect::<Vec<String>>().join(ENCODING_CHAR.to_string().as_str()).lines() {
		println!("{}", line);
	}

	Ok("".to_string())
}

fn main() {
	let file_path = std::env::args().nth(1).expect("no file path given").to_string();
	println!("{}", file_path);

	let mut instant = Instant::now();
	let mut file = File::open(file_path).expect("file not found");
	time_phase(&mut instant, "text file opened in ");
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	println!("{}", contents);
	time_phase(&mut instant, "converted to string in ");
	
	let encoded = encode_text(contents);
	println!("{}", encoded);
	time_phase(&mut instant, "encoded in ");


	let decoded = decode_text(encoded);
	if let Ok(decoded) = decoded {
		println!("{}", decoded);
	} else {
		println!("Decoding failed");
	}
	time_phase(&mut instant, "decoded in ")
}