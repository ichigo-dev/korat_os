use crate::vga_buffer::color::ColorCode;

//------------------------------------------------------------------------------
//  char buffer
//------------------------------------------------------------------------------
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar
{
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer
{
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//------------------------------------------------------------------------------
//  writer
//------------------------------------------------------------------------------
pub struct Writer
{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer
{
    //--------------------------------------------------------------------------
    //  write byte
    //--------------------------------------------------------------------------
    pub fn write_byte( &mut self, byte: u8 )
    {
        match byte
        {
            b'\n' => self.new_line(),
            byte =>
            {
                if self.column_position >= BUFFER_WIDTH
                {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar
                {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    //--------------------------------------------------------------------------
    //  write string
    //--------------------------------------------------------------------------
    pub fn write_string( &mut self, s: &str )
    {
        for byte in s.bytes()
        {
            match byte
            {
                //  ASCII character
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                //  Non ASCII character
                _ => self.write_byte(0xfe),
            }
        }
    }

    //--------------------------------------------------------------------------
    //  new line
    //--------------------------------------------------------------------------
    fn new_line( &mut self )
    {
        unimplemented!();
    }
}
