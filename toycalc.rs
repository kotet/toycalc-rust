struct State {
    inp: Vec<char>,
    i: usize,
    func: Vec<Vec<char>>,
    args: Vec<i64>,
}

fn main() {
    use std::env;
    let input = env::args()
        .skip(1)
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    let mut s = State {
        inp: input,
        i: 0,
        func: vec![<Vec<char>>::new(); 26],
        args: vec![0; 26],
    };

    while s.i < s.inp.len() {
        println!("{}", eval(&mut s));
    }
}

fn eval(s: &mut State) -> i64 {
    skip(s);

    if s.inp[s.i].is_digit(10) {
        let mut val = 0;

        while s.i < s.inp.len() {
            match s.inp[s.i].to_digit(10) {
                Some(n) => val = val * 10 + n as i64,
                None => return val,
            }
            s.i += 1;
        }

        val
    } else if s.inp[s.i] == '+' {
        s.i += 1;
        eval(s) + eval(s)
    } else if s.inp[s.i] == '-' {
        s.i += 1;
        let l = eval(s);
        let r = eval(s);
        l - r
    } else if s.inp[s.i] == '*' {
        s.i += 1;
        eval(s) * eval(s)
    } else if s.inp[s.i] == '/' {
        s.i += 1;
        let l = eval(s);
        let r = eval(s);
        l / r
    } else if s.inp[s.i] == 'P' {
        s.i += 2;

        let result = eval(s);
        println!("{}", result);

        s.i += 1;

        result
    } else if s.inp[s.i].is_ascii_alphabetic()
        && s.inp[s.i].is_ascii_uppercase()
        && s.inp[s.i + 1] == '['
    {
        let name = s.inp[s.i];
        let id = name as usize - 'A' as usize;

        s.i += 2;

        let f = read_until(s, ']');
        s.func[id] = f;

        s.i += 1;

        eval(s)
    } else if s.inp[s.i].is_ascii_alphabetic()
        && s.inp[s.i].is_ascii_uppercase()
        && s.inp[s.i + 1] == '('
    {
        let name = s.inp[s.i];
        let id = name as usize - 'A' as usize;

        s.i += 2;

        let mut f = State {
            inp: s.func[id].clone(),
            i: 0,
            func: s.func.clone(),
            args: vec![0; 26],
        };

        let mut i = 0;

        while s.inp[s.i] != ')' {
            f.args[i] = eval(s);
            i += 1;
            skip(s);
        }
        s.i += 1;

        let mut result = eval(&mut f);
        while f.i < f.inp.len() {
            result = eval(&mut f);
        }
        result
    } else if s.inp[s.i].is_ascii_alphabetic() && s.inp[s.i].is_ascii_lowercase() {
        let name = s.inp[s.i];
        let id = name as usize - 'a' as usize;
        s.i += 1;
        s.args[id]
    } else {
        use std::process;
        println!("Error: Invalid charactor: {}", s.inp[s.i]);
        process::exit(1);
    }
}

fn skip(s: &mut State) {
    while s.i < s.inp.len() && s.inp[s.i].is_whitespace() {
        s.i += 1;
    }
}

fn read_until(s: &mut State, c: char) -> Vec<char> {
    let mut result: Vec<char> = "".chars().collect::<Vec<char>>();
    while s.inp[s.i] != c {
        result.push(s.inp[s.i]);
        s.i += 1;
    }
    result
}
