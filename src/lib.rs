#[derive(Hash, PartialEq, Eq)]

enum Operator {
    Brackets,
    Exponents,
    Multiplication,
    Division,
    Addition,
    Subtraction,
}

impl Operator {
    pub fn get_level(op: &Operator) -> u8 {
        match op {
            Operator::Brackets => 1,
            Operator::Exponents => 2,
            Operator::Multiplication => 3,
            Operator::Division => 3,
            Operator::Addition => 4,
            Operator::Subtraction => 4,
        }
    }

    pub fn get_char(op: &Operator) -> char {
        match op {
            Operator::Brackets => '(',
            Operator::Exponents => '^',
            Operator::Multiplication => '*',
            Operator::Division => '/',
            Operator::Addition => '+',
            Operator::Subtraction => '-',
        }
    }

    pub fn from_char(c: char) -> Operator {
        match c {
            '(' | ')' => Operator::Brackets,
            '^' => Operator::Exponents,
            '*' => Operator::Multiplication,
            '/' => Operator::Division,
            '+' => Operator::Addition,
            '-' => Operator::Subtraction,
        }
    }
}

enum ExpressionEnum {
    Exp(Expression),
    Number(i32),
}

struct Expression {
    op: Option<Operator>,
    left: Option<Box<ExpressionEnum>>,
    right: Option<Box<ExpressionEnum>>,
}

impl Expression {
    pub fn empty() -> Self {
        Self {
            op: None,
            left: None,
            right: None,
        }
    }

    pub fn new(op: Operator) -> Self {
        Self {
            op: Some(op),
            left: None,
            right: None,
        }
    }
}
struct ExpQueue<'a> {
    pub tree: Expression,
    pub queue: Vec<&'a Expression>,
}

impl<'a> ExpQueue<'a> {
    pub fn new() -> Self {
        let exp = Expression::empty();
        Self {
            tree: exp,
            queue: vec![&exp],
        }
    }

    pub fn add_above(index: u32) {}
}

pub fn build_exp_tree(exp: &str) -> Expression {
    let op_chars: Vec<char> = "()^*/+-".chars().collect();

    let mut exp_tree: Expression = Expression {
        op: None,
        left: None,
        right: None,
    };
    let mut levels: Vec<&Expression> = Vec::from([&exp_tree]);

    let mut num_start_char;
    let mut num_stop_char;
    let mut parsing_num = false;

    let mut parse_num: i32;
    let mut op = None;

    for (i, char) in exp.char_indices() {
        // case for numbers
        if char.is_ascii_digit() {
            num_stop_char = i;
            if parsing_num {
                continue;
            } else {
                num_start_char = i;
                parsing_num = true;
                continue;
            }
        } else {
            if parsing_num {
                parsing_num = false;
                parse_num = exp[num_start_char..=num_stop_char]
                    .parse()
                    .expect("Should be number");
                let cur_exp = levels.last().expect("Levels should never be empty");

                if cur_exp.left.is_none() {
                    cur_exp.left = Some(Box::new(ExpressionEnum::Number(parse_num)));
                } else if cur_exp.right.is_none() {
                    cur_exp.right = Some(Box::new(ExpressionEnum::Number(parse_num)));
                }

                if char == '(' {
                    cur_exp.right = Some(Box::new(ExpressionEnum::Exp(Expression::new(
                        Operator::Multiplication,
                    ))));
                }
            }
        }

        match char {
            '(' => {
                if exp_tree.is_none() {
                    exp_tree = Expression::new(Operator::Brackets);
                }
                op = Some(Operator::Brackets);
            }
            ')' => {}
            _ => {
                parsing_num = false;
                continue;
            }
        }
    }

    exp_tree
}
