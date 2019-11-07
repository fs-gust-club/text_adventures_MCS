use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::error::ErrorKind;
use nom::sequence::*;
use nom::IResult;

type ParserPairResult<'a> = Result<(&'a str, (&'a str, &'a str)), nom::Err<(&'a str, ErrorKind)>>;

pub enum Action {
    Exit,
    Load,
    Save,
    Inventory,
    Move(String),
    Take(String),
    Put(String),
    Use(String),
    Interact(String, String),
    Unknown,
}

/// Apply parser combinator to user's input
/// 
/// # Arguments
/// 
/// * `user_input` - the user input to parse for recognised actions
pub fn parse_input(user_input: &str) -> Action {
    let trimmed = user_input.trim();

    let actions: Vec<fn(&str) -> Option<Action>> =
        vec![is_exit, is_load, is_save, is_inventory, is_move, is_take];

    // Here, we iterate through a list of higher order functions and
    // effectively request the first function to return a Some(). This uses the
    // filter_map to lazily collapse the list to a function that returns a
    // Some(), and then just request the first one.
    actions
        .iter()
        .filter_map(|f| f(trimmed))
        .next()
        .unwrap_or(Action::Unknown)
}

fn is_exit(input: &str) -> Option<Action> {
    let parser = pair(alt((tag_no_case("exit"), tag_no_case("quit"))), space0);

    let a: ParserPairResult = parser(input);

    match a {
        Ok(_result) => Some(Action::Exit),
        Err(_err) => None,
    }
}

fn is_inventory(input: &str) -> Option<Action> {
    let a: IResult<&str, &str> = alt((tag_no_case("inventory"), tag_no_case("inv")))(input);
    match a {
        Ok(_result) => Some(Action::Inventory),
        Err(_err) => None,
    }
}

fn is_load(input: &str) -> Option<Action> {
    let t: IResult<&str, &str> = tag_no_case("load")(input);
    match t {
        Ok(_result) => Some(Action::Load),
        Err(_error) => None,
    }
}

fn is_save(input: &str) -> Option<Action> {
    let t: IResult<&str, &str> = tag_no_case("save")(input);
    match t {
        Ok(_result) => Some(Action::Load),
        Err(_error) => None,
    }
}

fn is_move(input: &str) -> Option<Action> {
    let parser = separated_pair(
        alt((tag_no_case("move"), tag_no_case("go"))),
        space1,
        alpha1,
    );

    let result: ParserPairResult = parser(input);

    match result {
        Ok(res) => {
            let (_remaining_input, (_first, second)) = res;
            Some(Action::Move(second.to_string()))
        }
        Err(_err) => None,
    }
}

fn is_take(input: &str) -> Option<Action> {
    let parser = separated_pair(
        alt((tag_no_case("take"), tag_no_case("get"))),
        space1,
        alpha1,
    );

    let result: ParserPairResult = parser(input);

    match result {
        Ok(res) => {
            let (_remaining_input, (_first, second)) = res;
            Some(Action::Take(second.to_string()))
        }
        Err(_err) => None,
    }
}
