mod color;

use crate::vga_buffer::color::{ Color, ColorCode };

use core::fmt;
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

//------------------------------------------------------------------------------
//  A global `Writer` instance can be used for printing to the VGA text buffer.
//------------------------------------------------------------------------------
lazy_static!
{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer
    {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

//------------------------------------------------------------------------------
//  A screen character in the VGA text buffer, consisting of an ASCII character
//  and a `ColorCode`.
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar
{
    ascii_character: u8,
    color_code: ColorCode,
}

//------------------------------------------------------------------------------
//  A structure representing the VGA text buffer.
//------------------------------------------------------------------------------
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer
{
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//------------------------------------------------------------------------------
//  A writer type that allows writing ASCII bytes and strings to an underlying
//  `Buffer`.
//
//  Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character.
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
    //  Writes an ASCII byte to the buffer.
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
                self.buffer.chars[row][col].write(ScreenChar
                {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    //--------------------------------------------------------------------------
    //  Write the given ASCII string to the buffer.
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
                _ => self.write_byte(0x3f),
            }
        }
    }

    //--------------------------------------------------------------------------
    //  Shifts all lines one line up and clears the last row.
    //--------------------------------------------------------------------------
    fn new_line( &mut self )
    {
        for row in 1..BUFFER_HEIGHT
        {
            for col in 0..BUFFER_WIDTH
            {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    //--------------------------------------------------------------------------
    //  Clears a row by overwriting it with blank characters.
    //--------------------------------------------------------------------------
    fn clear_row( &mut self, row: usize )
    {
        let blank = ScreenChar
        {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH
        {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer
{
    //--------------------------------------------------------------------------
    //  Supports formatting macro.
    //--------------------------------------------------------------------------
    fn write_str( &mut self, s: &str ) -> fmt::Result
    {
        self.write_string(s);
        Ok(())
    }
}

//------------------------------------------------------------------------------
//  A macro that print strings to VGA text buffer.
//------------------------------------------------------------------------------
#[macro_export]
macro_rules! print
{
    ( $($arg:tt)* ) => ( $crate::vga_buffer::_print(format_args!($($arg)*)) );
}

//------------------------------------------------------------------------------
//  A macro that print strings to VGA text buffer.
//------------------------------------------------------------------------------
#[macro_export]
macro_rules! println
{
    () => ( $crate::print!("\n") );
    ( $($arg:tt)* ) => ( $crate::print!("{}\n", format_args!($($arg)*)) );
}

//------------------------------------------------------------------------------
//  Prints the given formatted string to VGA text buffer through the global
//  `WRITER` instance.
//------------------------------------------------------------------------------
#[doc(hidden)]
pub fn _print( args: fmt::Arguments )
{
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
