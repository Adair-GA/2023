use std::collections::HashMap;

use nom::{IResult, bytes::complete::{take_till, take_until, take, tag}, character::complete::{multispace0, newline, alpha1}, multi::separated_list1, sequence::separated_pair};

use crate::custom_error::AocError;

enum NodeType {
    Start,
    End,
    Any
}

struct Node<'a>{
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
    kind: NodeType
}

struct NodeStore<'a>{
    nodes: HashMap<&'a str, Node<'a>>
}

impl<'a> NodeStore<'a>{
    fn add_node(&mut self, name: &str){
        self.node
    }
}

fn node_pair(input: &str) -> IResult<&str, (Node, Node)>{
    let (input, _) = tag("(")(input)?;
    let (input, pair) = separated_pair(alpha1, tag(", "), alpha1)(input)?;

}

fn get_nodes(input: &str) -> IResult<&str, (Vec<Node>, Vec<(Node, Node)>)>{
    separated_list1(newline,
        separated_pair(alpha1, tag(" = "), node_pair)
    )
    
    todo!()
}

fn get_directions(_input: &str) -> IResult<&str, &str>{
    let (input, data) = take_until("\n")(_input)?;
    let (input, _) = take(2usize)(input)?;
    Ok((input, data))
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let (input, directions) = get_directions(_input).unwrap();

}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
    ";

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        assert_eq!("", process(input)?);
        Ok(())
    }

    #[test]
    fn test_get_directions() {
        let (_, data) = get_directions(input).unwrap();
        assert_eq!(data, "LLR")
    }
}
