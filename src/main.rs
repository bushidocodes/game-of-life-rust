// Any live cell with fewer than two live neighbors dies, as if caused by under population.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by overpopulation.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
use rand::Rng;

fn main() {
    const dimension : i8 = 16;

    #[derive(Debug)]
    enum Cell {
        Alive,
        Dead,
    }

    let mut gameboard: Vec<Vec<Cell>> = Vec::new();
    
    fn buildGameBoard(mut gameboard: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        let mut rng = rand::thread_rng();
        for row in 0..dimension {
            let mut row: Vec<Cell> = Vec::new();
            for cell in 0..dimension {
                let isAlive: i8 = rng.gen_range(0, 2);
                if isAlive == 1 {
                    row.push(Cell::Alive);
                } else {
                    row.push(Cell::Dead);
                }
            }
            gameboard.push(row);
        }
        gameboard
    }

    fn drawGameBoard(gameboard: Vec<Vec<Cell>>) {
        for row in gameboard {
            for cell in row {
                match cell {
                    Cell::Alive => print!(" * "),
                    Cell::Dead => print!("   "),
                }
            }
            println!();
        }

    }
    gameboard = buildGameBoard(gameboard);
    println!("{:?}", gameboard);
    drawGameBoard(gameboard);
}
