use simple_color::Srgba;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorScheme {
    pub background: Srgba,
    pub accent_main: Srgba,
    pub accent_background: Srgba,
    pub accent_opposite: Srgba,

    pub accent_tint: Srgba,
    pub accent_shade: Srgba,
    pub tile_background: Srgba,
    pub hint_main: Srgba,
    pub hint_tint: Srgba,

    pub firsts_tile: Srgba,

    pub hint_opposite: Srgba,

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

    pub hint_shade: Srgba,
    pub type_secondary: Srgba,
    pub highlight: Srgba,
    pub grey_3: Srgba,
    pub divider: Srgba,
    pub border: Srgba,
    pub scrim: Srgba,
    pub icon_muted: Srgba,
    pub on_accent: Srgba,
    pub on_accent_faint: Srgba,
    pub accent_wash: Srgba,

    pub logo: Srgba,
    pub logo_hero: Srgba,
    pub surface_raised: Srgba,

    pub firsts_letter: Srgba,
    pub answer_tile: Srgba,
    pub panel: Srgba,
    pub on_panel: Srgba,
    
    pub panel_on_tint: Srgba,
    pub panel_button_on_tint: Srgba,    
    pub panel_button: Srgba,
    pub hint_on_fill: Srgba,
    pub badge: Srgba,
    pub badge_icon: Srgba,
    pub on_inverse: Srgba,
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

// //Purple light
// pub const COLOR_SCHEME1A: ColorScheme = ColorScheme {
//     background: rgb_hex(0xF9F9F9),        // #F9F9F9
//     accent_main: rgb_hex(0x5F6BFA),       // #5F6BFA
//     accent_background: rgb_hex(0x5F6BFA), // #5F6BFA
//     accent_tint: rgb_hex(0xDBDDFF),       // #DBDDFF
//     accent_shade: rgb_hex(0x3735B8),      // #3735B8
//     tile_background: rgb_hex(0xFBFBFB),   // #FBFBFB
//     hint_main: rgb_hex(0xFF962D),         // #FF962D
//     hint_tint: rgb_hex(0xFFD8B2),         // #FFD8B2
//     type_dark: rgb_hex(0x003F42),         // #003F42
//     type_light: rgb_hex(0xFFFFFF),        // #FFFFFF
//     grey_1: rgb_hex(0xA0A0A0),            // #A0A0A0
//     grey_2: rgb_hex(0xE2E2E2),            // #E2E2E2
//     swoosh_1: rgb_hex(0x1A828D),          // #1A828D
//     swoosh_2: rgb_hex(0x1079A9),          // #1079A9
//     swoosh_3: rgb_hex(0x0F78C9),          // #0F78C9
//     swoosh_4: rgb_hex(0x1A54C9),          // #1A54C9
//     swoosh_5: rgb_hex(0x1A54C9),          // #1A54C9
//     swoosh_6: rgb_hex(0x2041C8),          // #2041C8
//     swoosh_7: rgb_hex(0x2331C8),          // #2331C8
//     swoosh_8: rgb_hex(0x2E27C8),          // #2E27C8
//     swoosh_9: rgb_hex(0x4027C8),          // #4027C8
//     swoosh_10: rgb_hex(0x5427C8),         // #5427C8
//     swoosh_11: rgb_hex(0x6927C8),         // #6927C8
//     swoosh_12: rgb_hex(0x7D27C8),         // #7D27C8
//     swoosh_13: rgb_hex(0x9328C8),         // #9328C8
//     swoosh_14: rgb_hex(0x9328C8),         // #9328C8
//     swoosh_15: rgb_hex(0xBE3BD3),         // #BE3BD3
//     swoosh_16: rgb_hex(0xD33BBC),         // #D33BBC
//     swoosh_17: rgb_hex(0xC534AD),         // #C534AD
//     swoosh_18: rgb_hex(0xC71E45),         // #C71E45
//     swoosh_19: rgb_hex(0xCC2C74),         // #CC2C74

//     firsts_logo_color: rgb_hex(0xd6dde5), // #d6dde5
//     type_grey: rgb_hex(0xAAB1B4),         // #AAB1B4
// };

// ///Orange light
// pub const COLOR_SCHEME2: ColorScheme = ColorScheme {
//     background: rgb_hex(0xFFFEF9),        // #FFFEF9
//     accent_main: rgb_hex(0xFD5032),       // #FD5032
//     accent_background: rgb_hex(0xFD5032), // #FD5032
//     accent_tint: rgb_hex(0xFFE6E2),       // #FFE6E2
//     accent_shade: rgb_hex(0xA93A27),      // #A93A27
//     tile_background: rgb_hex(0xFFFEFE),   // #FFFEFE
//     hint_main: rgb_hex(0x5600C7),         // #5600C7
//     hint_tint: rgb_hex(0xAA96FF),         // #AA96FF
//     type_dark: rgb_hex(0x003F42),         // #003F42
//     type_light: rgb_hex(0xFFFFFF),        // #FFFFFF
//     grey_1: rgb_hex(0xA0A0A0),            // #A0A0A0
//     grey_2: rgb_hex(0xE2E2E2),            // #E2E2E2
//     swoosh_1: rgb_hex(0xFFA230),          // #FFA230
//     swoosh_2: rgb_hex(0xDA9C1F),          // #DA9C1F
//     swoosh_3: rgb_hex(0xF0B504),          // #F0B504
//     swoosh_4: rgb_hex(0xD0C32F),          // #D0C32F
//     swoosh_5: rgb_hex(0xA1BF35),          // #A1BF35
//     swoosh_6: rgb_hex(0x8BA823),          // #8BA823
//     swoosh_7: rgb_hex(0x16A554),          // #16A554
//     swoosh_8: rgb_hex(0x1675A5),          // #1675A5
//     swoosh_9: rgb_hex(0x065F8C),          // #065F8C
//     swoosh_10: rgb_hex(0x0052B1),         // #0052B1
//     swoosh_11: rgb_hex(0x0216A7),         // #0216A7
//     swoosh_12: rgb_hex(0x1700C7),         // #1700C7
//     swoosh_13: rgb_hex(0x5600C7),         // #5600C7
//     swoosh_14: rgb_hex(0x0052B1),         // #0052B1
//     swoosh_15: rgb_hex(0x5F6BFA),         // #5F6BFA
//     swoosh_16: rgb_hex(0xD33BBC),         // #D33BBC
//     swoosh_17: rgb_hex(0xC534AD),         // #C534AD
//     swoosh_18: rgb_hex(0xC71E45),         // #C71E45
//     swoosh_19: rgb_hex(0x2041C8),         // #2041C8

//     firsts_logo_color: rgb_hex(0xd6dde5), // #d6dde5
//     type_grey: rgb_hex(0xAAB1B4),         // #AAB1B4
// };

pub const COLOR_SCHEME_LETTUCE: ColorScheme = ColorScheme {
    background: rgb_hex(0xFDFCF9).with_alpha(1.000), //#FDFCF9
accent_main: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
accent_background: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
accent_opposite: rgb_hex(0xFFFFFF).with_alpha(1.000), //#FFFFFF
accent_tint: rgb_hex(0xDDF5E7).with_alpha(1.000), //#DDF5E7
accent_shade: rgb_hex(0x1E8A52).with_alpha(1.000), //#1E8A52
tile_background: rgb_hex(0xFFFFFF).with_alpha(1.000), //#FFFFFF
hint_main: rgb_hex(0x4B67F4).with_alpha(1.000), //#4B67F4
hint_tint: rgb_hex(0xE7E9FF).with_alpha(1.000), //#E7E9FF
hint_opposite: rgb_hex(0xFFFFFF).with_alpha(1.000), //#FFFFFF
type_dark: rgb_hex(0x2F2C3E).with_alpha(1.000), //#2F2C3E
type_light: rgb_hex(0xFFFFFF).with_alpha(1.000), //#FFFFFF
grey_1: rgb_hex(0x8F8D9B).with_alpha(1.000), //#8F8D9B
grey_2: rgb_hex(0xDEDEDE).with_alpha(1.000), //#DEDEDE
swoosh_1: rgb_hex(0x5BC150).with_alpha(1.000), //#5BC150
swoosh_2: rgb_hex(0x5DC8AF).with_alpha(1.000), //#5DC8AF
swoosh_3: rgb_hex(0x57BFE9).with_alpha(1.000), //#57BFE9
swoosh_4: rgb_hex(0x3D8AF7).with_alpha(1.000), //#3D8AF7
swoosh_5: rgb_hex(0x1D46F6).with_alpha(1.000), //#1D46F6
swoosh_6: rgb_hex(0x1D15F3).with_alpha(1.000), //#1D15F3
swoosh_7: rgb_hex(0x4C0BF1).with_alpha(1.000), //#4C0BF1
swoosh_8: rgb_hex(0x8D1BF5).with_alpha(1.000), //#8D1BF5
swoosh_9: rgb_hex(0xD52EF5).with_alpha(1.000), //#D52EF5
swoosh_10: rgb_hex(0xE637C8).with_alpha(1.000), //#E637C8
swoosh_11: rgb_hex(0xE93680).with_alpha(1.000), //#E93680
swoosh_12: rgb_hex(0xE83443).with_alpha(1.000), //#E83443
swoosh_13: rgb_hex(0xE84429).with_alpha(1.000), //#E84429
swoosh_14: rgb_hex(0xEB712E).with_alpha(1.000), //#EB712E
swoosh_15: rgb_hex(0xEBA43B).with_alpha(1.000), //#EBA43B
swoosh_16: rgb_hex(0xDAB23E).with_alpha(1.000), //#DAB23E
swoosh_17: rgb_hex(0xC3B83E).with_alpha(1.000), //#C3B83E
swoosh_18: rgb_hex(0xB2C13E).with_alpha(1.000), //#B2C13E
swoosh_19: rgb_hex(0x95C13E).with_alpha(1.000), //#95C13E
hint_shade: rgb_hex(0x4A2FC4).with_alpha(1.000), //#4A2FC4
type_secondary: rgb_hex(0x57546A).with_alpha(1.000), //#57546A
highlight: rgb_hex(0xEFA03A).with_alpha(1.000), //#EFA03A
grey_3: rgb_hex(0xC9C8D0).with_alpha(1.000), //#C9C8D0
divider: rgb_hex(0x2F2C3E).with_alpha(0.080), //#2F2C3E
border: rgb_hex(0x2F2C3E).with_alpha(0.150), //#2F2C3E
scrim: rgb_hex(0x2F2C3E).with_alpha(0.350), //#2F2C3E
icon_muted: rgb_hex(0x2F2C3E).with_alpha(0.550), //#2F2C3E
on_accent: rgb_hex(0xFFFFFF).with_alpha(0.750), //#FFFFFF
on_accent_faint: rgb_hex(0xFFFFFF).with_alpha(0.300), //#FFFFFF
accent_wash: rgb_hex(0x34C77B).with_alpha(0.500), //#34C77B
logo: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
logo_hero: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
surface_raised: rgb_hex(0xFFFFFF).with_alpha(1.000), //#FFFFFF
firsts_tile: rgb_hex(0xDDF5E7).with_alpha(1.000), //#DDF5E7
firsts_letter: rgb_hex(0x1E8A52).with_alpha(1.000), //#1E8A52
answer_tile: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
panel: rgb_hex(0xDDF5E7).with_alpha(1.000), //#DDF5E7
panel_on_tint: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
on_panel: rgb_hex(0x2F2C3E).with_alpha(1.000), //#2F2C3E
panel_button: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
panel_button_on_tint: rgb_hex(0xFFFFFF).with_alpha(1.000), //#FFFFFF
hint_on_fill: rgb_hex(0x4A2FC4).with_alpha(1.000), //#4A2FC4
badge: rgb_hex(0x34C77B).with_alpha(1.000), //#34C77B
badge_icon: rgb_hex(0x2F2C3E).with_alpha(1.000), //#2F2C3E
on_inverse: rgb_hex(0xFFFFFF).with_alpha(1.000), //#FFFFFF
};

pub const COLOR_SCHEME_KALE: ColorScheme = ColorScheme {
    background: rgb_hex(0x1D1B27).with_alpha(1.000), // #1D1B27
accent_main: rgb_hex(0x34C77B).with_alpha(1.000), // #34C77B
accent_background: rgb_hex(0x34C77B).with_alpha(1.000), // #34C77B
accent_opposite: rgb_hex(0xFFFFFF).with_alpha(1.000), // #FFFFFF
accent_tint: rgb_hex(0x2B2938).with_alpha(1.000), // #2B2938
accent_shade: rgb_hex(0xF2F1F6).with_alpha(1.000), // #F2F1F6
tile_background: rgb_hex(0x262433).with_alpha(1.000), // #262433
hint_main: rgb_hex(0x34C77B).with_alpha(1.000), // #34C77B
hint_tint: rgb_hex(0x34C77B).with_alpha(1.000), // #34C77B
hint_opposite: rgb_hex(0xFFFFFF).with_alpha(1.000), // #FFFFFF
type_dark: rgb_hex(0xF2F1F6).with_alpha(1.000), // #F2F1F6
type_light: rgb_hex(0xFFFFFF).with_alpha(1.000), // #FFFFFF
grey_1: rgb_hex(0xA9A6B8).with_alpha(1.000), // #A9A6B8
grey_2: rgb_hex(0x3D3A4E).with_alpha(1.000), // #3D3A4E
swoosh_1: rgb_hex(0x5BC150).with_alpha(1.000), // #5BC150
swoosh_2: rgb_hex(0x5DC8AF).with_alpha(1.000), // #5DC8AF
swoosh_3: rgb_hex(0x57BFE9).with_alpha(1.000), // #57BFE9
swoosh_4: rgb_hex(0x3D8AF7).with_alpha(1.000), // #3D8AF7
swoosh_5: rgb_hex(0x1D46F6).with_alpha(1.000), // #1D46F6
swoosh_6: rgb_hex(0x1D15F3).with_alpha(1.000), // #1D15F3
swoosh_7: rgb_hex(0x4C0BF1).with_alpha(1.000), // #4C0BF1
swoosh_8: rgb_hex(0x8D1BF5).with_alpha(1.000), // #8D1BF5
swoosh_9: rgb_hex(0xD52EF5).with_alpha(1.000), // #D52EF5
swoosh_10: rgb_hex(0xE637C8).with_alpha(1.000), // #E637C8
swoosh_11: rgb_hex(0xE93680).with_alpha(1.000), // #E93680
swoosh_12: rgb_hex(0xE83443).with_alpha(1.000), // #E83443
swoosh_13: rgb_hex(0xE84429).with_alpha(1.000), // #E84429
swoosh_14: rgb_hex(0xEB712E).with_alpha(1.000), // #EB712E
swoosh_15: rgb_hex(0xEBA43B).with_alpha(1.000), // #EBA43B
swoosh_16: rgb_hex(0xDAB23E).with_alpha(1.000), // #DAB23E
swoosh_17: rgb_hex(0xC3B83E).with_alpha(1.000), // #C3B83E
swoosh_18: rgb_hex(0xB2C13E).with_alpha(1.000), // #B2C13E
swoosh_19: rgb_hex(0x95C13E).with_alpha(1.000), // #95C13E
hint_shade: rgb_hex(0x34C77B).with_alpha(1.000), // #34C77B
type_secondary: rgb_hex(0xA9A6B8).with_alpha(1.000), // #A9A6B8
highlight: rgb_hex(0xA9A9A9).with_alpha(1.000), // #A9A9A9
grey_3: rgb_hex(0x6E6B7E).with_alpha(1.000), // #6E6B7E
divider: rgb_hex(0xF2F1F6).with_alpha(0.100), // #F2F1F6
border: rgb_hex(0xF2F1F6).with_alpha(0.180), // #F2F1F6
scrim: rgb_hex(0x000000).with_alpha(0.500), // #000000
icon_muted: rgb_hex(0xF2F1F6).with_alpha(0.550), // #F2F1F6
on_accent: rgb_hex(0x1D1B27).with_alpha(0.850), // #1D1B27
on_accent_faint: rgb_hex(0x1D1B27).with_alpha(0.300), // #1D1B27
accent_wash: rgb_hex(0xF2F1F6).with_alpha(0.060), // #F2F1F6
logo: rgb_hex(0xA2A2A2).with_alpha(1.000), // #A2A2A2
logo_hero: rgb_hex(0xF2F1F6).with_alpha(1.000), // #F2F1F6
surface_raised: rgb_hex(0x2B2938).with_alpha(1.000), // #2B2938
firsts_tile: rgb_hex(0xEAEAEA).with_alpha(1.000), // #EAEAEA
firsts_letter: rgb_hex(0x4E4E4E).with_alpha(1.000), // #4E4E4E
answer_tile: rgb_hex(0x3A3748).with_alpha(1.000), // #3A3748
panel: rgb_hex(0xF2F1F6).with_alpha(0.100), // #F2F1F6
panel_on_tint: rgb_hex(0xF2F1F6).with_alpha(0.100), // #F2F1F6
on_panel: rgb_hex(0xFFFFFF).with_alpha(1.000), // #FFFFFF
panel_button: rgb_hex(0xF2F1F6).with_alpha(1.000), // #F2F1F6
panel_button_on_tint: rgb_hex(0x34C77B).with_alpha(1.000), // #34C77B
hint_on_fill: rgb_hex(0x1D1B27).with_alpha(1.000), // #1D1B27
badge: rgb_hex(0x1D1B27).with_alpha(1.000), // #1D1B27
badge_icon: rgb_hex(0x34C77B).with_alpha(1.000), // #34C77B
on_inverse: rgb_hex(0x1D1B27).with_alpha(1.000), // #1D1B27
};

pub const fn rgb_hex(number: u32) -> Srgba {
    Srgba::rgb_hex(number)
}

pub const fn rgba_hex(number: u32) -> Srgba {
    Srgba::rgba_hex(number)
}
