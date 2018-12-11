use vertex::{TextureVertex, Square};

pub enum ElementType
{
    AIR,
    WATER,
    EARTH,
    FIRE,
    SALT,
    SULFUR,
    MERCURY,
    ASH,
}

pub struct ElementTypeList
{
    pub AIR : ElementTypeData,
    pub WATER : ElementTypeData,
    pub EARTH : ElementTypeData,
    pub FIRE : ElementTypeData,
    pub SALT : ElementTypeData,
    pub SULFUR : ElementTypeData,
    pub MERCURY : ElementTypeData,
    pub ASH : ElementTypeData,
}
impl ElementTypeList
{
    pub fn new() -> ElementTypeList
    {
        ElementTypeList
        {
            AIR : ElementTypeData
            {
                x_offset: 0.00,
                y_offset: 0.75,
                weight: 1,
                interacts: vec![ElementType::AIR],
                produces: Option::Some(ElementType::SALT),
            },
            FIRE : ElementTypeData
            {
                x_offset: 0.25,
                y_offset: 0.75,
                weight: 1,
                interacts: vec![ElementType::FIRE],
                produces: Option::Some(ElementType::SALT),
            },
            WATER : ElementTypeData
            {
                x_offset: 0.50,
                y_offset: 0.75,
                weight: 1,
                interacts: vec![ElementType::WATER],
                produces: Option::Some(ElementType::SALT),
            },
            EARTH : ElementTypeData
            {
                x_offset: 0.75,
                y_offset: 0.75,
                weight: 1,
                interacts: vec![ElementType::FIRE],
                produces: Option::Some(ElementType::SALT),
            },
            SALT : ElementTypeData
            {
                x_offset: 0.00,
                y_offset: 0.50,
                weight: 3,
                interacts: vec![ElementType::SALT],
                produces: Option::Some(ElementType::SULFUR),
            },
            SULFUR : ElementTypeData
            {
                x_offset: 0.25,
                y_offset: 0.50,
                weight: 3,
                interacts: vec![ElementType::SULFUR],
                produces: Option::Some(ElementType::MERCURY),
            },
            MERCURY : ElementTypeData
            {
                x_offset: 0.50,
                y_offset: 0.50,
                weight: 3,
                interacts: vec![ElementType::MERCURY],
                produces: Option::None,
            },
            ASH : ElementTypeData
            {
                x_offset: 0.75,
                y_offset: 0.50,
                weight: 3,
                interacts: vec![ElementType::ASH],
                produces: Option::Some(ElementType::ASH),
            },
        }
    }

    pub fn get_data(&self, t : &ElementType) -> &ElementTypeData
    {
        match *t
        {
            ElementType::AIR => &self.AIR,
            ElementType::FIRE => &self.FIRE,
            ElementType::WATER => &self.WATER,
            ElementType::EARTH => &self.EARTH,
            ElementType::SALT => &self.SALT,
            ElementType::SULFUR => &self.SULFUR,
            ElementType::MERCURY => &self.MERCURY,
            ElementType::ASH => &self.ASH,
        }
    }

    pub fn get_element(&self, unlocks : &Vec<ElementType>) -> ElementType
    {
        ElementType::ASH
    }
}

pub struct ElementTypeData
{
    x_offset : f32,
    y_offset : f32,
    weight : u32,
    interacts : Vec<ElementType>,
    produces : Option<ElementType>,
}

pub struct Element
{
    x : f32,
    y : f32,
    t : ElementType,
    fallspeed : f32,
}
impl Element
{
    pub fn new(x:f32, y:f32, t:ElementType) -> Element
    {
        Element
        {
            x : x,
            y : y,
            t : t,
            fallspeed : 0.0,
        }
    }

    pub fn get_vertices(&self, dataref : &ElementTypeList) -> Vec<TextureVertex>
    {
        let s = 2.0;
        let depth = 1.0;
        let dt = 0.25;

        let data = dataref.get_data(&self.t);
        let tx = data.x_offset;
        let ty = data.y_offset;

        let sq = Square {
            top_left:     TextureVertex { position: [ self.x,  self.y+s, depth], tex_coords: [ tx, ty+dt ] },
            top_right:    TextureVertex { position: [self.x+s, self.y+s, depth], tex_coords: [ tx+dt, ty+dt ] },
            bottom_left:  TextureVertex { position: [ self.x,   self.y,  depth], tex_coords: [ tx, ty ] },
            bottom_right: TextureVertex { position: [self.x+s,  self.y,  depth], tex_coords: [ tx+dt, ty ] },
        };
        sq.get_vec()
    }

    pub fn move_to(&mut self, target_x:f32, target_y:f32, dt:f32, top:bool) -> bool
    {
        let mindistance = 0.01;
        let gravity = 50.0; //Adjustable
        let init_speed = 100.0;
        //Euclidian distance
        let dist = ((self.x - target_x).powi(2) + (self.y - target_y).powi(2)).sqrt();
        if dist < mindistance
        {
            self.x = target_x;
            self.y = target_y;
            self.fallspeed = 0.0;
            true
        }
        else
        {
            let speed_mult = 20.0;
            let clamped_dt = (1.0 as f32).min(dt * speed_mult);
            self.x += (target_x - self.x) * clamped_dt;
            if top
            {
                self.fallspeed = 0.0;
                self.y += (target_y - self.y) * clamped_dt;
            }
            else
            {
                if self.fallspeed > 1.0 { self.fallspeed = -init_speed; }
                self.fallspeed -= gravity * dt;
                //Dealing with negatives here, pick maximum instead
                self.y += (target_y - self.y).max(self.fallspeed * dt);
            }
            false
        }
    }
    pub fn set_pos(&mut self, new_x:f32, new_y:f32)
    {
        self.x = new_x;
        self.y = new_y;
    }
}