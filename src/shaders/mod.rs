extern crate glium;
mod basic_shader;

pub struct ProgramManager
{
    prog_basic : glium::Program,
}

impl ProgramManager
{
    //Initialize the programs from their individual modules
    pub fn new(display: &glium::Display) -> ProgramManager
    {
        ProgramManager
        { 
            prog_basic : basic_shader::get_shader(display),
        }
    }

    pub fn get_program(self, sp: ShaderProgram) -> glium::Program
    {
        match sp
        {
            ShaderProgram::Basic => self.prog_basic,
            //_ => panic!(), //THERE IS NO SHADER HERE!!
        }
    }
}

pub enum ShaderProgram
{
    Basic,
}
