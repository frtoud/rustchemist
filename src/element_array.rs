extern crate glium;

use grid::GridSize;
use element::{Element, ElementType, ElementTypeList};
use glium::VertexBuffer;
use glium::texture::Texture2d;
use vertex::TextureVertex;
use loader;
use std::mem;

pub struct Tile
{
    x : f32,
    y : f32,
    occupant : Option<Element>,
}
#[derive(Copy, Clone, Debug)]
pub struct Coord
{
    x : i32,
    y : i32,
}
impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool 
    {
        self.x == other.x && self.y == other.y
    }
}

pub enum GuideRotation
{
    // Position of pair_1, relative to pair_2
    LEFT,
    UP,
    RIGHT,
    DOWN,
}

pub struct ElementArray
{
    grid_size : GridSize,
    width : i32,
    height : i32,
    array : Vec<Tile>,

    guide_pos : i32,
    guide_rot : GuideRotation,
    //current pair in control
    pub pair_1 : Tile,
    pub pair_2 : Tile,
    //next pair
    next_1 : Tile,
    next_2 : Tile,

    unlocks : Vec<ElementType>,
    pub element_data : ElementTypeList,
    pub texture : Texture2d,

    effect_time : f32,
}

impl ElementArray
{
    fn reset_array(w: i32, h: i32) -> Vec<Tile>
    {
        let c = -3.0f32; //Center offset
        let zero = c - (w as f32); //[0, 0] offset
        let mut tilevec = Vec::with_capacity((w * h) as usize);
        for px in 0..w
        {
            for py in 0..h
            {
                //Initialize all tiles to empty
                // Px * H + Py = Index
                // Position 0,0 is bottom-left
                tilevec.push(Tile { x : zero + (2*px) as f32, y : zero + (2*py) as f32, occupant : None });
            }
        }
        tilevec
    }
    fn reset_pairs(w: i32, h: i32) -> (Tile, Tile)
    {
        let c = -3.0f32; //Center offset
        let zero = c - (w as f32); //[0, 0] offset
        (Tile { x : zero + (2*(w / 2 - 1)) as f32, y : zero + (2*(h - 1)) as f32, occupant : None },
        Tile { x : zero + (2*(w / 2)) as f32, y : zero + (2*(h - 1)) as f32, occupant : None })
    }
    fn reset_next() -> (Tile, Tile)
    {
        let c = 8.0f32; //Center of next window, X offset. +1 for Y offset
        (Tile { x : c - 1.0, y : c + 1.0, occupant : None }, //Left tile
        Tile { x : c + 1.0, y : c + 1.0, occupant : None }) //Right tile
    }
    pub fn set_next_position(&mut self, top : bool)
    {
        let c = 8.0f32;
        self.next_1.x = c - 1.0;
        self.next_2.x = c + 1.0;
        let y_offset = if top { 5.0 } else { 1.0 };
        self.next_1.y = c + y_offset;
        self.next_2.y = c + y_offset;
    }

    pub fn new(disp : &glium::Display, size: &GridSize) -> ElementArray
    {
        let w = *size as i32;
        //Two extra spots to hold excess elements, in case they react or cause game overs
        let h = w + 2;

        let tilevec = ElementArray::reset_array(w, h);
        let (p1, p2) = ElementArray::reset_pairs(w, h);
        let (n1, n2) = ElementArray::reset_next();
        ElementArray
        {
            grid_size : *size,
            width : w,
            height : h,
            array : tilevec,

            guide_pos : w / 2,
            guide_rot : GuideRotation::LEFT,
            pair_1 : p1,
            pair_2 : p2,
            next_1 : n1,
            next_2 : n2,
            
            texture : loader::get_sprite(disp, "Elements.png"),

            unlocks : vec![],
            element_data : ElementTypeList::new(),

            effect_time : 0.0,
        }
    }

    pub fn reset(&mut self, size: &GridSize)
    {
        //Reset this
        self.effect_time = 0.0;

        self.grid_size = *size;
        self.width = *size as i32;
        self.height = self.width + 2;
        
        self.array = ElementArray::reset_array(self.width, self.height);
        let (p1, p2) = ElementArray::reset_pairs(self.width, self.height);
        self.pair_1 = p1;
        self.pair_2 = p2;
        let (n1, n2) = ElementArray::reset_next();
        self.next_1 = n1;
        self.next_2 = n2;
        self.guide_pos = (self.width / 2) - 1;
        self.guide_rot = GuideRotation::LEFT;

        self.unlocks = vec![];
        //self.unlocks.push(ElementType::ASH);
        self.unlocks.push(ElementType::AIR);
        self.unlocks.push(ElementType::WATER);
        self.unlocks.push(ElementType::EARTH);
        self.unlocks.push(ElementType::FIRE);
    }

    //Shortcut to test array positions with X and Y
    pub fn array_at_mut(&mut self, x: i32, y:i32) -> Option<&mut Tile>
    {
        if !(x < self.width) || !(y < self.height) || x < 0 || y < 0
        {
            return None
        }
        let index = (x * self.height + y) as usize;
        if index < self.array.len()
        {
            Some(&mut self.array[index])
        }
        else
        {
            None
        }
    }
    pub fn array_at(&self, x: i32, y:i32) -> Option<&Tile>
    {
        if !(x < self.width) || !(y < self.height) || x < 0 || y < 0
        {
            return None
        }
        let index = (x * self.height + y) as usize;
        if index < self.array.len()
        {
            Some(&self.array[index])
        }
        else
        {
            None
        }
    }

    //Get the VertexBuffer for drawing the elements within
    pub fn get_buffer(&self, disp : &glium::Display) -> VertexBuffer<TextureVertex>
    {
        let mut texvec = vec![];
        for tile in &self.array
        {
            if tile.occupant.is_some()
            {
                texvec.extend(tile.occupant.as_ref().unwrap().get_vertices(&self.element_data));
            }
        }

        if self.pair_1.occupant.is_some()
        {
            texvec.extend(self.pair_1.occupant.as_ref().unwrap().get_vertices(&self.element_data));
        }
        if self.pair_2.occupant.is_some()
        {
            texvec.extend(self.pair_2.occupant.as_ref().unwrap().get_vertices(&self.element_data));
        }
        VertexBuffer::new(disp, &texvec).unwrap()
    }
    //Get the VertexBuffer for the next pairs (needed because of second scissor)
    pub fn get_next_buffer(&self, disp : &glium::Display) -> VertexBuffer<TextureVertex>
    {
        let mut texvec = vec![];
        if self.next_1.occupant.is_some()
        {
            texvec.extend(self.next_1.occupant.as_ref().unwrap().get_vertices(&self.element_data));
        }
        if self.next_2.occupant.is_some()
        {
            texvec.extend(self.next_2.occupant.as_ref().unwrap().get_vertices(&self.element_data));
        }
        VertexBuffer::new(disp, &texvec).unwrap()
    }

    pub fn move_elements(&mut self, dt : f32) -> bool
    {
        let mut done = true;
        for i in 0..self.array.len()
        {
            if self.array[i].occupant.is_some()
            {
                let x = self.array[i].x;
                let y = self.array[i].y;
                let reached = self.array[i].occupant.as_mut().unwrap().move_to(x, y, dt, false);
                done = done && reached;
            }
        }
        if self.pair_1.occupant.is_some()
        {
            let reached = self.pair_1.occupant.as_mut().unwrap().move_to(self.pair_1.x, self.pair_1.y, dt, true);
            done = done && reached;
        }
        if self.pair_2.occupant.is_some()
        {
            let reached = self.pair_2.occupant.as_mut().unwrap().move_to(self.pair_2.x, self.pair_2.y, dt, true);
            done = done && reached;
        }
        if self.next_1.occupant.is_some()
        {
            let reached = self.next_1.occupant.as_mut().unwrap().move_to(self.next_1.x, self.next_1.y, dt, true);
            done = done && reached;
        }
        if self.next_2.occupant.is_some()
        {
            let reached = self.next_2.occupant.as_mut().unwrap().move_to(self.next_2.x, self.next_2.y, dt, true);
            done = done && reached;
        }
        done
    }

    //INPUTS
    pub fn drop_pair(&mut self)
    {
        let x1 = self.guide_pos;
        let x2 = self.guide_pos +
        match self.guide_rot //Find offset for second element
        {
            GuideRotation::LEFT => 1,
            GuideRotation::RIGHT => -1,
            _ => 0, //Top, Down: Same target!
        };
        let (y1, y2) = match self.guide_rot
        {
            GuideRotation::UP => (self.height - 1, self.height - 2),
            GuideRotation::DOWN => (self.height - 2, self.height - 1),
            _ => (self.height - 1, self.height - 1), //Left, Right: Same altitude, because of different targets!
        };
        mem::swap(&mut self.array[(x1 * self.height + y1) as usize].occupant, &mut self.pair_1.occupant);
        mem::swap(&mut self.array[(x2 * self.height + y2) as usize].occupant, &mut self.pair_2.occupant);
        //Should never put elements back into pair, cause top layers should be cleared when in play.
        //Just in case, tho...
        self.pair_1.occupant = None;
        self.pair_2.occupant = None;
        //Set back to middle positions
        self.guide_pos = (self.width / 2) - 1;
        self.guide_rot = GuideRotation::LEFT;
        self.set_pair_positions();
    }
    pub fn rotate_pair(&mut self)
    {
        self.guide_rot = match self.guide_rot
        {
            GuideRotation::LEFT =>
            {
                GuideRotation::UP
            },
            GuideRotation::UP =>
            {
                self.guide_pos += 1;
                //Dont get outside the bounds
                if self.guide_pos >= self.width 
                { self.guide_pos = self.width - 1; }
                GuideRotation::RIGHT
            },
            GuideRotation::RIGHT =>
            {
                self.guide_pos -= 1;
                GuideRotation::DOWN
            },
            GuideRotation::DOWN =>
            {
                //Dont get outside the bounds
                if self.guide_pos >= self.width - 1
                { self.guide_pos = self.width - 2; }
                GuideRotation::LEFT
            },
        };
        self.set_pair_positions();
    }
    pub fn move_pair(&mut self, dx : i32)
    {
        let (min, max) = match self.guide_rot
        {
            GuideRotation::RIGHT => ( 1, self.width - 1 ),
            GuideRotation::LEFT => ( 0, self.width - 2 ),
            _ => ( 0, self.width - 1 ),
        };
        self.guide_pos += dx;
        if self.guide_pos < min { self.guide_pos = min; }
        if self.guide_pos > max { self.guide_pos = max; }
        self.set_pair_positions();
    }
    pub fn set_pair_positions(&mut self)
    {
        let c = -3.0f32; //Center offset
        let zero = c - (self.width as f32); //[0, 0] offset

        match self.guide_rot
        {
            GuideRotation::UP =>
            {
                self.pair_1.x = zero + (self.guide_pos as f32 * 2.0);
                self.pair_1.y = zero + (((self.height - 1) as f32) * 2.0 + 1.0);
                self.pair_2.x = zero + (self.guide_pos as f32 * 2.0);
                self.pair_2.y = zero + (((self.height - 1) as f32) * 2.0 - 1.0);
            },
            GuideRotation::RIGHT =>
            {
                self.pair_1.x = zero + (self.guide_pos as f32 * 2.0);
                self.pair_1.y = zero + (((self.height - 1) as f32) * 2.0);
                self.pair_2.x = zero + ((self.guide_pos - 1) as f32 * 2.0);
                self.pair_2.y = zero + (((self.height - 1) as f32) * 2.0);
            },
            GuideRotation::DOWN =>
            {
                self.pair_1.x = zero + (self.guide_pos as f32 * 2.0);
                self.pair_1.y = zero + (((self.height - 1) as f32) * 2.0 - 1.0);
                self.pair_2.x = zero + (self.guide_pos as f32 * 2.0);
                self.pair_2.y = zero + (((self.height - 1) as f32) * 2.0 + 1.0);
            },
            GuideRotation::LEFT =>
            {
                self.pair_1.x = zero + ((self.guide_pos as f32) * 2.0);
                self.pair_1.y = zero + (((self.height - 1) as f32) * 2.0);
                self.pair_2.x = zero + ((self.guide_pos + 1) as f32 * 2.0);
                self.pair_2.y = zero + (((self.height - 1) as f32) * 2.0);
            },
        };
    }

    //FALLING
    pub fn make_fall(&mut self)
    {
        for px in 0..self.width
        {
            for py in 0..self.height
            {
                // Px * H + Py = Index
                // Position 0,0 is bottom-left
                if self.array_at_mut(px,py).unwrap().occupant.is_none()
                {
                    //Missing element; look above to fill
                    'look: for py2 in (py+1)..self.height
                    {
                        if self.array_at_mut(px,py2).unwrap().occupant.is_some()
                        {
                            //Gottem
                            let (a1, a2) = self.array.split_at_mut((px * self.height + py2) as usize);
                            mem::swap(&mut a1[(px * self.height + py) as usize].occupant,
                                      &mut a2[(0) as usize].occupant);
                            break 'look;
                        }
                    }
                }
            }
        }
    }

    //CHECKING
    pub fn test_above(&self) -> bool
    {
        for px in 0..self.width
        {
            for py in self.height - 2..self.height
            {
                if self.array_at(px,py).unwrap().occupant.is_some()
                {
                    return false
                }
            }
        }
        //Did not find anything, grid is still valid for play
        return true
    }

    //PLAYING
    pub fn get_next_pair(&mut self, from_next : bool)
    {

        if from_next // we steal the ones ready in the next pair
        {
            mem::swap(&mut self.pair_1.occupant, &mut self.next_1.occupant);
            mem::swap(&mut self.pair_2.occupant, &mut self.next_2.occupant);
        }
        else // we make new ones, we just started a game
        {
            let t_1 = self.element_data.get_element(&self.unlocks);
            self.pair_1.occupant = Some(Element::new(0.0, 0.0, t_1));
            let t_2 = self.element_data.get_element(&self.unlocks);
            self.pair_2.occupant = Some(Element::new(0.0, 0.0, t_2));
        }
        //move into position
        if self.pair_1.occupant.is_some()
        {
            let xp1 = self.pair_1.x;
            let yp1 = self.pair_1.y + 5.0;
            self.pair_1.occupant.as_mut().unwrap().set_pos(xp1, yp1);
        }
        if self.pair_2.occupant.is_some()
        {
            let xp2 = self.pair_2.x;
            let yp2 = self.pair_2.y + 5.0;
            self.pair_2.occupant.as_mut().unwrap().set_pos(xp2, yp2);
        }
        //Spawn new elements
        let t1 = self.element_data.get_element(&self.unlocks);
        let x1 = self.next_1.x;
        let y1 = self.next_1.y - 4.0;
        self.next_1.occupant = Some(Element::new(x1, y1, t1));

        let t2 = self.element_data.get_element(&self.unlocks);
        let x2 = self.next_2.x;
        let y2 = self.next_2.y - 4.0;
        self.next_2.occupant = Some(Element::new(x2, y2, t2));
    }

    //REACTING
    pub fn test_reactions(&mut self) -> bool
    {
        let mut reacts = false;
        let mut products : Vec<(Coord, ElementType)> = vec![];
        for py in 0..self.height
        {
            for px in 0..self.width
            {
                if self.array_at(px,py).unwrap().occupant.is_some()
                {
                    //The initial type 
                    let t = *self.array_at(px, py).unwrap().occupant.as_ref().unwrap().get_type();
                    let mut reagents = vec![t];
                    let mut to_react = vec![Coord{x:px, y:py}];
                    let mut to_test = vec![Coord{x:px, y:py}];
                    while to_test.len() > 0
                    {
                        //Test neighbors to see if they can contribute to the reaction
                        let coord = to_test.pop().unwrap();
                        let tx = coord.x;
                        let ty = coord.y;
                        if self.array_at(tx,ty).unwrap().occupant.is_some()
                        {
                            //Our current type
                            let t1 = self.array_at(tx,ty).unwrap().occupant.as_ref().unwrap().get_type();
                            //Left
                            self.neighbor_reaction_test(Coord{x:tx-1, y:ty}, t1, &mut to_react, &mut to_test, &mut reagents);
                            //Right
                            self.neighbor_reaction_test(Coord{x:tx+1, y:ty}, t1, &mut to_react, &mut to_test, &mut reagents);
                            //Up
                            self.neighbor_reaction_test(Coord{x:tx, y:ty+1}, t1, &mut to_react, &mut to_test, &mut reagents);
                            //Down
                            self.neighbor_reaction_test(Coord{x:tx, y:ty-1}, t1, &mut to_react, &mut to_test, &mut reagents);
                        }
                    }
                    if to_react.len() >= 3
                    {
                        let prod = self.element_data.get_product(&reagents);
                        if prod.is_some()
                        {
                            reacts = true;
                            while to_react.len() > 0
                            {
                                let coord = to_react.pop().unwrap();
                                self.array_at_mut(coord.x, coord.y).unwrap().occupant = None;
                            }
                            products.push((Coord{x:px, y:py}, prod.unwrap()));
                        }
                    }
                }
            }
        }
        while products.len() > 0
        {
            let (coord, prod) = products.pop().unwrap();
            if !self.unlocks.contains(&prod) {self.unlocks.push(prod)}
            let tile = self.array_at_mut(coord.x, coord.y).unwrap();
            tile.occupant = Some(Element::new(tile.x, tile.y, prod));
        }
        return reacts
    }
    fn neighbor_reaction_test(&self, pos:Coord, t:&ElementType, to_react:&mut Vec<Coord>, to_test:&mut Vec<Coord>, reagents:&mut Vec<ElementType>)
    {
        if !to_react.contains(&pos)
        && self.array_at(pos.x,pos.y).is_some() 
        && self.array_at(pos.x,pos.y).unwrap().occupant.is_some()
        {
            let t2 = self.array_at(pos.x,pos.y).unwrap().occupant.as_ref().unwrap().get_type();
            if self.element_data.can_react(t, t2)
            {
                to_react.push(pos);
                to_test.push(pos);
                reagents.push(*t2);
            }
        }
    }
}