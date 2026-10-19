use simple_color::Srgba;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorScheme {
    pub background: Srgba,
    pub accent_main: Srgba,
    pub accent_background: Srgba,
    pub accent_tint: Srgba,
    pub accent_shade: Srgba,
    pub tile_background: Srgba,
    pub hint_main: Srgba,
    pub hint_tint: Srgba,
    pub type_dark: Srgba,
    pub type_light: Srgba,
    pub grey_1: Srgba,
    pub grey_2: Srgba,
    pub swoosh_1: Srgba,
    pub swoosh_2: Srgba,
    pub swoosh_3: Srgba,
    pub swoosh_4: Srgba,
    pub swoosh_5: Srgba,
    pub swoosh_6: Srgba,
    pub swoosh_7: Srgba,
    pub swoosh_8: Srgba,
    pub swoosh_9: Srgba,
    pub swoosh_10: Srgba,
    pub swoosh_11: Srgba,
    pub swoosh_12: Srgba,
    pub swoosh_13: Srgba,
    pub swoosh_14: Srgba,
    pub swoosh_15: Srgba,
    pub swoosh_16: Srgba,
    pub swoosh_17: Srgba,
    pub swoosh_18: Srgba,
    pub swoosh_19: Srgba,

    pub firsts_logo_color: Srgba,
    pub type_grey: Srgba,
}

impl ColorScheme {
    pub fn wordline_color(self, index: usize) -> Srgba {
        match index % 19 {
            0 => self.swoosh_1,
            1 => self.swoosh_2,
            2 => self.swoosh_3,
            3 => self.swoosh_4,
            4 => self.swoosh_5,
            5 => self.swoosh_6,
            6 => self.swoosh_7,
            7 => self.swoosh_8,
            8 => self.swoosh_9,
            9 => self.swoosh_10,
            10 => self.swoosh_11,
            11 => self.swoosh_12,
            12 => self.swoosh_13,
            13 => self.swoosh_14,
            14 => self.swoosh_15,
            15 => self.swoosh_16,
            16 => self.swoosh_17,
            17 => self.swoosh_18,
            18 | _ => self.swoosh_19,
        }
    }
}

pub const COLOR_SCHEME1A: ColorScheme = ColorScheme {
    background: rgb_hex(0xF9F9F9),        // #F9F9F9
    accent_main: rgb_hex(0x5F6BFA),       // #5F6BFA
    accent_background: rgb_hex(0x5F6BFA), // #5F6BFA
    accent_tint: rgb_hex(0xDBDDFF),       // #DBDDFF
    accent_shade: rgb_hex(0x3735B8),      // #3735B8
    tile_background: rgb_hex(0xFBFBFB),   // #FBFBFB
    hint_main: rgb_hex(0xFF962D),         // #FF962D
    hint_tint: rgb_hex(0xFFD8B2),         // #FFD8B2
    type_dark: rgb_hex(0x003F42),         // #003F42
    type_light: rgb_hex(0xFFFFFF),        // #FFFFFF
    grey_1: rgb_hex(0xA0A0A0),            // #A0A0A0
    grey_2: rgb_hex(0xE2E2E2),            // #E2E2E2
    swoosh_1: rgb_hex(0x1A828D),          // #1A828D
    swoosh_2: rgb_hex(0x1079A9),          // #1079A9
    swoosh_3: rgb_hex(0x0F78C9),          // #0F78C9
    swoosh_4: rgb_hex(0x1A54C9),          // #1A54C9
    swoosh_5: rgb_hex(0x1A54C9),          // #1A54C9
    swoosh_6: rgb_hex(0x2041C8),          // #2041C8
    swoosh_7: rgb_hex(0x2331C8),          // #2331C8
    swoosh_8: rgb_hex(0x2E27C8),          // #2E27C8
    swoosh_9: rgb_hex(0x4027C8),          // #4027C8
    swoosh_10: rgb_hex(0x5427C8),         // #5427C8
    swoosh_11: rgb_hex(0x6927C8),         // #6927C8
    swoosh_12: rgb_hex(0x7D27C8),         // #7D27C8
    swoosh_13: rgb_hex(0x9328C8),         // #9328C8
    swoosh_14: rgb_hex(0x9328C8),         // #9328C8
    swoosh_15: rgb_hex(0xBE3BD3),         // #BE3BD3
    swoosh_16: rgb_hex(0xD33BBC),         // #D33BBC
    swoosh_17: rgb_hex(0xC534AD),         // #C534AD
    swoosh_18: rgb_hex(0xC71E45),         // #C71E45
    swoosh_19: rgb_hex(0xCC2C74),         // #CC2C74

    firsts_logo_color: rgb_hex(0xd6dde5), // #d6dde5
    type_grey: rgb_hex(0xAAB1B4),         // #AAB1B4
};

pub const COLOR_SCHEME2: ColorScheme = ColorScheme {
    background: rgb_hex(0xFFFEF9),        // #FFFEF9
    accent_main: rgb_hex(0xFD5032),       // #FD5032
    accent_background: rgb_hex(0xFD5032), // #FD5032
    accent_tint: rgb_hex(0xFFE6E2),       // #FFE6E2
    accent_shade: rgb_hex(0xA93A27),      // #A93A27
    tile_background: rgb_hex(0xFFFEFE),   // #FFFEFE
    hint_main: rgb_hex(0x5600C7),         // #5600C7
    hint_tint: rgb_hex(0xAA96FF),         // #AA96FF
    type_dark: rgb_hex(0x003F42),         // #003F42
    type_light: rgb_hex(0xFFFFFF),        // #FFFFFF
    grey_1: rgb_hex(0xA0A0A0),            // #A0A0A0
    grey_2: rgb_hex(0xE2E2E2),            // #E2E2E2
    swoosh_1: rgb_hex(0xFFA230),          // #FFA230
    swoosh_2: rgb_hex(0xDA9C1F),          // #DA9C1F
    swoosh_3: rgb_hex(0xF0B504),          // #F0B504
    swoosh_4: rgb_hex(0xD0C32F),          // #D0C32F
    swoosh_5: rgb_hex(0xA1BF35),          // #A1BF35
    swoosh_6: rgb_hex(0x8BA823),          // #8BA823
    swoosh_7: rgb_hex(0x16A554),          // #16A554
    swoosh_8: rgb_hex(0x1675A5),          // #1675A5
    swoosh_9: rgb_hex(0x065F8C),          // #065F8C
    swoosh_10: rgb_hex(0x0052B1),         // #0052B1
    swoosh_11: rgb_hex(0x0216A7),         // #0216A7
    swoosh_12: rgb_hex(0x1700C7),         // #1700C7
    swoosh_13: rgb_hex(0x5600C7),         // #5600C7
    swoosh_14: rgb_hex(0x0052B1),         // #0052B1
    swoosh_15: rgb_hex(0x5F6BFA),         // #5F6BFA
    swoosh_16: rgb_hex(0xD33BBC),         // #D33BBC
    swoosh_17: rgb_hex(0xC534AD),         // #C534AD
    swoosh_18: rgb_hex(0xC71E45),         // #C71E45
    swoosh_19: rgb_hex(0x2041C8),         // #2041C8

    firsts_logo_color: rgb_hex(0xd6dde5), // #d6dde5
    type_grey: rgb_hex(0xAAB1B4),         // #AAB1B4
};

pub const COLOR_SCHEME_DARK: ColorScheme = ColorScheme {
    background: rgb_hex(0x222328),        // #222328
    accent_main: rgb_hex(0x5F6BFA),       // #5F6BFA
    accent_background: rgb_hex(0x24243A), // #24243A
    accent_tint: rgb_hex(0x44545F),       // #44545F
    accent_shade: rgb_hex(0x5F6BFA),      // #5F6BFA
    tile_background: rgb_hex(0x222328),   // #222328
    hint_main: rgb_hex(0xFFC744),         // #FFC744
    hint_tint: rgb_hex(0xFFD8B2),         // #FFD8B2
    type_dark: rgb_hex(0xFFFFFF),         // #FFFFFF
    type_light: rgb_hex(0xFFFFFF),        // #FFFFFF
    grey_1: rgb_hex(0xA0A0A0),            // #A0A0A0
    grey_2: rgb_hex(0x4C505E),            // #4C505E
    swoosh_1: rgb_hex(0x1A828D),          // #1A828D
    swoosh_2: rgb_hex(0x1079A9),          // #1079A9
    swoosh_3: rgb_hex(0x0F78C9),          // #0F78C9
    swoosh_4: rgb_hex(0x1A54C9),          // #1A54C9
    swoosh_5: rgb_hex(0x1A54C9),          // #1A54C9
    swoosh_6: rgb_hex(0x2041C8),          // #2041C8
    swoosh_7: rgb_hex(0x2331C8),          // #2331C8
    swoosh_8: rgb_hex(0x2E27C8),          // #2E27C8
    swoosh_9: rgb_hex(0x4027C8),          // #4027C8
    swoosh_10: rgb_hex(0x5427C8),         // #5427C8
    swoosh_11: rgb_hex(0x6927C8),         // #6927C8
    swoosh_12: rgb_hex(0x7D27C8),         // #7D27C8
    swoosh_13: rgb_hex(0x9328C8),         // #9328C8
    swoosh_14: rgb_hex(0x9328C8),         // #9328C8
    swoosh_15: rgb_hex(0xBE3BD3),         // #BE3BD3
    swoosh_16: rgb_hex(0xD33BBC),         // #D33BBC
    swoosh_17: rgb_hex(0xC534AD),         // #C534AD
    swoosh_18: rgb_hex(0xC71E45),         // #C71E45
    swoosh_19: rgb_hex(0xCC2C74),         // #CC2C74

    firsts_logo_color: rgb_hex(0xd6dde5), // #d6dde5
    type_grey: rgb_hex(0xAAB1B4),         // #AAB1B4
};



pub const fn rgb_hex(number: u32) -> Srgba {
    Srgba::rgb_hex(number)
}

pub const fn rgba_hex(number: u32) -> Srgba{
    Srgba::rgba_hex(number)
}