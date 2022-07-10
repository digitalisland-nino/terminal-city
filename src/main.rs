extern crate pancurses;
mod entities;
mod logo;

use pancurses::{
    cbreak, endwin, getmouse, initscr, mousemask, noecho, resize_term, Input, Window, ACS_HLINE,
    ALL_MOUSE_EVENTS, REPORT_MOUSE_POSITION,
};
use std::convert::TryInto;

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

fn main() {
    let mut window = initscr();

    noecho();
    cbreak();

    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, Some(&mut 0));

    window.refresh();
    window.draw_box(0, 0);

    draw_logo(&mut window);
    draw_instructions(&mut window);

    loop {
        match window.getch() {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() {
                    if mouse_event.y > 14 {
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
            Some(input) => {
                //window.addstr(&format!("{:?}", input));
            }
            None => (),
        }
    }

    endwin();
}
