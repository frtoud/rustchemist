use vertex::{TextureVertex, Square};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ElementType
{
    AIR,
    WATER,
    EARTH,
    FIRE,
    SALT,
    SULFUR,
    MERCURY,
    LEAD,
    TIN,
    IRON,
    COPPER,
    SILVER,
    GOLD,
    
    ASH,
    ANTIMONY,
    AETHER,
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
    pub LEAD : ElementTypeData,
    pub TIN : ElementTypeData,
    pub IRON : ElementTypeData,
    pub COPPER : ElementTypeData,
    pub SILVER : ElementTypeData,
    pub GOLD : ElementTypeData,

    pub ASH : ElementTypeData,
    pub ANTIMONY : ElementTypeData,
    pub AETHER : ElementTypeData,
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
                value: 1,
                weight: 1,
                interacts: vec![ElementType::AIR],
                produces: Option::Some(ElementType::SALT),
            },
            FIRE : ElementTypeData
            {
                x_offset: 0.25,
                y_offset: 0.75,
                value: 1,
                weight: 1,
                interacts: vec![ElementType::FIRE],
                produces: Option::Some(ElementType::SALT),
            },
            WATER : ElementTypeData
            {
                x_offset: 0.50,
                y_offset: 0.75,
                value: 1,
                weight: 1,
                interacts: vec![ElementType::WATER],
                produces: Option::Some(ElementType::SALT),
            },
            EARTH : ElementTypeData
            {
                x_offset: 0.75,
                y_offset: 0.75,
                value: 1,
                weight: 1,
                interacts: vec![ElementType::FIRE],
                produces: Option::Some(ElementType::SALT),
            },
            SALT : ElementTypeData
            {
                x_offset: 0.00,
                y_offset: 0.50,
                value: 3,
                weight: 3,
                interacts: vec![ElementType::SALT],
                produces: Option::Some(ElementType::SULFUR),
            },
            SULFUR : ElementTypeData
            {
                x_offset: 0.25,
                y_offset: 0.50,
                value: 9,
                weight: 3,
                interacts: vec![ElementType::SULFUR],
                produces: Option::Some(ElementType::MERCURY),
            },
            MERCURY : ElementTypeData
            {
                x_offset: 0.50,
                y_offset: 0.50,
                value: 27,
                weight: 3,
                interacts: vec![ElementType::MERCURY],
                produces: Option::Some(ElementType::LEAD),
            },
            LEAD : ElementTypeData
            {
                x_offset: 0.00,
                y_offset: 0.25,
                value: 81,
                weight: 3,
                interacts: vec![ElementType::LEAD],
                produces: Option::Some(ElementType::TIN),
            },
            TIN : ElementTypeData
            {
                x_offset: 0.25,
                y_offset: 0.25,
                value: 243,
                weight: 3,
                interacts: vec![ElementType::TIN],
                produces: Option::Some(ElementType::IRON),
            },
            IRON : ElementTypeData
            {
                x_offset: 0.50,
                y_offset: 0.25,
                value: 729,
                weight: 3,
                interacts: vec![ElementType::IRON],
                produces: Option::Some(ElementType::COPPER),
            },
            COPPER : ElementTypeData
            {
                x_offset: 0.00,
                y_offset: 0.00,
                value: 2187,
                weight: 3,
                interacts: vec![ElementType::COPPER],
                produces: Option::Some(ElementType::SILVER),
            },
            SILVER : ElementTypeData
            {
                x_offset: 0.25,
                y_offset: 0.00,
                value: 6561,
                weight: 3,
                interacts: vec![ElementType::SILVER],
                produces: Option::Some(ElementType::GOLD),
            },
            GOLD : ElementTypeData
            {
                x_offset: 0.50,
                y_offset: 0.00,
                value: 19683,
                weight: 3,
                interacts: vec![],
                produces: Option::None,
            },
            ASH : ElementTypeData
            {
                x_offset: 0.75,
                y_offset: 0.50,
                value: 0,
                weight: 3,
                interacts: vec![ElementType::ASH],
                produces: Option::Some(ElementType::ASH),
            },
            ANTIMONY : ElementTypeData
            {
                x_offset: 0.75,
                y_offset: 0.25,
                value: 0,
                weight: 3,
                //All metals except Gold
                interacts: vec![ElementType::MERCURY, ElementType::LEAD, ElementType::TIN, ElementType::IRON, ElementType::COPPER, ElementType::SILVER],
                produces: Option::None,
            },
            AETHER : ElementTypeData
            {
                x_offset: 0.75,
                y_offset: 0.00,
                value: 0,
                weight: 3,
                interacts: vec![ElementType::AIR, ElementType::EARTH, ElementType::FIRE, ElementType::WATER],
                produces: Option::None,
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
            ElementType::LEAD => &self.LEAD,
            ElementType::TIN => &self.TIN,
            ElementType::IRON => &self.IRON,
            ElementType::COPPER => &self.COPPER,
            ElementType::SILVER => &self.SILVER,
            ElementType::GOLD => &self.GOLD,
            ElementType::ASH => &self.ASH,
            ElementType::AETHER => &self.AETHER,
            ElementType::ANTIMONY => &self.ANTIMONY,
        }
    }

    pub fn get_element(&self, unlocks : &Vec<ElementType>) -> ElementType
    {
        ElementType::AIR
    }

    pub fn can_react(&self, e1:&ElementType, e2:&ElementType) -> bool
    {
        self.get_data(e1).interacts.contains(e2) || self.get_data(e2).interacts.contains(e1)
    }
    pub fn get_product(&self, reagents : &Vec<ElementType>) -> Option<ElementType>
    {
        let mut product = None;
        for i in 0..reagents.len()
        {
            let p = self.get_data(&reagents[i]).produces;
            if p.is_some() && (product.is_none() || self.get_data(product.as_ref().unwrap()).value < self.get_data(p.as_ref().unwrap()).value)
            {
                product = p;
            }
        }
        product
    }
}

pub struct ElementTypeData
{
    x_offset : f32,
    y_offset : f32,
    value : u32,
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
    pub fn get_type(&self) -> &ElementType
    {
        &self.t
    }
}