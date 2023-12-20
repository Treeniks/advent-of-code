use std::{
    collections::{HashMap, VecDeque},
    io::{Error, Read},
    ops::{Deref, DerefMut, Not},
};

const EXAMPLE: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

const EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Off,
    On,
}

impl Not for State {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            State::Off => State::On,
            State::On => State::Off,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Memory<'a>(HashMap<&'a str, PulseType>);

impl<'a> Deref for Memory<'a> {
    type Target = HashMap<&'a str, PulseType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Memory<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType<'a> {
    FlipFlop(State),
    Conjunction(Memory<'a>),
}

impl<'a> TryFrom<char> for ModuleType<'a> {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '%' => Ok(ModuleType::FlipFlop(State::Off)),
            '&' => Ok(ModuleType::Conjunction(Memory(HashMap::new()))),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    name: &'a str,
    mtype: ModuleType<'a>,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn send_signal(&mut self, ptype: PulseType, sender: &str, pulses: &mut VecDeque<Pulse<'a>>) {
        match self.mtype {
            ModuleType::FlipFlop(ref mut state) => {
                match ptype {
                    PulseType::Low => {
                        let ptype = match state {
                            State::Off => PulseType::High,
                            State::On => PulseType::Low,
                        };

                        for recipient in &self.destinations {
                            pulses.push_back(Pulse {
                                sender: self.name,
                                ptype,
                                recipient,
                            });
                        }

                        *state = !*state;
                    }
                    PulseType::High => {} // nothing happens
                }
            }
            ModuleType::Conjunction(ref mut memory) => {
                let pulse = memory.get_mut(sender).unwrap();
                *pulse = ptype;

                let ptype = if memory.values().all(|p| *p == PulseType::High) {
                    PulseType::Low
                } else {
                    PulseType::High
                };

                for recipient in &self.destinations {
                    pulses.push_back(Pulse {
                        sender: self.name,
                        ptype,
                        recipient,
                    });
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Puzzle<'a> {
    broadcaster: Vec<&'a str>,
    modules: HashMap<&'a str, Module<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseType {
    Low,
    High,
}

impl Not for PulseType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PulseType::Low => PulseType::High,
            PulseType::High => PulseType::Low,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pulse<'a> {
    sender: &'a str,
    ptype: PulseType,
    recipient: &'a str,
}

fn parse_destinations(destinations: &str) -> Vec<&str> {
    destinations.split(", ").collect()
}

fn parse_input(input: &str) -> Puzzle {
    let mut map = HashMap::new();
    let mut broadcaster: Option<Vec<&str>> = None;

    for line in input.trim().lines() {
        match line.chars().next().unwrap().try_into() {
            Ok(mtype) => {
                let name = &line[1..line.find(" ").unwrap()];
                let destinations = parse_destinations(&line[line.rfind(" -> ").unwrap() + 4..]);
                map.insert(
                    name,
                    Module {
                        name,
                        mtype,
                        destinations,
                    },
                );
            }
            Err(_) => {
                if &line[..line.find(" ").unwrap()] == "broadcaster" {
                    let destinations = parse_destinations(&line[line.rfind(" -> ").unwrap() + 4..]);
                    broadcaster = Some(destinations);
                }
            }
        }
    }

    // subscribe inputs to conjunctions
    // have to copy map for rust aliasing reasons
    // there is probably a more elegant way
    // but oof
    let oof = map.clone();
    for module in oof.values() {
        for conjunction in &module.destinations {
            if let Some(m) = map.get_mut(conjunction) {
                match m.mtype {
                    ModuleType::Conjunction(ref mut memory) => {
                        memory.insert(module.name, PulseType::Low);
                    }
                    _ => continue,
                }
            } // else module is a named output
        }
    }

    Puzzle {
        broadcaster: broadcaster.unwrap(),
        modules: map,
    }
}

fn cycle<'a>(puzzle: &'a mut Puzzle, cycles: usize) -> usize {
    let mut pulses = VecDeque::new();
    let mut log: Vec<Pulse> = Vec::new();

    for _ in 0..cycles {
        for module in &puzzle.broadcaster {
            puzzle.modules.get_mut(module).unwrap().send_signal(
                PulseType::Low,
                module,
                &mut pulses,
            );

            log.push(Pulse {
                sender: "broadcaster",
                ptype: PulseType::Low,
                recipient: module,
            });
        }

        while let Some(pulse) = pulses.pop_front() {
            if let Some(m) = puzzle.modules.get_mut(pulse.recipient) {
                m.send_signal(pulse.ptype, pulse.sender, &mut pulses);
            } // else the module is a named output

            log.push(pulse);
        }
    }

    // + cycles for the button pulse(s)
    let low_pulses = log.iter().filter(|p| p.ptype == PulseType::Low).count() + cycles;
    let high_pulses = log.iter().filter(|p| p.ptype == PulseType::High).count();

    low_pulses * high_pulses
}

fn part1(input: &str) -> usize {
    let mut puzzle = parse_input(input);

    cycle(&mut puzzle, 1000)
}

fn part2(input: &str) -> usize {
    0
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 32000000;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_2() {
        let expected = 11687500;
        let actual = part1(EXAMPLE2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let actual = part2(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
