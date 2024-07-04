pub struct Packet
{
    pub x:i32,
    pub y:i32,
    pub m1:i32,
    pub m2:i32,
}

pub struct JoyStick
{
    pub left_x:f32,
    pub left_y:f32,
    pub right_x:f32,
    pub right_y:f32,
}

pub struct Dpad
{
    pub up_key:bool,
    pub down_key:bool,
    pub left_key:bool,
    pub right_key:bool,   
}

pub struct Buttons
{
    pub circle:bool,
    pub cross:bool,
    pub triangle:bool,
    pub cube:bool,
    pub r1:bool,
    pub r2:bool,
    pub l1:bool,
    pub l2:bool,
    pub left_push:bool,
    pub right_push:bool
}

pub struct DualShock4
{
    pub sticks:JoyStick,
    pub btns:Buttons,
    pub dpad:Dpad
}