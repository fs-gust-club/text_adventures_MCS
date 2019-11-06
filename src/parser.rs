use nom::{IResult};
use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::error::ErrorKind;
use nom::sequence::*;

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

pub fn parse_input(user_input: &str) -> Action {
    let trimmed = user_input.trim();
    is_exit(trimmed)
        .unwrap_or_else(|| is_load(trimmed)
        .unwrap_or_else(|| is_save(trimmed)
        .unwrap_or_else(|| is_inventory(trimmed)
        .unwrap_or_else(|| is_move(trimmed)
        .unwrap_or_else(|| is_take(trimmed)
        .unwrap_or(Action::Unknown))))))
}

fn is_exit(input: &str) -> Option<Action> {
    let parser = pair(
            alt((tag_no_case("exit"), tag_no_case("quit"))),
            space0);    

    let a: Result<(&str, (&str, &str)), nom::Err<(&str, ErrorKind)>> = parser(input);

    match a {
        Ok(_result) => Some(Action::Exit),
        Err(_err) => None
    }
}

fn is_inventory(input: &str) -> Option<Action> {
    let a: IResult<&str, &str> = alt((tag_no_case("inventory"), tag_no_case("inv")))(input);
    match a {
        Ok(_result) => Some(Action::Inventory),
        Err(_err) => None
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
        alpha1);

    let result: Result<(&str, (&str, &str)), nom::Err<(&str, ErrorKind)>> = parser(input);

    match result {
        Ok(res) => {
            let (_remaining_input, (_first, second)) = res;
            Some(Action::Move(second.to_string()))
        }
        Err(_err) => None
    }
}

fn is_take(input: &str) -> Option<Action> {
    let parser = separated_pair(
        alt((tag_no_case("take"), tag_no_case("get"))), 
        space1,
        alpha1);

    let result: Result<(&str, (&str, &str)), nom::Err<(&str, ErrorKind)>> = parser(input);

    match result {
        Ok(res) => {
            let (_remaining_input, (_first, second)) = res;
            Some(Action::Take(second.to_string()))
        }
        Err(_err) => None
    }
}