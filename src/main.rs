fn main() {
    // Read bit string length
    println!("Enter bit string length: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let bit_string_length = input.trim().parse::<usize>().unwrap();

    // Read two numbers
    println!("Enter two numbers (whitespace seperated): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut q = numbers[0];
    let m = numbers[1];
    let mut q_1 = '0';
    let mut cycle = 0; 
    let mut a = 0;

    println!("Numbers in decimal: {} {}", numbers[0], numbers[1]);
    println!("Numbers in binary: {} {}", generate_binary(q, bit_string_length), generate_binary(m, bit_string_length));

    println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} |", cycle, generate_binary(a, bit_string_length), generate_binary(q, bit_string_length), q_1, generate_binary(m, bit_string_length));
    for i in 0..bit_string_length {
        // let q_2 be the last digit of q plus q_1
        let mut q_2: String = generate_binary(q, bit_string_length).chars().nth(bit_string_length-1).unwrap().to_string();
        q_2.push(q_1);

        cycle = i as i32 + 1; 
        if q_2 == "10" {
            let (temp, temp_a, temp_q, temp_q_1) = subtraction(a, q, q_1, m, cycle, bit_string_length);
            a = temp_a;
            q = temp_q;
            q_1 = temp_q_1;
        } else if q_2 == "01" {
            let (temp, temp_a, temp_q, temp_q_1) = addition(a, q, q_1, m, cycle, bit_string_length);
            a = temp_a;
            q = temp_q;
            q_1 = temp_q_1;
        } else {
            println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} | Q_0Q_(-1) = {}", cycle, generate_binary(a, bit_string_length), generate_binary(q, bit_string_length), q_1, generate_binary(m, bit_string_length), q_2);
            let (temp, temp_a, temp_q, temp_q_1) = shift(a, q, q_1, m, cycle, bit_string_length);
            a = temp_a;
            q = temp_q;
            q_1 = temp_q_1;
        }
    }
    println!("Final result: {}{}", generate_binary(a, bit_string_length), generate_binary(q, bit_string_length));
    println!("Final result: {}", numbers[0] * numbers[1]);
}

/// Generate binary string of length len from num
///
/// # Arguments
/// * `num` - number to convert to binary
/// * `len` - length of binary string
///
/// # Returns
/// * binary string
///
/// # Examples
/// ```
/// assert_eq!(generate_binary(23, 32), "00000000000000000000000000010111");
/// ```
fn generate_binary(num: i32, len: usize) -> String {
    let mut b = format!("{:032b}", num);
    b.replace_range(..b.len() - len, "");
    b.to_owned()
}

fn addition(a: i32, q: i32, q_1: char, m: i32, cycle: i32, len: usize) -> (i32, i32, i32, char) {
    println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} | Q_0Q_(-1) = 01", cycle, generate_binary(a, len), generate_binary(q, len), q_1, generate_binary(m, len));
    println!("|         |   + {} | Q = {} | Q_1 = {} | M = {} | Add {} to {}", generate_binary(m, len), generate_binary(q, len), q_1, generate_binary(m, len), m, generate_binary(a, len));
    println!("|         |     {} | Q = {} | Q_1 = {} | M = {} | ", generate_binary(a+m, len), generate_binary(q, len), q_1, generate_binary(m, len));
    shift(a+m, q, q_1, m, cycle, len)
}

/// Subtract two binary numbers and print the process
///
/// # Arguments
/// * `a` - first number
/// * `q` - second number
/// * `q_1` - leftover digit
/// * `m` - third number
/// * `cycle` - number of cycles
/// * `len` - length of bit string
///
/// # Returns
/// * i32: subtraction result
/// * i32: a, first number
/// * i32: q, second number
/// * i32: q_1, leftover digit
fn subtraction(a: i32, q: i32, q_1: char, m: i32, cycle: i32, len: usize) -> (i32, i32, i32, char) {
    println!("| Cycle {} | A = {} | Q = {} | Q_1 = {} | M = {} | Q_0Q_(-1) = 10", cycle, generate_binary(a, len), generate_binary(q, len), q_1, generate_binary(m, len));
    println!("|         |   + {} | Q = {} | Q_1 = {} | M = {} | Subtract {} from {}", generate_binary(-m, len), generate_binary(q, len), q_1, generate_binary(m, len), m, generate_binary(a, len));
    println!("|         |     {} | Q = {} | Q_1 = {} | M = {} | ", generate_binary(a-m, len), generate_binary(q, len), q_1, generate_binary(m, len));
    shift(a-m, q, q_1, m, cycle, len)
}

fn shift(a: i32, q: i32, q_1: char, m: i32, cycle: i32, len: usize) -> (i32, i32, i32, char) {
    let mut shifted_value = generate_binary(a, len);
    shifted_value.push_str(&generate_binary(q, len));
    shifted_value.push(q_1);
    let first: char = shifted_value[..].chars().nth(0).unwrap();
    if first == '1' {
        shifted_value = '1'.to_string() + &shifted_value[..];
    } else if first == '0' {
        shifted_value = '0'.to_string() + &shifted_value[..];
    } else {
        println!("Error: {}", first);
    }
    println!("|         | A = {} | Q = {} | Q_1 = {} | M = {} | Shift", &shifted_value[0..len], &shifted_value[len..len*2], &shifted_value[len*2..(len*2)+1], generate_binary(m, len));
    (0, twos_complement_to_i32(&shifted_value[0..len], len), twos_complement_to_i32(&shifted_value[len..len*2], len), shifted_value[len*2..len*2+1].chars().nth(0).unwrap())
}

fn twos_complement_to_i32(num: &str, len: usize) -> i32 {
    let mut result = 0; 
    if num.to_string().as_bytes()[0] == 49 {
        result = -2_i32.pow(len as u32 - 1);
    }
    for (i, char) in num.chars().enumerate() {
        if i == 0 {
            continue;
        }
        if char == '1' {
            result += 2_i32.pow((len - i - 1).try_into().unwrap());
        }
    }
    result
}


