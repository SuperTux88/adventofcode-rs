use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    io::{self, BufRead},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use pathfinding::prelude::bfs;

use crate::day::AoCDay;

#[derive(Debug, Clone)]
struct Valve {
    id: u16,
    flow_rate: u8,
    tunnels: Vec<u16>,
}

const START: u16 = convert_id("AA");

const fn convert_id(id: &str) -> u16 {
    let id = id.as_bytes();
    (((id[0] - b'A') as u16) << 5) + (id[1] - b'A') as u16
}

fn valve_id(input: &str) -> IResult<&str, u16> {
    let (input, id) = alpha1(input)?;
    Ok((input, convert_id(id)))
}
fn valve(input: &str) -> IResult<&str, Valve> {
    let (input, (id, flow_rate, tunnels)) = tuple((
        preceded(tag("Valve "), valve_id),
        preceded(tag(" has flow rate="), complete::u8),
        preceded(
            alt((
                tag("; tunnel leads to valve "),
                tag("; tunnels lead to valves "),
            )),
            separated_list1(tag(", "), valve_id),
        ),
    ))(input)?;

    Ok((
        input,
        Valve {
            id,
            flow_rate,
            tunnels,
        },
    ))
}
fn valves(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, valve)(input)
}

pub struct Solution {
    valves_with_flow: HashMap<u16, u8>,
    valve_distances: HashMap<(u16, u16), u8>,
}

impl AoCDay for Solution {
    fn title() -> &'static str {
        "Proboscidea Volcanium"
    }

    fn with_input(input: &mut impl BufRead) -> Self {
        let input = io::read_to_string(input).unwrap();
        let (_, valves) = valves(input.as_str()).unwrap();

        let tunnels = valves
            .iter()
            .map(|valve| (valve.id, valve.tunnels.clone()))
            .collect::<HashMap<u16, Vec<u16>>>();

        let valves_with_flow = valves
            .into_iter()
            .filter(|valve| valve.flow_rate > 0)
            .collect::<Vec<Valve>>();

        let mut valves_to_visit = valves_with_flow
            .iter()
            .map(|valve| valve.id)
            .collect::<Vec<u16>>();
        valves_to_visit.push(START);

        let mut valve_distances = HashMap::new();
        valves_to_visit.into_iter().combinations(2).for_each(|v| {
            if let &[from, to] = v.as_slice() {
                let distance = get_distance(from, to, &tunnels);
                valve_distances.insert((from, to), distance);
                valve_distances.insert((to, from), distance);
            }
        });

        let valves_with_flow = valves_with_flow
            .iter()
            .map(|valve| (valve.id, valve.flow_rate))
            .collect::<HashMap<u16, u8>>();

        Self {
            valves_with_flow,
            valve_distances,
        }
    }

    fn part1(&self) -> String {
        self.max_pressure(30).values().max().unwrap().to_string()
    }

    fn part2(&self) -> String {
        //todo!();
        "TODO".to_string()
    }
}

impl Solution {
    fn max_pressure(&self, minutes: u8) -> HashMap<BTreeSet<u16>, u16> {
        let mut states_per_minute = vec![HashMap::new(); minutes as usize];
        states_per_minute[0].insert(
            State {
                valve: START,
                open: BTreeSet::new(),
            },
            0u16,
        );

        for min in 0..minutes {
            states_per_minute[min as usize]
                .clone()
                .iter()
                .for_each(|(state, current_pressure)| {
                    self.valves_with_flow
                        .iter()
                        .filter(|(v, _)| !state.open.contains(v))
                        .for_each(|(new_valve, new_flow)| {
                            let open_minute =
                                min + self.valve_distances[&(state.valve, *new_valve)] + 1;
                            if open_minute < minutes {
                                let new_pressure = current_pressure
                                    + *new_flow as u16 * (minutes - open_minute) as u16;
                                let new_state = state.open(*new_valve);
                                let old_pressure = states_per_minute[open_minute as usize]
                                    .get(&new_state)
                                    .cloned()
                                    .unwrap_or_default();
                                states_per_minute[open_minute as usize]
                                    .insert(new_state, old_pressure.max(new_pressure));
                            }
                        });
                });
        }

        states_per_minute
            .into_iter()
            .flatten()
            .into_grouping_map_by(|(s, _)| s.open.clone())
            .aggregate(|acc, _open, (_state, pressure)| Some(acc.unwrap_or(0).max(pressure)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    valve: u16,
    open: BTreeSet<u16>,
}

impl State {
    fn open(&self, valve: u16) -> Self {
        let mut open = self.open.clone();
        open.insert(valve);
        Self { valve, open }
    }
}

fn get_distance(from: u16, to: u16, tunnels: &HashMap<u16, Vec<u16>>) -> u8 {
    let path = bfs(
        &from,
        |&valve| tunnels.get(&valve).unwrap().clone(),
        |&valve| valve == to,
    );
    path.unwrap().len() as u8 - 1 // path includes the start, but the start doesn't add to the distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_convert_id() {
        assert_eq!(convert_id("AA"), 0);
        assert_eq!(convert_id("AB"), 1);
        assert_eq!(convert_id("BA"), 32);
        assert_eq!(convert_id("BB"), 33);
        assert_eq!(convert_id("ZZ"), 825);
    }

    #[test]
    fn test_part1_example() {
        let day = Solution::with_input(input!(example: 2022 16));
        assert_eq!(day.part1(), "1651");
    }

    #[test]
    fn test_part1_input() {
        let day = Solution::with_input(input!(input: 2022 16));
        assert_eq!(day.part1(), "1767");
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let day = Solution::with_input(input!(example: 2022 16));
        assert_eq!(day.part2(), "1707");
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let day = Solution::with_input(input!(input: 2022 16));
        assert_eq!(day.part2(), "2528");
    }
}
