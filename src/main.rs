mod game;

use glam::vec2;

use crate::game::Color::White;
use macroquad::prelude::*;


const BACKGROUND: Color = color_u8!(0x16, 0x14, 0x12, 0xff);
const LIGHT_SQUARE: Color = color_u8!(0xf0, 0xd9, 0xb5, 0xff);
const DARK_SQUARE: Color = color_u8!(0xb5, 0x88, 0x63, 0xff);

const SELECTED: Color = color_u8!(20, 85, 30, 0x7f);

const FONT: &[u8] = include_bytes!("../assets/Atkinson-Hyperlegible-Bold-102.ttf");

const BLACK_BISHOP_FILE: &[u8] = include_bytes!("../assets/bB.png");
const BLACK_KING_FILE: &[u8] = include_bytes!("../assets/bK.png");
const BLACK_KNIGHT_FILE: &[u8] = include_bytes!("../assets/bN.png");
const BLACK_PAWN_FILE: &[u8] = include_bytes!("../assets/bP.png");
const BLACK_QUEEN_FILE: &[u8] = include_bytes!("../assets/bQ.png");
const BLACK_ROOK_FILE: &[u8] = include_bytes!("../assets/bR.png");
const WHITE_BISHOP_FILE: &[u8] = include_bytes!("../assets/wB.png");
const WHITE_KING_FILE: &[u8] = include_bytes!("../assets/wK.png");
const WHITE_KNIGHT_FILE: &[u8] = include_bytes!("../assets/wN.png");
const WHITE_PAWN_FILE: &[u8] = include_bytes!("../assets/wP.png");
const WHITE_QUEEN_FILE: &[u8] = include_bytes!("../assets/wQ.png");
const WHITE_ROOK_FILE: &[u8] = include_bytes!("../assets/wR.png");

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        // high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    clear_background(WHITE);
    let font = load_ttf_font_from_bytes(FONT).unwrap();
    draw_text_ex(
        "Loading...",
        screen_width() / 2f32,
        screen_height() / 2f32,
        TextParams {
            font_size: 32,
            color: BLACK,
            font: Some(&font),
            ..Default::default()
        },
    );
    next_frame().await;

    let black_bishop: Texture2D =
        Texture2D::from_file_with_format(BLACK_BISHOP_FILE, Some(ImageFormat::Png));
    let black_king: Texture2D =
        Texture2D::from_file_with_format(BLACK_KING_FILE, Some(ImageFormat::Png));
    let black_knight: Texture2D =
        Texture2D::from_file_with_format(BLACK_KNIGHT_FILE, Some(ImageFormat::Png));
    let black_pawn: Texture2D =
        Texture2D::from_file_with_format(BLACK_PAWN_FILE, Some(ImageFormat::Png));
    let black_queen: Texture2D =
        Texture2D::from_file_with_format(BLACK_QUEEN_FILE, Some(ImageFormat::Png));
    let black_rook: Texture2D =
        Texture2D::from_file_with_format(BLACK_ROOK_FILE, Some(ImageFormat::Png));
    let white_bishop: Texture2D =
        Texture2D::from_file_with_format(WHITE_BISHOP_FILE, Some(ImageFormat::Png));
    let white_king: Texture2D =
        Texture2D::from_file_with_format(WHITE_KING_FILE, Some(ImageFormat::Png));
    let white_knight: Texture2D =
        Texture2D::from_file_with_format(WHITE_KNIGHT_FILE, Some(ImageFormat::Png));
    let white_pawn: Texture2D =
        Texture2D::from_file_with_format(WHITE_PAWN_FILE, Some(ImageFormat::Png));
    let white_queen: Texture2D =
        Texture2D::from_file_with_format(WHITE_QUEEN_FILE, Some(ImageFormat::Png));
    let white_rook: Texture2D =
        Texture2D::from_file_with_format(WHITE_ROOK_FILE, Some(ImageFormat::Png));

    let mut game = game::Game::default();

    let mut moving_piece: Option<game::Square> = None;
    let mut selected_piece: Option<game::Square> = None;

    let draw_piece = |p: &game::Piece, x: f32, y: f32, size: f32, color: Color| {
        draw_texture_ex(
            match p.color {
                game::Color::Black => match p.piece_type {
                    game::PieceType::Pawn => &black_pawn,
                    game::PieceType::Knight => &black_knight,
                    game::PieceType::Bishop => &black_bishop,
                    game::PieceType::Rook => &black_rook,
                    game::PieceType::Queen => &black_queen,
                    game::PieceType::King => &black_king,
                },
                White => match p.piece_type {
                    game::PieceType::Pawn => &white_pawn,
                    game::PieceType::Knight => &white_knight,
                    game::PieceType::Bishop => &white_bishop,
                    game::PieceType::Rook => &white_rook,
                    game::PieceType::Queen => &white_queen,
                    game::PieceType::King => &white_king,
                },
            },
            x,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    };
    loop {
        clear_background(BACKGROUND);
        let width = screen_width();
        let height = screen_height();
        let board_size = f32::min(width, height);
        let top_left = ((width - board_size) / 2f32, (height - board_size) / 2f32);
        let square_size = board_size / 8f32;

        let mouse_pos = mouse_position();
        let row = ((mouse_pos.1 - top_left.1) / square_size).floor() as i8;
        let col = ((mouse_pos.0 - top_left.0) / square_size).floor() as i8;
        let mouse_square_option = game::is_valid_square(&(row, col));

        if is_key_pressed(KeyCode::Z) {
            game.unmake_move_and_recalculate();
        }

        if let Some(mouse_square) = mouse_square_option {
            if is_mouse_button_pressed(MouseButton::Left) {
                let piece_at_square = game.piece_at_square(&mouse_square);
                if let Some(s) = selected_piece {
                    if s != mouse_square {
                        if let Some(p) = piece_at_square {
                            if p.color == game.turn {
                                moving_piece = Some(mouse_square);
                                selected_piece = Some(mouse_square);
                            } else {
                                game.request_move(&s, &mouse_square);
                                moving_piece = None;
                                selected_piece = None;
                            }
                        } else {
                            game.request_move(&s, &mouse_square);
                            moving_piece = None;
                            selected_piece = None;
                        }
                    } else {
                        moving_piece = None;
                        selected_piece = None;
                    }
                } else if let Some(p) = piece_at_square {
                    if p.color == game.turn {
                        moving_piece = Some(mouse_square);
                        selected_piece = Some(mouse_square);
                    } else {
                        moving_piece = None;
                        selected_piece = None;
                    }
                } else {
                    moving_piece = None;
                    selected_piece = None;
                }
            }
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(p) = moving_piece {
                    if p == mouse_square {
                        moving_piece = None;
                        // intentionally don't touch selected piece
                    } else if let Some(s) = selected_piece {
                        game.request_move(&s, &mouse_square);
                        moving_piece = None;
                        selected_piece = None;
                    }
                }
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            moving_piece = None;
            selected_piece = None;
        }

        for row in 0..8 {
            for col in 0..8 {
                let mut selected = false;
                if let Some(m) = selected_piece {
                    if m == (row, col) {
                        selected = true;
                    }
                }
                let (color, inverse_color) = if row % 2 == col % 2 {
                    (LIGHT_SQUARE, DARK_SQUARE)
                } else {
                    (DARK_SQUARE, LIGHT_SQUARE)
                };
                draw_rectangle(
                    top_left.0 + col as f32 * square_size,
                    top_left.1 + row as f32 * square_size,
                    square_size,
                    square_size,
                    color,
                );
                if col == 0 {
                    draw_text_ex(
                        &(8 - row).to_string(),
                        top_left.0 + col as f32 * square_size + square_size / 50f32,
                        top_left.1 + row as f32 * square_size + square_size / 5f32,
                        TextParams {
                            font_size: (square_size / 5f32) as u16,
                            color: inverse_color,
                            font: Some(&font),
                            ..Default::default()
                        },
                    );
                }
                if row == 7 {
                    draw_text_ex(
                        &"abcdefgh".chars().nth((col) as usize).unwrap().to_string(),
                        top_left.0 + col as f32 * square_size + square_size * (6f32 / 7f32),
                        top_left.1 + row as f32 * square_size + square_size * (49f32 / 50f32),
                        TextParams {
                            font_size: (square_size / 5f32) as u16,
                            color: inverse_color,
                            font: Some(&font),
                            ..Default::default()
                        },
                    );
                }
                if selected {
                    draw_rectangle(
                        top_left.0 + col as f32 * square_size,
                        top_left.1 + row as f32 * square_size,
                        square_size,
                        square_size,
                        SELECTED,
                    );
                }
                if let Some(p) = game.piece_at_square(&(row, col)) {
                    // draw moving piece at half opacity
                    let mut color = WHITE;
                    if let Some(m) = moving_piece {
                        if m == (row, col) {
                            color = color_u8!(0xff, 0xff, 0xff, 0x7f);
                        }
                    }
                    draw_piece(
                        p,
                        top_left.0 + col as f32 * square_size,
                        top_left.1 + row as f32 * square_size,
                        square_size,
                        color,
                    );
                };
            }
        }
        // draw selected squares
        if let Some(s) = selected_piece {
            for mov in game.legal_moves_on_square(s) {
                let (row, col) = mov.to;
                let offset = (
                    top_left.0 + col as f32 * square_size,
                    top_left.1 + row as f32 * square_size,
                );
                if mov.capture.is_some() {
                    // a capturing move
                    draw_triangle(
                        vec2(offset.0, offset.1),
                        vec2(offset.0 + square_size / 4f32, offset.1),
                        vec2(offset.0, offset.1 + square_size / 4f32),
                        SELECTED,
                    );
                    draw_triangle(
                        vec2(offset.0 + square_size, offset.1),
                        vec2(offset.0 + square_size - square_size / 4f32, offset.1),
                        vec2(offset.0 + square_size, offset.1 + square_size / 4f32),
                        SELECTED,
                    );
                    draw_triangle(
                        vec2(offset.0, offset.1 + square_size),
                        vec2(offset.0 + square_size / 4f32, offset.1 + square_size),
                        vec2(offset.0, offset.1 + square_size - square_size / 4f32),
                        SELECTED,
                    );
                    draw_triangle(
                        vec2(offset.0 + square_size, offset.1 + square_size),
                        vec2(
                            offset.0 + square_size - square_size / 4f32,
                            offset.1 + square_size,
                        ),
                        vec2(
                            offset.0 + square_size,
                            offset.1 + square_size - square_size / 4f32,
                        ),
                        SELECTED,
                    );
                } else {
                    // empty square
                    draw_circle(
                        offset.0 + square_size / 2f32,
                        offset.1 + square_size / 2f32,
                        square_size / 10f32,
                        SELECTED,
                    )
                }
            }
        }

        // draw held piece
        if let Some(p) = moving_piece {
            if let Some(m) = game.piece_at_square(&p) {
                draw_piece(
                    m,
                    mouse_pos.0 - square_size / 2f32,
                    mouse_pos.1 - square_size / 2f32,
                    square_size,
                    WHITE,
                )
            }
        }
        next_frame().await;
    }
}
