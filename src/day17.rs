use std::collections::HashMap;

/*
 * The board and pieces both use a simple inverted coordinate system:
 * board.data[0] is the lowest row in the board, board.data[1] is the
 * second row, and so on. This makes iterating over rows straightforward.
 *
 * The data itself is stored in bitmaps: both the board and the pieces
 * are just vectors of u8's. The board is only 7 columns wide, so only
 * the low 7 bits of each u8 is actually used. This means that checking
 * whether a piece intersects a spot on the board is just a bitwise-AND.
 * It also makes it pretty easy to move pieces left and right with shifts.
 */
struct Board {
    data: Vec<u8>,
}

impl Board {
    fn height(&self) -> usize {
        self.data.len()
    }

    /**
     * Checks whether a piece can be placed at a particular height.
     */
    fn can_place(&self, piece: &Piece, base_height: usize) -> bool {
        for row in 0..piece.height() {
            // If we've gone off the top of the board, there's nothing
            // for the piece to run into, so we're done.
            if base_height + row >= self.height() {
                break;
            }

            // If the piece and the board row have a 1 bit in common,
            // then they intersect, and the piece can't be placed there.
            if self.data[base_height + row] & piece.data[row] != 0 {
                return false;
            }
        }

        true
    }

    /**
     * Adds a piece to the board at a given height, adding new rows
     * to the board if needed to contain the added piece.
     *
     * Because we only add new rows to the board when needed, we
     * can always find the height of the tower by just checking
     * self.board.len().
     */
    fn add_piece(&mut self, piece: &Piece, base_height: usize) {
        for row in 0..piece.height() {
            if base_height + row >= self.height() {
                self.data.push(0b0000000);
            }

            self.data[base_height + row] |= piece.data[row];
        }
    }

    /**
     * Simulates dropping a new piece into the board.
     *
     * Because wind is preserved across drops, this returns the updated wind index. ("windex"?)
     */
    fn drop(&mut self, initial_piece: &Piece, winds: &[u8], initial_wind: usize) -> usize {
        let mut piece = initial_piece.clone();

        // Pieces always start at 3 above the highest point on the board.
        let mut piece_y = self.height() + 3;

        let mut wind = initial_wind;
        loop {
            let shifted = match winds[wind] {
                b'<' => piece.shifted_left(),
                b'>' => piece.shifted_right(),
                _ => unreachable!(),
            };

            wind = (wind + 1) % winds.len();

            // Check whether we're able to move this piece in the
            // direction of the wind, and update it if so.
            if self.can_place(&shifted, piece_y) {
                piece = shifted;
            }

            // Check whether moving the piece downwards would cause it to
            // intersect with an already-placed piece. If so, we're done.
            if piece_y == 0 || !self.can_place(&piece, piece_y - 1) {
                self.add_piece(&piece, piece_y);
                break;
            }

            // Otherwise, move the piece down one unit.
            piece_y -= 1;
        }

        wind
    }
}

/*
 * A Piece is basically just a tiny Board: it's a vector of u8's with
 * some utility functions attached.
 */
#[derive(Clone)]
struct Piece {
    data: Vec<u8>,
}

impl Piece {
    fn height(&self) -> usize {
        self.data.len()
    }

    /**
     * Checks whether a given pixel/location on this piece is filled with rock.
     * Naming this stuff is hard.
     */
    fn filled(&self, row: usize, col: usize) -> bool {
        let mask = 1 << col;
        self.data[row] & mask != 0
    }

    /**
     * Tries to shift this piece to the left. This might not be possible if
     * doing so would cause the piece to run into the wall of the board.
     *
     * Returns a new Piece representing the (possibly-)shifted original.
     */
    fn shifted_left(&self) -> Self {
        // We cannot shift left if any part of the piece is
        // already in the leftmost (6th) column.
        for i in 0..self.height() {
            if self.filled(i, 6) {
                return self.clone();
            }
        }

        Piece {
            data: self.data.iter().map(|r| r << 1).collect(),
        }
    }

    /**
     * Tries to shift this piece to the right. This might not be possible if
     * doing so would cause the piece to run into the wall of the board.
     *
     * Returns a new Piece representing the (possibly-)shifted original.
     */
    fn shifted_right(&self) -> Self {
        // We cannot shift left if any part of the piece is
        // already in the rightmost (0th) column.
        for i in 0..self.height() {
            if self.filled(i, 0) {
                return self.clone();
            }
        }

        Piece {
            data: self.data.iter().map(|r| r >> 1).collect(),
        }
    }
}

/**
 * Returns a list of the pieces as they first appear when dropped.
 * 
 * I wanted to make this a constant, but Rust didn't like that. Ah well.
 * They're written out in a long format so it's easier to see the
 * mapping between bits and piece shapes.
 * 
 * Two things to note:
 *   - Because pieces always appear 2 units from the left wall, the high
 *     two bits are always zero, and at least one row has the next bit set.
 *   - The byte ordering is reversed from the visuals, since in our
 *     coordinate system a low index means that the row appears *lower*.
 *     This only matters for the L shape since the others are mirrored vertically.
 */
#[rustfmt::skip]
fn base_pieces() -> Vec<Piece> {
    vec![
        Piece { data: vec![0b0011110] },
        Piece { 
            data: vec![
                0b0001000, 
                0b0011100, 
                0b0001000
            ] 
        },
        // Note that the L piece looks upside down!
        // This is to match the coordinate system used by the board, where
        // lower-indexed rows have lower y-coordinates.
        Piece { 
            data: vec![
                0b0011100, 
                0b0000100, 
                0b0000100
            ]   
        },
        Piece { 
            data: vec![
                0b0010000, 
                0b0010000, 
                0b0010000, 
                0b0010000
            ] 
        },
        Piece {     
            data: vec![
                0b0011000, 
                0b0011000
            ]
        },
    ]
}

/*
 * Part 1 is pretty straightforward, given all the work we did above.
 * We just have to set things up, simulate 2022 drops, and check the height.
 */
#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let winds = input.trim().as_bytes();
    let mut wind = 0;

    let mut board = Board { data: Vec::new() };

    let pieces = base_pieces();

    for num_pieces in 0..2022 {
        let piece = &pieces[num_pieces % pieces.len()];
        wind = board.drop(piece, winds, wind);
    }

    board.height()
}

/*
 * For Part 2, we won't be able to simulate dropping a trillion pieces. So we
 * need to take a shortcut.
 *
 * The key insight is that we're doing a lot of things repetitively: the pieces
 * cycle every 5 drops, the winds cycle every so often, and so on. And, since
 * where a piece ends up is determined entirely by the top few rows of the board,
 * it's not too hard to imagine that the state of those rows might repeat as well.
 *
 * So, our goal is to find two points at which all of those things repeat. Then we
 * can skip almost all of the actual simulation, and replace it with arithmetic!
 *
 * The steps will look something like this:
 * 1. Drop a bunch of pieces until the first cycle starts.
 * 2. Go through the cycle many many many times. Each repetition uses a known
 *    number of blocks, and generates a known additional height.
 * 3. The last cycle probably won't end right at 1 trillion blocks, so simulate
 *    adding the last few blocks to see how much height we get.
 */

/**
 * This is the state we store to check for repeats.
 *
 * It contains the current piece ID, the current wind state, and a copy of the top
 * several rows of the board.
 */
#[derive(Hash, PartialEq, Eq)]
struct State {
    piece: usize,
    gust: usize,
    board: Vec<u8>,
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let winds = input.trim().as_bytes();
    let mut wind = 0;

    let mut board = Board { data: Vec::new() };

    let pieces = base_pieces();

    // To find a cycle, we need to track our board states.
    // This maps a State object to a pair (# pieces dropped, board height).
    let mut visited_states: HashMap<State, (usize, usize)> = HashMap::new();

    // Once we find a cycle, we'll be able to figure out how tall the tower
    // is at the end of the last full cycle before a trillion drops, and the
    // number of pieces left to actually get all the way there.
    let height_after_last_full_cycle;
    let pieces_remaining;

    let mut num_pieces = 0;
    loop {
        let piece = &pieces[num_pieces % pieces.len()];

        wind = board.drop(piece, winds, wind);
        num_pieces += 1;

        // We can't grab the board state if there's not enough board state to grab!
        // It's *very* unlikely that the first cycle will start this early anyways.
        if board.height() < 30 {
            continue;
        }

        // Grab the board state.
        let board_data = board
            .data
            .iter()
            .skip(board.height() - 30)
            .cloned()
            .collect();
        let state = State {
            piece: num_pieces % pieces.len(),
            gust: wind,
            board: board_data,
        };

        if let Some((previous_num_pieces, previous_height)) =
            visited_states.insert(state, (num_pieces, board.height()))
        {
            let cycle_length = num_pieces - previous_num_pieces;

            // By construction, the first cycle starts at `previous_num_pieces`.
            // We need to make sure that we don't count those first few drops when figuring
            // out how many times the cycle repeated.
            let num_cycles = (1_000_000_000_000 - previous_num_pieces) / cycle_length;
            let height_per_cycle = board.height() - previous_height;

            height_after_last_full_cycle = previous_height + num_cycles * height_per_cycle;
            pieces_remaining = (1_000_000_000_000 - previous_num_pieces) % cycle_length;

            break;
        }
    }

    // Simulate the last few pieces.
    let height_after_cycle = board.height();
    for i in 0..pieces_remaining {
        let piece = &pieces[(i + num_pieces) % pieces.len()];
        wind = board.drop(piece, winds, wind);
    }

    // Figure out how much additional height those last few pieces added.
    let extra_board_height = board.height() - height_after_cycle;

    // Put it all together!
    height_after_last_full_cycle + extra_board_height
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 1514285714288);
    }
}
