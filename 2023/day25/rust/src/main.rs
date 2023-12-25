use rand::prelude::*;
use std::{collections::HashSet, io::Read};

type Graph = (V, E);

type V = HashSet<String>;
type E = Vec<(String, String)>;

fn parse_input(input: &str) -> Graph {
    let mut vertices = HashSet::new();
    let mut edges = Vec::new();

    for line in input.trim().lines() {
        let mut it = line.split(':');

        let left = it.next().unwrap();
        vertices.insert(left.to_owned());

        for right in it.next().unwrap().trim().split_whitespace() {
            vertices.insert(right.to_owned());
            edges.push((left.to_owned(), right.to_owned()));
        }
    }

    (vertices, edges)
}

fn part1(input: &str) -> usize {
    let (vertices, edges) = parse_input(input);

    let mut rng = rand::thread_rng();

    // Karger's Algorithm
    loop {
        let mut vertices = vertices.clone();
        let mut edges = edges.clone();
        while vertices.len() > 2 {
            let i = rng.gen_range(0..edges.len());
            let (v1, v2) = edges[i].clone();

            // contract the edge
            edges.swap_remove(i);
            vertices.remove(&v1);
            vertices.remove(&v2);

            let new_v = format!("{}:{}", v1, v2);
            vertices.insert(new_v.clone());

            for (e1, e2) in edges.iter_mut() {
                if *e1 == v1 || *e1 == v2 {
                    *e1 = new_v.clone()
                }
                if *e2 == v1 || *e2 == v2 {
                    *e2 = new_v.clone()
                }
            }

            // remove loops
            let mut j = 0;
            while j < edges.len() {
                let (e1, e2) = &edges[j];
                if e1 == e2 {
                    edges.swap_remove(j);
                } else {
                    j += 1;
                }
            }
        }

        if edges.len() == 3 {
            break vertices
                .iter()
                .map(|s| s.split(':').count())
                .product::<usize>();
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn test_part1() {
        let expected = 54;
        let actual = part1(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
