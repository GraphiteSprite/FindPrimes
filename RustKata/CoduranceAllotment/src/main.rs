fn main() {
    let size:i32 = 5;
    let allotment:Vec<Vec<bool>> = vec![vec![]];
}

struct Cell {
    x: usize,
    y: usize,
}

fn has_neighbour(cell:Cell, allotment: &[Vec<bool>]) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_have_neighbour() {
        let allotment= vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ];
        let result = has_neighbour(
            Cell{
                x:1,
                y:1,
            },
        &allotment
        );

        assert_eq!(result, false);
    }
    #[test]
    fn does_not_have_neighbour() {
        let allotment= vec![
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ];

        let result = has_neighbour(
            Cell{
                x:1,
                y:1,
            },
            &allotment
        );

        assert_eq!(result, true);
    }
}

