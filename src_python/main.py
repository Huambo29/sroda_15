import time
import sys

ENCODING_CHAR = '@'

def time_phase(last_time: float, message: str):
	time_now = time.time()
	print(f"{message}\t\t{time_now - last_time} s")
	return time_now

def is_good_for_encoding(word: str, popularity: int):
	return not all(map(lambda x: x.isnumeric(), word)) and popularity > 1 and len(word) > 4

def count_popularity(contents: str):
	result = {}
	for line in contents.splitlines():
		for word in line.split(' '):
			if word in result:
				result[word] += 1
			else:
				result[word] = 1
	
	return result

def encode_text(contents: str):
	result = ""
	result += ENCODING_CHAR

	dictionary = {}

	for id, k in enumerate(filter(lambda k: is_good_for_encoding(k[0], k[1]), count_popularity(contents).items())):
		word, _ = k
		if id != 0:
			result += ' '
		result += word
		dictionary[word] = id

	result += ENCODING_CHAR
	
	for i, line in enumerate(contents.splitlines()):
		if i != 0:
			result += "\n"
		result_line = ""

		for i, word in enumerate(line.split(' ')):
			if i != 0:
				result_line += ' '

			if word in dictionary:
				result_line += f"{ENCODING_CHAR}{dictionary[word]}"
			else:
				if len(word) > 0 and word[0] == ENCODING_CHAR:
					result_line += ENCODING_CHAR
				result_line += word

		result += result_line

	return result

class DecodingError(Exception):
	pass


def decode_text(contents: str):
	split = contents.split(ENCODING_CHAR)

	if len(split) < 2:
		raise DecodingError
	dictionary = split[1].split()

	result = ""

	for i, line in enumerate(ENCODING_CHAR.join(split[2:]).splitlines()):
		if i != 0:
			result += "\n"
		
		result_line = ""
		for i, word in enumerate(line.split(' ')):
			if i != 0:
				result_line += ' '

			if len(word) > 1 and word[0] == ENCODING_CHAR:
				parse_candidate = word[1:]

				try:
					result_line += dictionary[int(parse_candidate)]
				except:
					result_line += parse_candidate
			else:
				result_line += word
			
		result += result_line

	return result

def main():
	if len(sys.argv) < 2:
		raise Exception("no file path given")

	file_path = sys.argv[1]
	print(f"{file_path}")

	start_time = time.time()
	with open(file_path, 'r', encoding='utf-8') as file:
		contents = file.read()
	start_time = time_phase(start_time, "text file opened in ")

	already_encoded = len(contents) > 0 if contents[0] == ENCODING_CHAR else False
	contents = "\n".join(contents.splitlines())
	start_time = time_phase(start_time, "converted to string in ")

	if not already_encoded:
		encoded = encode_text(contents)
		start_time = time_phase(start_time, "encoded in ")
		manipulated = encoded
	else:
		try:
			decoded_result = decode_text(contents)
			start_time = time_phase(start_time, "decoded in ")
			manipulated = decoded_result
		except:
			print("Decoding failed")
			manipulated = ""	

	if len(sys.argv) > 2:
		save_path = sys.argv[2]
		with open(save_path, 'w', encoding='utf-8') as f:
			f.write(manipulated)

	if not already_encoded:
		try:
			decoded_result = decode_text(manipulated)
			start_time = time_phase(start_time, "decoded in ")

			decoded_lines = decoded_result.splitlines()
			for i, line in enumerate(contents.splitlines()):
				# print(line)
				# print(decoded_lines[i])
				assert line == decoded_lines[i]
			print("ENCODING CORRECT")
		except DecodingError:
			print("Decoding for assert failed")

	compression_ratio = 1.0 - (len(manipulated) / len(contents))
	print(f"{file_path} Compression ratio: {round(compression_ratio, 2)}\n")

if __name__ == "__main__":
	main()