use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    io::{Error, Read},
    ops::{Deref, DerefMut, Not},
};

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
    fn send_signal(&mut self, ptype: PulseType, sender: &str) -> Vec<Pulse<'a>> {
        match self.mtype {
            ModuleType::FlipFlop(ref mut state) => {
                match ptype {
                    PulseType::Low => {
                        let ptype = match state {
                            State::Off => PulseType::High,
                            State::On => PulseType::Low,
                        };

                        *state = !*state;

                        self.destinations
                            .iter()
                            .map(|recipient| Pulse {
                                sender: self.name,
                                ptype,
                                recipient,
                            })
                            .collect()
                    }
                    PulseType::High => vec![], // nothing happens
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

                self.destinations
                    .iter()
                    .map(|recipient| Pulse {
                        sender: self.name,
                        ptype,
                        recipient,
                    })
                    .collect()
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

fn press_button<'a>(puzzle: &mut Puzzle<'a>) -> Vec<Pulse<'a>> {
    let mut pulses = VecDeque::new();
    let mut result = Vec::new();

    for module in &puzzle.broadcaster {
        pulses.extend(
            puzzle
                .modules
                .get_mut(module)
                .unwrap()
                .send_signal(PulseType::Low, module),
        );

        result.push(Pulse {
            sender: "broadcaster",
            ptype: PulseType::Low,
            recipient: module,
        });
    }

    while let Some(pulse) = pulses.pop_front() {
        if let Some(m) = puzzle.modules.get_mut(pulse.recipient) {
            pulses.extend(m.send_signal(pulse.ptype, pulse.sender));
        } // else the module is a named output

        result.push(pulse);
    }

    result
}

fn cycle<'a>(puzzle: &'a mut Puzzle, cycles: usize) -> usize {
    let mut log: Vec<Pulse> = Vec::new();

    for _ in 0..cycles {
        log.extend(press_button(puzzle));
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
    let mut puzzle = parse_input(input);

    // So basically you were supposed to figure out that in the input,
    // the only way for rx to get a signal is by getting it from a single conjunction
    // (for my input that conjunction is called gf).
    // Then you should figure out that this conjunction only depends on 4 other conjunctions
    // (in my case called sp, pg, sv and qs) and if you look further,
    // then you'll even notive that these each only depend on 1 conjunction
    // (rn, pz, jt and mh for my input),
    // though that last fact seems irrelevant.
    //
    // What you can then do is measure how long it takes for the 4 inputs of gf to send a high
    // pulse, because for gf to send a low pulse, all it needs is to get a high pulse from all of
    // its 4 inputs. The actual result is then the least common multiple of those 4 results.
    // Since I am lazy, I hardcoded my 4 gf inputs. Maybe I'll improve it at some point.
    //
    // Also notice how we don't need to actually need to use an lcm function,
    // as those 4 inputs seem to be coprime.
    // However, I did anyway as I think it's nicer.
    let mut counter = 0;
    let mut sp = 0;
    let mut pg = 0;
    let mut sv = 0;
    let mut qs = 0;
    loop {
        counter += 1;

        let pulses = press_button(&mut puzzle);

        if sp == 0
            && pulses
                .iter()
                .any(|p| p.sender == "sp" && p.ptype == PulseType::High)
        {
            sp = counter;
        }

        if pg == 0
            && pulses
                .iter()
                .any(|p| p.sender == "pg" && p.ptype == PulseType::High)
        {
            pg = counter;
        }

        if sv == 0
            && pulses
                .iter()
                .any(|p| p.sender == "sv" && p.ptype == PulseType::High)
        {
            sv = counter;
        }

        if qs == 0
            && pulses
                .iter()
                .any(|p| p.sender == "qs" && p.ptype == PulseType::High)
        {
            qs = counter;
        }

        if sp > 0 && pg > 0 && sv > 0 && qs > 0 {
            break;
        }
    }

    lcm(lcm(sp, pg), lcm(sv, qs))
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
}
