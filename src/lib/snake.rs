use crate::lib::types::{Cell, SnakeHead, Grid};

pub fn init(x: i32, y: i32, cell: Cell ) -> SnakeHead {
    let snake = SnakeHead{
        row: x,
        column: y,
        cell,
    };
    snake
}

pub fn update_position(
    snake: &mut SnakeHead,
    direction: (i32, i32)
) -> SnakeHead {
    SnakeHead{
        row: snake.row + direction.0,
        column: snake.column + direction.1,
        cell: Cell {
            red: snake.cell.red,
            green: snake.cell.green,
            blue: snake.cell.blue,
        },
    }
}

pub fn fill_move(
    mut grid: Grid,
    snake: &SnakeHead,
) -> Grid {
    let cell_color = Cell{
        red: snake.cell.red,
        green: snake.cell.green,
        blue: snake.cell.blue,
    };

    grid.grid[snake.row as usize][snake.column as usize] = cell_color;
    grid
}
