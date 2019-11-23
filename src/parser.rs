use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::error::ErrorKind;
use nom::sequence::*;

type ParserPairResult<'a> = Result<(&'a str, (&'a str, &'a str)), nom::Err<(&'a str, ErrorKind)>>;
type ParserTwoPairResult<'a> =
    Result<(&'a str, ((&'a str, &'a str), (&'a str, &'a str))), nom::Err<(&'a str, ErrorKind)>>;

pub enum Action {
    Exit,
    Load,
    Save,
    Inventory,
    Move(String),
    Take(String),
    Put(String),
    Use(String, String),
    Unknown,
}

/// Apply parsers to user's input until there is a match or all known
/// commands have been tried.
///
/// # Arguments
///
/// * `user_input` - the user input to parse for recognized actions
pub fn parse_input(user_input: &str) -> Action {
    let trimmed = user_input.trim();

    let actions: Vec<fn(&str) -> Option<Action>> = vec![
        is_exit,
        is_load,
        is_save,
        is_inventory,
        is_move,
        is_take,
        is_use,
    ];

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
    let exit_parser = pair(alt((tag_no_case("exit"), tag_no_case("quit"))), space0);
    let parser_result: ParserPairResult = exit_parser(input);
    match parser_result {
        Ok(_result) => Some(Action::Exit),
        Err(_err) => None,
    }
}

fn is_inventory(input: &str) -> Option<Action> {
    let inv_parser = pair(alt((tag_no_case("inventory"), tag_no_case("inv"))), space0);
    let parser_result: ParserPairResult = inv_parser(input);
    match parser_result {
        Ok(_result) => Some(Action::Inventory),
        Err(_err) => None,
    }
}

fn is_load(input: &str) -> Option<Action> {
    let load_parser = pair(tag_no_case("load"), space0);
    let parser_result: ParserPairResult = load_parser(input);
    match parser_result {
        Ok(_result) => Some(Action::Load),
        Err(_error) => None,
    }
}

fn is_save(input: &str) -> Option<Action> {
    let save_parser = pair(tag_no_case("save"), space0);
    let parser_result: ParserPairResult = save_parser(input);
    match parser_result {
        Ok(_result) => Some(Action::Save),
        Err(_error) => None,
    }
}

fn is_move(input: &str) -> Option<Action> {
    let move_parser = separated_pair(
        alt((tag_no_case("move"), tag_no_case("go"))),
        space1,
        alpha1,
    );

    let parser_result: ParserPairResult = move_parser(input);

    match parser_result {
        Ok(res) => Some(Action::Move(deconstruct_pair_result(res))),
        Err(_err) => None,
    }
}

fn is_take(input: &str) -> Option<Action> {
    let take_parser = separated_pair(
        alt((tag_no_case("take"), tag_no_case("get"))),
        space1,
        alpha1,
    );

    let parser_result: ParserPairResult = take_parser(input);

    match parser_result {
        Ok(res) => Some(Action::Take(deconstruct_pair_result(res))),
        Err(_err) => None,
    }
}

fn is_use(input: &str) -> Option<Action> {
    let use_parser = separated_pair(tag_no_case("use"), space1, alpha1);

    let on_parser = separated_pair(tag_no_case("on"), space1, alpha1);

    let combined_parser = separated_pair(use_parser, space1, on_parser);

    // Result<(&'<empty> str, ((&'<empty> str, &'<empty> str), (&'<empty> str, &'<empty> str)))
    let parser_result: ParserTwoPairResult = combined_parser(input);

    match parser_result {
        Ok(res) => {
            let (_remaining_input, ((_use, subject), (_on, target))) = res;
            Some(Action::Use(subject.to_string(), target.to_string()))
        }
        Err(_err) => None,
    }
}

fn deconstruct_pair_result(result: (&str, (&str, &str))) -> String {
    let (_remaining_input, (_first, second)) = result;
    second.to_string()
}
