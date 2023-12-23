use super::*;

#[test]
fn construct_grid() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Tile {
        Hash,
        Dot,
    }

    impl TryFrom<char> for Tile {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '#' => Ok(Self::Hash),
                '.' => Ok(Self::Dot),
                _ => Err("unknown tile".to_string()),
            }
        }
    }

    let input = "##..##..##
.##....#..
##########
..........
.##..#...#
";

    let grid: Grid<Tile> = Grid::try_from(input).unwrap();

    assert_eq!(grid.columns, 10);
    assert_eq!(grid.rows, 5);

    let expected = vec![
        Tile::Hash,
        Tile::Hash,
        Tile::Dot,
        Tile::Dot,
        Tile::Hash,
        Tile::Hash,
        Tile::Dot,
        Tile::Dot,
        Tile::Hash,
        Tile::Hash,
        Tile::Dot,
        Tile::Hash,
        Tile::Hash,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Hash,
        Tile::Dot,
        Tile::Dot,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Hash,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Hash,
        Tile::Hash,
        Tile::Dot,
        Tile::Dot,
        Tile::Hash,
        Tile::Dot,
        Tile::Dot,
        Tile::Dot,
        Tile::Hash,
    ];

    assert_eq!(grid.grid, expected);
}
