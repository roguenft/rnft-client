mod utils;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub struct Map {
struct Map {
    width: u32,
    height: u32,
    tiles: Vec<char>,
}
impl Map {
    fn new(width: u32, height: u32) -> Map {

        // 0	1	2	3	4	5	6	7	8	9	10
        // 11	12	13	14	15	16	17	18	19	20	21
        // 22	23	24	25	26	27	28	29	30	31	32
        // 33	34	35	36	37	38	39	40	41	42	43
        // 44	45	46	47	48	49	50	51	52	53	54
        // 55	56	57	58	59	60	61	62	63	64	65
        // 66	67	68	69	70	71	72	73	74	75	76
        // 77	78	79	80	81	82	83	84	85	86	87
        // 88	89	90	91	92	93	94	95	96	97	98
        // 99	100	101	102	103	104	105	106	107	108	109
        // 110	111	112	113	114	115	116	117	118	119	120
        // Map Generation
        let tiles = (0..(width*height))
            .map( |i| {
                if  i < width                   ||
                    i > (width*height-width)    ||
                    (i % width) == 0            ||
                    (i % width) == 10
                    {
                    '#'
                }
                else {
                    '.'
                }
            })
            .collect();

        Map {
            width,
            height,
            tiles,
        }
    }

    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn get_tile(&self, idx:usize) -> char {
        self.tiles[idx]
    }

    fn center(&self) -> u32 {
        let width: u32 = if self.width % 2 == 0 {
            self.width+1
        } else {
            self.width };

        let height: u32 = if self.height % 2 == 0 {
            self.height+1
        } else {
           self.height };

        self.width * (height/2) - (width/2)
    }
}

// #[wasm_bindgen]
// pub struct Character {
struct Character {
    pos: u32,
    token: char,
}
impl Character {
    fn new() -> Character {
        let pos = 0;
        let token = '@';

        Character {
            pos,
            token,
        }
    }
    fn get_pos(&self) -> u32 {
        self.pos
    }
    fn set_pos(&mut self, pos:u32) {
        self.pos = pos;
    }
    fn get_token(&self) -> char {
        self.token
    }
}
// #[wasm_bindgen]
// impl Character {
//     pub fn increment_pos(&mut self, x:u32, y:u32) {
//         self.x = self.x + x;
//         self.y = self.y + y;
//     }
// }

#[wasm_bindgen]
pub struct GameState {
    tick_time: u32,
    map: Map,
    character: Character,
}
#[wasm_bindgen]
impl GameState {
    pub fn new() -> GameState {
        utils::set_panic_hook();

        let width = 11;
        let height = 11;

        let tick_time = 0;
        let map = Map::new(width, height);
        let mut character = Character::new();

        character.set_pos(map.center());

        GameState {
            tick_time,
            map,
            character,
        }
    }

    pub fn tick(&mut self) {
        self.tick_time += 1;
    }

    pub fn get_map_width(&self) -> u32 {
        self.map.width()
    }
    pub fn get_map_height(&self) -> u32 {
        self.map.height()
    }
    pub fn get_map_tile(&self, idx:usize) -> char {
        self.map.get_tile(idx)
    }
    pub fn get_char_tile(&self) -> char {
        self.character.get_token()
    }
    pub fn get_char_pos(&self) -> u32 {
        self.character.get_pos()
    }
}


    // pub fn width(&self) -> u32 {
    //     self.width
    // }

    // /// Set the width of the universe.
    // ///
    // /// Resets all cells to the dead state.
    // pub fn set_width(&mut self, width: u32) {
    //     self.width = width;
    //     self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    // }

    // pub fn height(&self) -> u32 {
    //     self.height
    // }

    // /// Set the height of the universe.
    // ///
    // /// Resets all cells to the dead state.
    // pub fn set_height(&mut self, height: u32) {
    //     self.height = height;
    //     self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    // }

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }
