extern crate pancurses;
mod board;
mod entities;
mod logo;

use board::{
    board::*,
    grid::{Tile, SUB_TILE_SIZE, TILE_SIZE},
};

use pancurses::{
    cbreak, endwin, getmouse, initscr, mousemask, newwin, noecho, resize_term, Input, Window,
    ALL_MOUSE_EVENTS, REPORT_MOUSE_POSITION,
};
use std::convert::TryInto;

const HEADER_HEIGHT: i32 = 14;

fn draw_logo(window: &mut Window) {
    window.mvprintw(1, 63, logo::logo_line_1());
    window.mvprintw(2, 62, logo::logo_line_2());
    window.mvprintw(3, 61, logo::logo_line_3());
    window.mvprintw(4, 60, logo::logo_line_4());
    window.mvprintw(5, 60, logo::logo_line_5());
    window.mvprintw(6, 76, logo::logo_line_6());
    window.mvprintw(7, 75, logo::logo_line_7());
    window.mvprintw(8, 74, logo::logo_line_8());
    window.mvprintw(9, 74, logo::logo_line_9());
}

fn draw_instructions(window: &mut Window) {
    let instructions = "Click in the terminal to place a house, press q or ctrl+c to exit\n";
    let mut separator: String = "".to_string();

    for _ in 1..instructions.len() {
        separator.push('=');
    }

    window.mvprintw(11, 50, &separator);
    window.mvprintw(12, 50, instructions);
    window.mvprintw(13, 50, separator);
}

fn store_tile(tile_vector: &mut Vec<Tile>, new_tile: Tile) {
    tile_vector.push(new_tile);
}

fn draw_sub_tile(window: &mut Window, y: i32, x: i32) {
    let initialized_sub_tile = window.derwin(SUB_TILE_SIZE[0], SUB_TILE_SIZE[1], y, x);

    let sub_tile = match initialized_sub_tile {
        Ok(sub_tile_window) => sub_tile_window,
        Err(error) => panic!("Failed to create sub_tile subwindow: {:?}", error),
    };

    sub_tile.touch();
    //sub_tile.draw_box(0, 0);
    sub_tile.refresh();
}

fn draw_tile(window: &mut Window, y: i32, x: i32, mut all_sub_tiles: &mut Vec<Tile>) {
    let initialized_tile = window.derwin(TILE_SIZE[0], TILE_SIZE[1], y + 1, x + 2);

    let mut tile = match initialized_tile {
        Ok(tile_window) => tile_window,
        Err(error) => panic!("Failed to create tile subwindow: {:?}", error),
    };

    tile.touch();
    tile.draw_box(0, 0);
    tile.refresh();
    let mut sub_y = 0;
    let mut sub_x = 0;

    for sub_row in 0..SUB_TILE_SIZE[0] {
        for sub_column in 0..SUB_TILE_SIZE[0] {
            let x = sub_y;
            let y = sub_x;
            store_tile(
                &mut all_sub_tiles,
                Tile::new(SUB_TILE_SIZE[0], SUB_TILE_SIZE[1], sub_row, sub_column),
            );
            draw_sub_tile(&mut tile, y, x);
            sub_x += SUB_TILE_SIZE[1] - 2;
        }
        sub_x = 0;
        sub_y += SUB_TILE_SIZE[0];
    }
}

fn draw_board(
    window: &mut Window,
    board: Board,
    mut all_tiles: &mut Vec<Tile>,
    mut all_sub_tiles: &mut Vec<Tile>,
) -> Window {
    let tile_y = TILE_SIZE[0];
    let tile_x = TILE_SIZE[1];

    let board_height = 10 * tile_y;
    let board_width = 10 * tile_x;

    let max_y = window.get_max_y();
    let max_x = window.get_max_x();

    let center_y = ((max_y + HEADER_HEIGHT) / 2) - board_height / 2;
    let center_x = (max_x / 2) - board_width / 2;

    let mut board_window = newwin(board_height, board_width, center_y, center_x);

    board_window.draw_box(0, 0);
    board_window.refresh();

    let mut row_position = 0;
    let mut column_position = 0;

    for _row in &board.row_tiles {
        for _column in &board.column_tiles {
            let x = column_position;
            let y = row_position;
            store_tile(
                &mut all_tiles,
                Tile::new(SUB_TILE_SIZE[0], SUB_TILE_SIZE[1], y, x),
            );
            draw_tile(&mut board_window, y, x, &mut all_sub_tiles);
            column_position += TILE_SIZE[1];
        }
        column_position = 0;
        row_position += TILE_SIZE[0];
    }
    board_window.touch();

    board_window
}

fn main() {
    let mut window: Window = initscr();

    let width = window.get_max_y();
    let height = window.get_max_x();

    let board = Board::new(Size {
        y: width,
        x: height,
    });

    let mut all_tiles: Vec<Tile> = Vec::new();
    let mut all_sub_tiles: Vec<Tile> = Vec::new();

    noecho();
    cbreak();

    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, Some(&mut 0));

    window.refresh();
    //window.draw_box(0, 0);

    draw_logo(&mut window);
    draw_instructions(&mut window);
    let board_window = draw_board(&mut window, board, &mut all_tiles, &mut all_sub_tiles);

    let board_dimensions = (
        board_window.get_beg_y(),
        board_window.get_beg_x() + 3,
        (board_window.get_max_y() as f64 * 1.4) as i32 - 2,
        (board_window.get_max_x() as f64 * 1.5) as i32 - 4,
    );
    window.mvprintw(18, 43, "*");
    window.mvprintw(54, 116, "*");

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    let inside_board = mouse_event.y > board_dimensions.0
                        && mouse_event.x > board_dimensions.1
                        && mouse_event.y < board_dimensions.2
                        && mouse_event.x < board_dimensions.3;
                    if inside_board {
                        let house_ascii_art_chimney = entities::house::print_house_chimney();
                        let house_ascii_art_top = entities::house::print_house_top();
                        let house_ascii_art_middle = entities::house::print_house_middle();
                        let house_ascii_art_bottom = entities::house::print_house_bottom();
                        let x_offset: i32 = (&house_ascii_art_middle.len() / 2).try_into().unwrap();
                        let x_position = mouse_event.x as i32 - x_offset;
                        window.mvprintw(mouse_event.y, x_position, &house_ascii_art_chimney);
                        window.mvprintw(mouse_event.y + 1, x_position, &house_ascii_art_top);
                        window.mvprintw(mouse_event.y + 2, x_position, &house_ascii_art_middle);
                        window.mvprintw(mouse_event.y + 3, x_position, &house_ascii_art_bottom);
                    }
                }
            }
            Some(Input::Character(x)) if x == 'q' => break,
            //Some(Input::Character(c)) => {
            //    window.addch(c);
            //}
            Some(Input::KeyResize) => {
                resize_term(0, 0);
            }
            Some(Input::KeyDC) => break,
            Some(_input) => {
                //window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }

    endwin();

    let mut tile_count = 0;
    for _tile in all_tiles {
        tile_count += 1;
    }

    let mut sub_tile_count = 0;
    for _tile in all_sub_tiles {
        sub_tile_count += 1;
    }

    println!(
        "Tile count: {:?}, Subtile count: {:?}",
        tile_count, sub_tile_count
    );
}
