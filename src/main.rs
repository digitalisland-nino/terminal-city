extern crate pancurses;
mod board;
mod entities;
mod logo;

use board::{board::*, grid::TILE_SIZE};

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

fn draw_board(window: &mut Window, board: &Board) -> Window {
    let tile_y = TILE_SIZE[0];
    let tile_x = TILE_SIZE[1];

    let board_height = 10 * tile_y;
    let board_width = 10 * tile_x;

    let max_y = window.get_max_y();
    let max_x = window.get_max_x();

    let center_y = ((max_y + HEADER_HEIGHT) / 2) - board_height / 2;
    let center_x = (max_x / 2) - board_width / 2;

    let board_window = newwin(board_height, board_width, center_y, center_x);

    board_window.draw_box(0, 0);
    board_window.refresh();

    //let new_tile = board_window.subwin(tile_y, tile_x, 0, 0);
    //let tile_window = new_tile.expect("Failed to derive subwindow for tile.");

    //tile_window.draw_box(0, 0);
    //tile_window.refresh();
    board_window.touch();

    for tile in &board.row_tiles {}

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

    noecho();
    cbreak();

    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, Some(&mut 0));

    window.refresh();
    //window.draw_box(0, 0);

    draw_logo(&mut window);
    draw_instructions(&mut window);
    let board_window = draw_board(&mut window, &board);

    let board_dimensions = (
        board_window.get_beg_y(),
        board_window.get_beg_x() + 3,
        (board_window.get_max_y() as f64 * 1.4) as i32 - 2,
        (board_window.get_max_x() as f64 * 1.5) as i32 - 4,
    );
    println!("{:?}", board_dimensions);
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
}
