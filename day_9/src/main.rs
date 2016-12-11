
use std::fs::File;
use std::io::Read;

fn decompress(input: &str) -> String {
    let mut output = String::new();

    let bytes = input.as_bytes();
    let mut tag_open_idx = 0;
    while tag_open_idx < bytes.len() {
        let next_chr = bytes[tag_open_idx] as char;
        //println!("checking '{}' (code {})", next_chr, bytes[tag_open_idx]);
        if next_chr == '(' {
            let sub_array = &bytes[tag_open_idx..bytes.len()];
            if let Some(close_offset) = sub_array.iter().position(|&x| x as char == ')') {
                let tag_close_idx = tag_open_idx + close_offset;
                let tag_string = &input[tag_open_idx + 1..tag_close_idx];
                let tag_values = tag_string.split('x')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                let block_width = tag_values[0];
                let block_count = tag_values[1];

                let block_start = tag_close_idx+1;
                let block_end = block_start+block_width;
                let block = &input[block_start..block_end];
                //println!("Block: {:?}, count {}", block, block_count);
                for _ in 0..block_count {
                    output.push_str(block);
                }

                //println!("Tag '{}' parsed as {}x{}", &tag_string, block_width, block_count);
                //println!("Tag @ [{},{}], data block [{}]", tag_open_idx, tag_close_idx, block);
                //println!("Tag [{}], data block [{}]", &tag_string, block);

                // Advance the processing index to after the decompressed block.
                tag_open_idx = block_end;
            } else {
                panic!("Unable to parse tag");
            }
        }
        else if bytes[tag_open_idx] <= 32 {
            // Skip non-printing ASCII characters <= 32.
            tag_open_idx += 1;
        } else {
            // No change. Copy byte to the output.
            output.push(bytes[tag_open_idx] as char);
            tag_open_idx += 1;
        }
    }
    //println!("Output: {}", output);
    output
}

// Return the number bytes in the output if 'input' were decompressed.
fn simulate_decompress_v2(input: &str) -> usize {
    let mut output_size = 0;

    let bytes = input.as_bytes();
    let mut tag_open_idx = 0;
    while tag_open_idx < bytes.len() {
        let next_chr = bytes[tag_open_idx] as char;
        if next_chr == '(' {
            let sub_array = &bytes[tag_open_idx..bytes.len()];
            if let Some(close_offset) = sub_array.iter().position(|&x| x as char == ')') {
                let tag_close_idx = tag_open_idx + close_offset;
                let tag_string = &input[tag_open_idx + 1..tag_close_idx];
                let tag_values = tag_string.split('x')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                let block_width = tag_values[0];
                let block_count = tag_values[1];

                let block_start = tag_close_idx+1;
                let block_end = block_start+block_width;
                let block = &input[block_start..block_end];

                // Assuming there are no dependencies between blocks, we can just multiply the size.
                let decompressed_block_size = simulate_decompress_v2(&block);
                output_size += block_count * decompressed_block_size;

                // Advance the processing index to after the decompressed block.
                tag_open_idx = block_end;
            } else {
                panic!("Unable to parse tag");
            }
        }
            else if bytes[tag_open_idx] <= 32 {
                // Skip non-printing ASCII characters <= 32.
                tag_open_idx += 1;
            } else {
                // No change. Add size to output.
                output_size += 1;
                tag_open_idx += 1;
            }
    }
    //println!("Output: {}", output);
    output_size
}

// Iterate decompress() until all symbols are expanded.
#[allow(dead_code)]
fn decompress_v2(input: &str) -> String {
    let mut last_length = input.len();
    let mut output = decompress(input);
    while last_length != output.len() {
        last_length = output.len();
        output = decompress(&output);
    }
    output
}

fn main() {
    let mut input_string = String::new();
    let mut file = File::open("input.txt").unwrap();
    let _ = file.read_to_string(&mut input_string);

    {
        let output = decompress(&input_string);
        println!("Part 1: length {} bytes", output.len());
        assert!(output.len() == 123908);
    }

    {
        let output_2_len = simulate_decompress_v2(&input_string);
        println!("Part 2: length {} bytes", output_2_len);
        assert!(output_2_len == 10755693147);
    }
}

#[test]
fn test_format1() {
    assert!(decompress("ADVENT") == "ADVENT");
    assert!(decompress("ADVENT").len() == 6);

    assert!(decompress("A(1x5)BC") == "ABBBBBC");
    assert!(decompress("A(1x5)BC").len() == 7);

    assert!(decompress("(3x3)XYZ") == "XYZXYZXYZ");
    assert!(decompress("(3x3)XYZ").len() == 9);

    assert!(decompress("A(2x2)BCD(2x2)EFG") == "ABCBCDEFEFG");
    assert!(decompress("A(2x2)BCD(2x2)EFG").len() == 11);

    assert!(decompress("(6x1)(1x3)A") == "(1x3)A");
    assert!(decompress("(6x1)(1x3)A").len() == 6);

    assert!(decompress("X(8x2)(3x3)ABCY\n") == "X(3x3)ABC(3x3)ABCY");
    assert!(decompress("X(8x2)(3x3)ABCY\n").len() == 18);
}

#[test]
fn test_format2() {
    assert!(decompress_v2("ADVENT") == "ADVENT");
    assert!(decompress_v2("A(1x5)BC") == "ABBBBBC");
    assert!(decompress_v2("(3x3)XYZ") == "XYZXYZXYZ");
    assert!(decompress_v2("A(2x2)BCD(2x2)EFG") == "ABCBCDEFEFG");
    assert!(decompress_v2("(6x1)(1x3)A") == "AAA");
    assert!(decompress_v2("X(8x2)(3x3)ABCY") == "XABCABCABCABCABCABCY");

    // Output is too long to type, so just compare the output length.
    assert!(decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A").len() == 241920);
    assert!(decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN").len() == 445);
}

#[test]
fn test_format2_simulate() {
    assert!(simulate_decompress_v2("ADVENT") == 6);
    assert!(simulate_decompress_v2("A(1x5)BC") == 7);
    assert!(simulate_decompress_v2("(3x3)XYZ") == 9);
    assert!(simulate_decompress_v2("A(2x2)BCD(2x2)EFG") == 11);
    assert!(simulate_decompress_v2("(6x1)(1x3)A") == 3);
    assert!(simulate_decompress_v2("X(8x2)(3x3)ABCY") == 20);
    assert!(simulate_decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A") == 241920);
    assert!(simulate_decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN") == 445);
}