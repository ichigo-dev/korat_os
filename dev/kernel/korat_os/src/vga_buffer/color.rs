/*

    Colors in VGA text mode

    ----------------------------------------------------------------------------

    The following colors can be used for characters displayed on the screen.

    | Number | Color        | Number | Color        |
    | ------ | ------------ | ------ | ------------ |
    | 0x0    | Black        | 0x8    | Dark Gray    |
    | 0x1    | Blue         | 0x9    | Light Blue   |
    | 0x2    | Green        | 0xa    | Light Green  |
    | 0x3    | Cyan         | 0xb    | Light Cyan   |
    | 0x4    | Red          | 0xc    | Light Red    |
    | 0x5    | Magenta      | 0xd    | Pink         |
    | 0x6    | Brown        | 0xe    | Yellow       |
    | 0x7    | Light Gray   | 0xf    | White        |

*/

//------------------------------------------------------------------------------
//  The standard color palette in VGA text mode
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color
{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

//------------------------------------------------------------------------------
//  A combination of a foreground and a background color
//------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct ColorCode(u8);

impl ColorCode
{
    //--------------------------------------------------------------------------
    //  Create a new `ColorCode` with the given foreground and background colors
    //--------------------------------------------------------------------------
    pub fn new( foreground: Color, background: Color ) -> ColorCode
    {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
