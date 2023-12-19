use std::{
    collections::HashMap,
    io::{Error, Read},
    ops::{Index, IndexMut},
};

const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConditionType {
    Less,
    Greater,
}

impl TryFrom<char> for ConditionType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(ConditionType::Less),
            '>' => Ok(ConditionType::Greater),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Cool = 0,
    Musical = 1,
    Aerodynamic = 2,
    Shiny = 3,
}

impl TryFrom<char> for Category {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Category::Cool),
            'm' => Ok(Category::Musical),
            'a' => Ok(Category::Aerodynamic),
            's' => Ok(Category::Shiny),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Condition {
    category: Category,
    ctype: ConditionType,
    value: usize,
}

impl Condition {
    fn evaluate(&self, part: Part) -> bool {
        let v = part[self.category];
        match self.ctype {
            ConditionType::Less => v < self.value,
            ConditionType::Greater => v > self.value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkflowResult<'a> {
    Jump(&'a str),
    Accept,
    Reject,
}

impl<'a> From<&'a str> for WorkflowResult<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => WorkflowResult::Accept,
            "R" => WorkflowResult::Reject,
            s => WorkflowResult::Jump(s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule<'a> {
    condition: Condition,
    result: WorkflowResult<'a>,
}

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let f = s.find(':').ok_or(())?;
        let category: Category = s.chars().next().ok_or(())?.try_into()?;
        let ctype: ConditionType = s.chars().nth(1).ok_or(())?.try_into()?;
        let value: usize = s[2..f].parse().map_err(|_| ())?;
        let result: WorkflowResult = s[f + 1..].into();

        Ok(Rule {
            condition: Condition {
                category,
                ctype,
                value,
            },
            result,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    fallback: WorkflowResult<'a>,
}

impl<'a> TryFrom<&'a str> for Workflow<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut rules: Vec<Rule> = Vec::new();
        let mut fallback: Option<WorkflowResult> = None;

        for rule in s.split(',') {
            match rule.find(':') {
                Some(_) => rules.push(rule.try_into()?),
                None => fallback = Some(rule.into()),
            }
        }

        Ok(Workflow {
            rules,
            fallback: fallback.ok_or(())?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    arr: [usize; 4],
}

impl Index<Category> for Part {
    type Output = usize;

    fn index(&self, index: Category) -> &Self::Output {
        &self.arr[index as usize]
    }
}

impl IndexMut<Category> for Part {
    fn index_mut(&mut self, index: Category) -> &mut Self::Output {
        &mut self.arr[index as usize]
    }
}

impl TryFrom<&str> for Part {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut arr = [0; 4];
        for entry in s.split(',') {
            let category: Category = entry.chars().next().ok_or(())?.try_into()?;
            let value: usize = entry[2..].parse().map_err(|_| ())?;

            arr[category as usize] = value;
        }

        Ok(Part { arr })
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let f = input.trim().find("\n\n").unwrap();
    let workflows_input = input[..f].trim();
    let parts_input = input[f + 1..].trim();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    for workflow in workflows_input.lines() {
        let f = workflow.find('{').unwrap();
        let name = &workflow[..f];

        workflows.insert(
            name.to_string(),
            workflow[f + 1..workflow.len() - 1].try_into().unwrap(),
        );
    }

    for part in parts_input.lines() {
        parts.push(part[1..part.len() - 1].try_into().unwrap());
    }

    (workflows, parts)
}

fn part1(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    let mut result = 0;

    for part in parts {
        let mut current = WorkflowResult::Jump("in");

        'outer: loop {
            match current {
                WorkflowResult::Jump(name) => {
                    let workflow = &workflows[name];
                    for rule in &workflow.rules {
                        if rule.condition.evaluate(part) {
                            current = rule.result;
                            continue 'outer;
                        }
                    }
                    current = workflow.fallback;
                }
                WorkflowResult::Accept => {
                    for i in 0..4 {
                        result += part.arr[i];
                    }
                    break;
                }
                WorkflowResult::Reject => break,
            }
        }
    }

    result
}

fn part2(input: &str) -> usize {
    0
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    // dbg!(parse_input(EXAMPLE));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 19114;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
