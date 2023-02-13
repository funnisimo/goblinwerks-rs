use super::COLORS;
use super::{RGB, RGBA};

// Named Colors (derived from X11 rgb.txt, which is also the source of HTML/W3C/SVG names)
pub const NONE: RGBA = RGBA::rgba(0, 0, 0, 0);
pub const SNOW: RGB = (255, 250, 250);
pub const GHOST_WHITE: RGB = (248, 248, 255);
pub const GHOSTWHITE: RGB = (248, 248, 255);
pub const WHITE_SMOKE: RGB = (245, 245, 245);
pub const WHITESMOKE: RGB = (245, 245, 245);
pub const GAINSBORO: RGB = (220, 220, 220);
pub const FLORAL_WHITE: RGB = (255, 250, 240);
pub const FLORALWHITE: RGB = (255, 250, 240);
pub const OLD_LACE: RGB = (253, 245, 230);
pub const OLDLACE: RGB = (253, 245, 230);
pub const LINEN: RGB = (250, 240, 230);
pub const ANTIQUE_WHITE: RGB = (250, 235, 215);
pub const ANTIQUEWHITE: RGB = (250, 235, 215);
pub const PAPAYA_WHIP: RGB = (255, 239, 213);
pub const PAPAYAWHIP: RGB = (255, 239, 213);
pub const BLANCHED_ALMOND: RGB = (255, 235, 205);
pub const BLANCHEDALMOND: RGB = (255, 235, 205);
pub const BISQUE: RGB = (255, 228, 196);
pub const PEACH_PUFF: RGB = (255, 218, 185);
pub const PEACHPUFF: RGB = (255, 218, 185);
pub const NAVAJO_WHITE: RGB = (255, 222, 173);
pub const NAVAJOWHITE: RGB = (255, 222, 173);
pub const MOCCASIN: RGB = (255, 228, 181);
pub const CORNSILK: RGB = (255, 248, 220);
pub const IVORY: RGB = (255, 255, 240);
pub const LEMON_CHIFFON: RGB = (255, 250, 205);
pub const LEMONCHIFFON: RGB = (255, 250, 205);
pub const SEASHELL: RGB = (255, 245, 238);
pub const HONEYDEW: RGB = (240, 255, 240);
pub const MINT_CREAM: RGB = (245, 255, 250);
pub const MINTCREAM: RGB = (245, 255, 250);
pub const AZURE: RGB = (240, 255, 255);
pub const ALICE_BLUE: RGB = (240, 248, 255);
pub const ALICEBLUE: RGB = (240, 248, 255);
pub const LAVENDER: RGB = (230, 230, 250);
pub const LAVENDER_BLUSH: RGB = (255, 240, 245);
pub const LAVENDERBLUSH: RGB = (255, 240, 245);
pub const MISTY_ROSE: RGB = (255, 228, 225);
pub const MISTYROSE: RGB = (255, 228, 225);
pub const WHITE: RGB = (255, 255, 255);
pub const BLACK: RGB = (0, 0, 0);
pub const DARK_SLATE: RGB = (47, 79, 79);
pub const DARKSLATEGRAY: RGB = (47, 79, 79);
pub const DARKSLATEGREY: RGB = (47, 79, 79);
pub const DIM_GRAY: RGB = (105, 105, 105);
pub const DIMGRAY: RGB = (105, 105, 105);
pub const DIM_GREY: RGB = (105, 105, 105);
pub const DIMGREY: RGB = (105, 105, 105);
pub const SLATE_GRAY: RGB = (112, 128, 144);
pub const SLATEGRAY: RGB = (112, 128, 144);
pub const SLATE_GREY: RGB = (112, 128, 144);
pub const SLATEGREY: RGB = (112, 128, 144);
pub const LIGHT_SLATE: RGB = (119, 136, 153);
pub const LIGHTSLATEGRAY: RGB = (119, 136, 153);
pub const LIGHTSLATEGREY: RGB = (119, 136, 153);
pub const GRAY: RGB = (190, 190, 190);
pub const GREY: RGB = (190, 190, 190);
pub const X11_GRAY: RGB = (190, 190, 190);
pub const X11GRAY: RGB = (190, 190, 190);
pub const X11_GREY: RGB = (190, 190, 190);
pub const X11GREY: RGB = (190, 190, 190);
pub const WEB_GRAY: RGB = (128, 128, 128);
pub const WEBGRAY: RGB = (128, 128, 128);
pub const WEB_GREY: RGB = (128, 128, 128);
pub const WEBGREY: RGB = (128, 128, 128);
pub const LIGHT_GREY: RGB = (211, 211, 211);
pub const LIGHTGREY: RGB = (211, 211, 211);
pub const LIGHT_GRAY: RGB = (211, 211, 211);
pub const LIGHTGRAY: RGB = (211, 211, 211);
pub const MIDNIGHT_BLUE: RGB = (25, 25, 112);
pub const MIDNIGHTBLUE: RGB = (25, 25, 112);
pub const NAVY: RGB = (0, 0, 128);
pub const NAVY_BLUE: RGB = (0, 0, 128);
pub const NAVYBLUE: RGB = (0, 0, 128);
pub const CORNFLOWER_BLUE: RGB = (100, 149, 237);
pub const CORNFLOWERBLUE: RGB = (100, 149, 237);
pub const DARKSLATEBLUE: RGB = (72, 61, 139);
pub const SLATE_BLUE: RGB = (106, 90, 205);
pub const SLATEBLUE: RGB = (106, 90, 205);
pub const MEDIUM_SLATE: RGB = (123, 104, 238);
pub const MEDIUMSLATEBLUE: RGB = (123, 104, 238);
pub const LIGHTSLATEBLUE: RGB = (132, 112, 255);
pub const MEDIUM_BLUE: RGB = (0, 0, 205);
pub const MEDIUMBLUE: RGB = (0, 0, 205);
pub const ROYAL_BLUE: RGB = (65, 105, 225);
pub const ROYALBLUE: RGB = (65, 105, 225);
pub const BLUE: RGB = (0, 0, 255);
pub const DODGER_BLUE: RGB = (30, 144, 255);
pub const DODGERBLUE: RGB = (30, 144, 255);
pub const DEEP_SKY: RGB = (0, 191, 255);
pub const DEEPSKYBLUE: RGB = (0, 191, 255);
pub const SKY_BLUE: RGB = (135, 206, 235);
pub const SKYBLUE: RGB = (135, 206, 235);
pub const LIGHT_SKY: RGB = (135, 206, 250);
pub const LIGHTSKYBLUE: RGB = (135, 206, 250);
pub const STEEL_BLUE: RGB = (70, 130, 180);
pub const STEELBLUE: RGB = (70, 130, 180);
pub const LIGHT_STEEL: RGB = (176, 196, 222);
pub const LIGHTSTEELBLUE: RGB = (176, 196, 222);
pub const LIGHT_BLUE: RGB = (173, 216, 230);
pub const LIGHTBLUE: RGB = (173, 216, 230);
pub const POWDER_BLUE: RGB = (176, 224, 230);
pub const POWDERBLUE: RGB = (176, 224, 230);
pub const PALE_TURQUOISE: RGB = (175, 238, 238);
pub const PALETURQUOISE: RGB = (175, 238, 238);
pub const DARK_TURQUOISE: RGB = (0, 206, 209);
pub const DARKTURQUOISE: RGB = (0, 206, 209);
pub const MEDIUM_TURQUOISE: RGB = (72, 209, 204);
pub const MEDIUMTURQUOISE: RGB = (72, 209, 204);
pub const TURQUOISE: RGB = (64, 224, 208);
pub const CYAN: RGB = (0, 255, 255);
pub const AQUA: RGB = (0, 255, 255);
pub const LIGHT_CYAN: RGB = (224, 255, 255);
pub const LIGHTCYAN: RGB = (224, 255, 255);
pub const CADET_BLUE: RGB = (95, 158, 160);
pub const CADETBLUE: RGB = (95, 158, 160);
pub const MEDIUM_AQUAMARINE: RGB = (102, 205, 170);
pub const MEDIUMAQUAMARINE: RGB = (102, 205, 170);
pub const AQUAMARINE: RGB = (127, 255, 212);
pub const DARK_GREEN: RGB = (0, 100, 0);
pub const DARKGREEN: RGB = (0, 100, 0);
pub const DARK_OLIVE: RGB = (85, 107, 47);
pub const DARKOLIVEGREEN: RGB = (85, 107, 47);
pub const DARK_SEA: RGB = (143, 188, 143);
pub const DARKSEAGREEN: RGB = (143, 188, 143);
pub const SEA_GREEN: RGB = (46, 139, 87);
pub const SEAGREEN: RGB = (46, 139, 87);
pub const MEDIUM_SEA: RGB = (60, 179, 113);
pub const MEDIUMSEAGREEN: RGB = (60, 179, 113);
pub const LIGHT_SEA: RGB = (32, 178, 170);
pub const LIGHTSEAGREEN: RGB = (32, 178, 170);
pub const PALE_GREEN: RGB = (152, 251, 152);
pub const PALEGREEN: RGB = (152, 251, 152);
pub const SPRING_GREEN: RGB = (0, 255, 127);
pub const SPRINGGREEN: RGB = (0, 255, 127);
pub const LAWN_GREEN: RGB = (124, 252, 0);
pub const LAWNGREEN: RGB = (124, 252, 0);
pub const GREEN: RGB = (0, 255, 0);
pub const LIME: RGB = (0, 255, 0);
pub const X11_GREEN: RGB = (0, 255, 0);
pub const X11GREEN: RGB = (0, 255, 0);
pub const WEB_GREEN: RGB = (0, 128, 0);
pub const WEBGREEN: RGB = (0, 128, 0);
pub const CHARTREUSE: RGB = (127, 255, 0);
pub const MEDIUM_SPRING: RGB = (0, 250, 154);
pub const MEDIUMSPRINGGREEN: RGB = (0, 250, 154);
pub const GREEN_YELLOW: RGB = (173, 255, 47);
pub const GREENYELLOW: RGB = (173, 255, 47);
pub const LIME_GREEN: RGB = (50, 205, 50);
pub const LIMEGREEN: RGB = (50, 205, 50);
pub const YELLOW_GREEN: RGB = (154, 205, 50);
pub const YELLOWGREEN: RGB = (154, 205, 50);
pub const FOREST_GREEN: RGB = (34, 139, 34);
pub const FORESTGREEN: RGB = (34, 139, 34);
pub const OLIVE_DRAB: RGB = (107, 142, 35);
pub const OLIVEDRAB: RGB = (107, 142, 35);
pub const DARK_KHAKI: RGB = (189, 183, 107);
pub const DARKKHAKI: RGB = (189, 183, 107);
pub const KHAKI: RGB = (240, 230, 140);
pub const PALE_GOLDENROD: RGB = (238, 232, 170);
pub const PALEGOLDENROD: RGB = (238, 232, 170);
pub const LIGHT_GOLDENROD: RGB = (250, 250, 210);
pub const LIGHTGOLDENRODYELLOW: RGB = (250, 250, 210);
pub const LIGHT_YELLOW: RGB = (255, 255, 224);
pub const LIGHTYELLOW: RGB = (255, 255, 224);
pub const YELLOW: RGB = (255, 255, 0);
pub const GOLD: RGB = (255, 215, 0);
pub const LIGHTGOLDENROD: RGB = (238, 221, 130);
pub const GOLDENROD: RGB = (218, 165, 32);
pub const DARK_GOLDENROD: RGB = (184, 134, 11);
pub const DARKGOLDENROD: RGB = (184, 134, 11);
pub const ROSY_BROWN: RGB = (188, 143, 143);
pub const ROSYBROWN: RGB = (188, 143, 143);
pub const INDIAN_RED: RGB = (205, 92, 92);
pub const INDIANRED: RGB = (205, 92, 92);
pub const SADDLE_BROWN: RGB = (139, 69, 19);
pub const SADDLEBROWN: RGB = (139, 69, 19);
pub const SIENNA: RGB = (160, 82, 45);
pub const PERU: RGB = (205, 133, 63);
pub const BURLYWOOD: RGB = (222, 184, 135);
pub const BEIGE: RGB = (245, 245, 220);
pub const WHEAT: RGB = (245, 222, 179);
pub const SANDY_BROWN: RGB = (244, 164, 96);
pub const SANDYBROWN: RGB = (244, 164, 96);
pub const TAN: RGB = (210, 180, 140);
pub const CHOCOLATE: RGB = (210, 105, 30);
pub const FIREBRICK_34: RGB = (178, 34, 34);
pub const BROWN_42: RGB = (165, 42, 42);
pub const DARK_SALMON: RGB = (233, 150, 122);
pub const DARKSALMON: RGB = (233, 150, 122);
pub const SALMON: RGB = (250, 128, 114);
pub const LIGHT_SALMON: RGB = (255, 160, 122);
pub const LIGHTSALMON: RGB = (255, 160, 122);
pub const ORANGE: RGB = (255, 165, 0);
pub const DARK_ORANGE: RGB = (255, 140, 0);
pub const DARKORANGE: RGB = (255, 140, 0);
pub const CORAL: RGB = (255, 127, 80);
pub const LIGHT_CORAL: RGB = (240, 128, 128);
pub const LIGHTCORAL: RGB = (240, 128, 128);
pub const TOMATO: RGB = (255, 99, 71);
pub const ORANGE_RED: RGB = (255, 69, 0);
pub const ORANGERED: RGB = (255, 69, 0);
pub const RED: RGB = (255, 0, 0);
pub const HOT_PINK: RGB = (255, 105, 180);
pub const HOTPINK: RGB = (255, 105, 180);
pub const DEEP_PINK: RGB = (255, 20, 147);
pub const DEEPPINK: RGB = (255, 20, 147);
pub const PINK: RGB = (255, 192, 203);
pub const LIGHT_PINK: RGB = (255, 182, 193);
pub const LIGHTPINK: RGB = (255, 182, 193);
pub const PALE_VIOLET: RGB = (219, 112, 147);
pub const PALEVIOLETRED: RGB = (219, 112, 147);
pub const MAROON: RGB = (176, 48, 96);
pub const X11_MAROON: RGB = (176, 48, 96);
pub const X11MAROON: RGB = (176, 48, 96);
pub const WEB_MAROON: RGB = (128, 0, 0);
pub const WEBMAROON: RGB = (128, 0, 0);
pub const MEDIUM_VIOLET: RGB = (199, 21, 133);
pub const MEDIUMVIOLETRED: RGB = (199, 21, 133);
pub const VIOLET_RED: RGB = (208, 32, 144);
pub const VIOLETRED: RGB = (208, 32, 144);
pub const MAGENTA: RGB = (255, 0, 255);
pub const FUCHSIA: RGB = (255, 0, 255);
pub const VIOLET: RGB = (238, 130, 238);
pub const PLUM: RGB = (221, 160, 221);
pub const ORCHID: RGB = (218, 112, 214);
pub const MEDIUM_ORCHID: RGB = (186, 85, 211);
pub const MEDIUMORCHID: RGB = (186, 85, 211);
pub const DARK_ORCHID: RGB = (153, 50, 204);
pub const DARKORCHID: RGB = (153, 50, 204);
pub const DARK_VIOLET: RGB = (148, 0, 211);
pub const DARKVIOLET: RGB = (148, 0, 211);
pub const BLUE_VIOLET: RGB = (138, 43, 226);
pub const BLUEVIOLET: RGB = (138, 43, 226);
pub const PURPLE: RGB = (160, 32, 240);
pub const X11_PURPLE: RGB = (160, 32, 240);
pub const X11PURPLE: RGB = (160, 32, 240);
pub const WEB_PURPLE: RGB = (128, 0, 128);
pub const WEBPURPLE: RGB = (128, 0, 128);
pub const MEDIUM_PURPLE: RGB = (147, 112, 219);
pub const MEDIUMPURPLE: RGB = (147, 112, 219);
pub const THISTLE: RGB = (216, 191, 216);
pub const SNOW1: RGB = (255, 250, 250);
pub const SNOW2: RGB = (238, 233, 233);
pub const SNOW3: RGB = (205, 201, 201);
pub const SNOW4: RGB = (139, 137, 137);
pub const SEASHELL1: RGB = (255, 245, 238);
pub const SEASHELL2: RGB = (238, 229, 222);
pub const SEASHELL3: RGB = (205, 197, 191);
pub const SEASHELL4: RGB = (139, 134, 130);
pub const ANTIQUEWHITE1: RGB = (255, 239, 219);
pub const ANTIQUEWHITE2: RGB = (238, 223, 204);
pub const ANTIQUEWHITE3: RGB = (205, 192, 176);
pub const ANTIQUEWHITE4: RGB = (139, 131, 120);
pub const BISQUE1: RGB = (255, 228, 196);
pub const BISQUE2: RGB = (238, 213, 183);
pub const BISQUE3: RGB = (205, 183, 158);
pub const BISQUE4: RGB = (139, 125, 107);
pub const PEACHPUFF1: RGB = (255, 218, 185);
pub const PEACHPUFF2: RGB = (238, 203, 173);
pub const PEACHPUFF3: RGB = (205, 175, 149);
pub const PEACHPUFF4: RGB = (139, 119, 101);
pub const NAVAJOWHITE1: RGB = (255, 222, 173);
pub const NAVAJOWHITE2: RGB = (238, 207, 161);
pub const NAVAJOWHITE3: RGB = (205, 179, 139);
pub const NAVAJOWHITE4: RGB = (139, 121, 94);
pub const LEMONCHIFFON1: RGB = (255, 250, 205);
pub const LEMONCHIFFON2: RGB = (238, 233, 191);
pub const LEMONCHIFFON3: RGB = (205, 201, 165);
pub const LEMONCHIFFON4: RGB = (139, 137, 112);
pub const CORNSILK1: RGB = (255, 248, 220);
pub const CORNSILK2: RGB = (238, 232, 205);
pub const CORNSILK3: RGB = (205, 200, 177);
pub const CORNSILK4: RGB = (139, 136, 120);
pub const IVORY1: RGB = (255, 255, 240);
pub const IVORY2: RGB = (238, 238, 224);
pub const IVORY3: RGB = (205, 205, 193);
pub const IVORY4: RGB = (139, 139, 131);
pub const HONEYDEW1: RGB = (240, 255, 240);
pub const HONEYDEW2: RGB = (224, 238, 224);
pub const HONEYDEW3: RGB = (193, 205, 193);
pub const HONEYDEW4: RGB = (131, 139, 131);
pub const LAVENDERBLUSH1: RGB = (255, 240, 245);
pub const LAVENDERBLUSH2: RGB = (238, 224, 229);
pub const LAVENDERBLUSH3: RGB = (205, 193, 197);
pub const LAVENDERBLUSH4: RGB = (139, 131, 134);
pub const MISTYROSE1: RGB = (255, 228, 225);
pub const MISTYROSE2: RGB = (238, 213, 210);
pub const MISTYROSE3: RGB = (205, 183, 181);
pub const MISTYROSE4: RGB = (139, 125, 123);
pub const AZURE1: RGB = (240, 255, 255);
pub const AZURE2: RGB = (224, 238, 238);
pub const AZURE3: RGB = (193, 205, 205);
pub const AZURE4: RGB = (131, 139, 139);
pub const SLATEBLUE1: RGB = (131, 111, 255);
pub const SLATEBLUE2: RGB = (122, 103, 238);
pub const SLATEBLUE3: RGB = (105, 89, 205);
pub const SLATEBLUE4: RGB = (71, 60, 139);
pub const ROYALBLUE1: RGB = (72, 118, 255);
pub const ROYALBLUE2: RGB = (67, 110, 238);
pub const ROYALBLUE3: RGB = (58, 95, 205);
pub const ROYALBLUE4: RGB = (39, 64, 139);
pub const BLUE1: RGB = (0, 0, 255);
pub const BLUE2: RGB = (0, 0, 238);
pub const BLUE3: RGB = (0, 0, 205);
pub const BLUE4: RGB = (0, 0, 139);
pub const DODGERBLUE1: RGB = (30, 144, 255);
pub const DODGERBLUE2: RGB = (28, 134, 238);
pub const DODGERBLUE3: RGB = (24, 116, 205);
pub const DODGERBLUE4: RGB = (16, 78, 139);
pub const STEELBLUE1: RGB = (99, 184, 255);
pub const STEELBLUE2: RGB = (92, 172, 238);
pub const STEELBLUE3: RGB = (79, 148, 205);
pub const STEELBLUE4: RGB = (54, 100, 139);
pub const DEEPSKYBLUE1: RGB = (0, 191, 255);
pub const DEEPSKYBLUE2: RGB = (0, 178, 238);
pub const DEEPSKYBLUE3: RGB = (0, 154, 205);
pub const DEEPSKYBLUE4: RGB = (0, 104, 139);
pub const SKYBLUE1: RGB = (135, 206, 255);
pub const SKYBLUE2: RGB = (126, 192, 238);
pub const SKYBLUE3: RGB = (108, 166, 205);
pub const SKYBLUE4: RGB = (74, 112, 139);
pub const LIGHTSKYBLUE1: RGB = (176, 226, 255);
pub const LIGHTSKYBLUE2: RGB = (164, 211, 238);
pub const LIGHTSKYBLUE3: RGB = (141, 182, 205);
pub const LIGHTSKYBLUE4: RGB = (96, 123, 139);
pub const SLATEGRAY1: RGB = (198, 226, 255);
pub const SLATEGRAY2: RGB = (185, 211, 238);
pub const SLATEGRAY3: RGB = (159, 182, 205);
pub const SLATEGRAY4: RGB = (108, 123, 139);
pub const LIGHTSTEELBLUE1: RGB = (202, 225, 255);
pub const LIGHTSTEELBLUE2: RGB = (188, 210, 238);
pub const LIGHTSTEELBLUE3: RGB = (162, 181, 205);
pub const LIGHTSTEELBLUE4: RGB = (110, 123, 139);
pub const LIGHTBLUE1: RGB = (191, 239, 255);
pub const LIGHTBLUE2: RGB = (178, 223, 238);
pub const LIGHTBLUE3: RGB = (154, 192, 205);
pub const LIGHTBLUE4: RGB = (104, 131, 139);
pub const LIGHTCYAN1: RGB = (224, 255, 255);
pub const LIGHTCYAN2: RGB = (209, 238, 238);
pub const LIGHTCYAN3: RGB = (180, 205, 205);
pub const LIGHTCYAN4: RGB = (122, 139, 139);
pub const PALETURQUOISE1: RGB = (187, 255, 255);
pub const PALETURQUOISE2: RGB = (174, 238, 238);
pub const PALETURQUOISE3: RGB = (150, 205, 205);
pub const PALETURQUOISE4: RGB = (102, 139, 139);
pub const CADETBLUE1: RGB = (152, 245, 255);
pub const CADETBLUE2: RGB = (142, 229, 238);
pub const CADETBLUE3: RGB = (122, 197, 205);
pub const CADETBLUE4: RGB = (83, 134, 139);
pub const TURQUOISE1: RGB = (0, 245, 255);
pub const TURQUOISE2: RGB = (0, 229, 238);
pub const TURQUOISE3: RGB = (0, 197, 205);
pub const TURQUOISE4: RGB = (0, 134, 139);
pub const CYAN1: RGB = (0, 255, 255);
pub const CYAN2: RGB = (0, 238, 238);
pub const CYAN3: RGB = (0, 205, 205);
pub const CYAN4: RGB = (0, 139, 139);
pub const DARKSLATEGRAY1: RGB = (151, 255, 255);
pub const DARKSLATEGRAY2: RGB = (141, 238, 238);
pub const DARKSLATEGRAY3: RGB = (121, 205, 205);
pub const DARKSLATEGRAY4: RGB = (82, 139, 139);
pub const AQUAMARINE1: RGB = (127, 255, 212);
pub const AQUAMARINE2: RGB = (118, 238, 198);
pub const AQUAMARINE3: RGB = (102, 205, 170);
pub const AQUAMARINE4: RGB = (69, 139, 116);
pub const DARKSEAGREEN1: RGB = (193, 255, 193);
pub const DARKSEAGREEN2: RGB = (180, 238, 180);
pub const DARKSEAGREEN3: RGB = (155, 205, 155);
pub const DARKSEAGREEN4: RGB = (105, 139, 105);
pub const SEAGREEN1: RGB = (84, 255, 159);
pub const SEAGREEN2: RGB = (78, 238, 148);
pub const SEAGREEN3: RGB = (67, 205, 128);
pub const SEAGREEN4: RGB = (46, 139, 87);
pub const PALEGREEN1: RGB = (154, 255, 154);
pub const PALEGREEN2: RGB = (144, 238, 144);
pub const PALEGREEN3: RGB = (124, 205, 124);
pub const PALEGREEN4: RGB = (84, 139, 84);
pub const SPRINGGREEN1: RGB = (0, 255, 127);
pub const SPRINGGREEN2: RGB = (0, 238, 118);
pub const SPRINGGREEN3: RGB = (0, 205, 102);
pub const SPRINGGREEN4: RGB = (0, 139, 69);
pub const GREEN1: RGB = (0, 255, 0);
pub const GREEN2: RGB = (0, 238, 0);
pub const GREEN3: RGB = (0, 205, 0);
pub const GREEN4: RGB = (0, 139, 0);
pub const CHARTREUSE1: RGB = (127, 255, 0);
pub const CHARTREUSE2: RGB = (118, 238, 0);
pub const CHARTREUSE3: RGB = (102, 205, 0);
pub const CHARTREUSE4: RGB = (69, 139, 0);
pub const OLIVEDRAB1: RGB = (192, 255, 62);
pub const OLIVEDRAB2: RGB = (179, 238, 58);
pub const OLIVEDRAB3: RGB = (154, 205, 50);
pub const OLIVEDRAB4: RGB = (105, 139, 34);
pub const DARKOLIVEGREEN1: RGB = (202, 255, 112);
pub const DARKOLIVEGREEN2: RGB = (188, 238, 104);
pub const DARKOLIVEGREEN3: RGB = (162, 205, 90);
pub const DARKOLIVEGREEN4: RGB = (110, 139, 61);
pub const KHAKI1: RGB = (255, 246, 143);
pub const KHAKI2: RGB = (238, 230, 133);
pub const KHAKI3: RGB = (205, 198, 115);
pub const KHAKI4: RGB = (139, 134, 78);
pub const LIGHTGOLDENROD1: RGB = (255, 236, 139);
pub const LIGHTGOLDENROD2: RGB = (238, 220, 130);
pub const LIGHTGOLDENROD3: RGB = (205, 190, 112);
pub const LIGHTGOLDENROD4: RGB = (139, 129, 76);
pub const LIGHTYELLOW1: RGB = (255, 255, 224);
pub const LIGHTYELLOW2: RGB = (238, 238, 209);
pub const LIGHTYELLOW3: RGB = (205, 205, 180);
pub const LIGHTYELLOW4: RGB = (139, 139, 122);
pub const YELLOW1: RGB = (255, 255, 0);
pub const YELLOW2: RGB = (238, 238, 0);
pub const YELLOW3: RGB = (205, 205, 0);
pub const YELLOW4: RGB = (139, 139, 0);
pub const GOLD1: RGB = (255, 215, 0);
pub const GOLD2: RGB = (238, 201, 0);
pub const GOLD3: RGB = (205, 173, 0);
pub const GOLD4: RGB = (139, 117, 0);
pub const GOLDENROD1: RGB = (255, 193, 37);
pub const GOLDENROD2: RGB = (238, 180, 34);
pub const GOLDENROD3: RGB = (205, 155, 29);
pub const GOLDENROD4: RGB = (139, 105, 20);
pub const DARKGOLDENROD1: RGB = (255, 185, 15);
pub const DARKGOLDENROD2: RGB = (238, 173, 14);
pub const DARKGOLDENROD3: RGB = (205, 149, 12);
pub const DARKGOLDENROD4: RGB = (139, 101, 8);
pub const ROSYBROWN1: RGB = (255, 193, 193);
pub const ROSYBROWN2: RGB = (238, 180, 180);
pub const ROSYBROWN3: RGB = (205, 155, 155);
pub const ROSYBROWN4: RGB = (139, 105, 105);
pub const INDIANRED1: RGB = (255, 106, 106);
pub const INDIANRED2: RGB = (238, 99, 99);
pub const INDIANRED3: RGB = (205, 85, 85);
pub const INDIANRED4: RGB = (139, 58, 58);
pub const SIENNA1: RGB = (255, 130, 71);
pub const SIENNA2: RGB = (238, 121, 66);
pub const SIENNA3: RGB = (205, 104, 57);
pub const SIENNA4: RGB = (139, 71, 38);
pub const BURLYWOOD1: RGB = (255, 211, 155);
pub const BURLYWOOD2: RGB = (238, 197, 145);
pub const BURLYWOOD3: RGB = (205, 170, 125);
pub const BURLYWOOD4: RGB = (139, 115, 85);
pub const WHEAT1: RGB = (255, 231, 186);
pub const WHEAT2: RGB = (238, 216, 174);
pub const WHEAT3: RGB = (205, 186, 150);
pub const WHEAT4: RGB = (139, 126, 102);
pub const TAN1: RGB = (255, 165, 79);
pub const TAN2: RGB = (238, 154, 73);
pub const TAN3: RGB = (205, 133, 63);
pub const TAN4: RGB = (139, 90, 43);
pub const CHOCOLATE1: RGB = (255, 127, 36);
pub const CHOCOLATE2: RGB = (238, 118, 33);
pub const CHOCOLATE3: RGB = (205, 102, 29);
pub const CHOCOLATE4: RGB = (139, 69, 19);
pub const FIREBRICK1: RGB = (255, 48, 48);
pub const FIREBRICK2: RGB = (238, 44, 44);
pub const FIREBRICK3: RGB = (205, 38, 38);
pub const FIREBRICK4: RGB = (139, 26, 26);
pub const BROWN1: RGB = (255, 64, 64);
pub const BROWN2: RGB = (238, 59, 59);
pub const BROWN3: RGB = (205, 51, 51);
pub const BROWN4: RGB = (139, 35, 35);
pub const SALMON1: RGB = (255, 140, 105);
pub const SALMON2: RGB = (238, 130, 98);
pub const SALMON3: RGB = (205, 112, 84);
pub const SALMON4: RGB = (139, 76, 57);
pub const LIGHTSALMON1: RGB = (255, 160, 122);
pub const LIGHTSALMON2: RGB = (238, 149, 114);
pub const LIGHTSALMON3: RGB = (205, 129, 98);
pub const LIGHTSALMON4: RGB = (139, 87, 66);
pub const ORANGE1: RGB = (255, 165, 0);
pub const ORANGE2: RGB = (238, 154, 0);
pub const ORANGE3: RGB = (205, 133, 0);
pub const ORANGE4: RGB = (139, 90, 0);
pub const DARKORANGE1: RGB = (255, 127, 0);
pub const DARKORANGE2: RGB = (238, 118, 0);
pub const DARKORANGE3: RGB = (205, 102, 0);
pub const DARKORANGE4: RGB = (139, 69, 0);
pub const CORAL1: RGB = (255, 114, 86);
pub const CORAL2: RGB = (238, 106, 80);
pub const CORAL3: RGB = (205, 91, 69);
pub const CORAL4: RGB = (139, 62, 47);
pub const TOMATO1: RGB = (255, 99, 71);
pub const TOMATO2: RGB = (238, 92, 66);
pub const TOMATO3: RGB = (205, 79, 57);
pub const TOMATO4: RGB = (139, 54, 38);
pub const ORANGERED1: RGB = (255, 69, 0);
pub const ORANGERED2: RGB = (238, 64, 0);
pub const ORANGERED3: RGB = (205, 55, 0);
pub const ORANGERED4: RGB = (139, 37, 0);
pub const RED1: RGB = (255, 0, 0);
pub const RED2: RGB = (238, 0, 0);
pub const RED3: RGB = (205, 0, 0);
pub const RED4: RGB = (139, 0, 0);
pub const DEEPPINK1: RGB = (255, 20, 147);
pub const DEEPPINK2: RGB = (238, 18, 137);
pub const DEEPPINK3: RGB = (205, 16, 118);
pub const DEEPPINK4: RGB = (139, 10, 80);
pub const HOTPINK1: RGB = (255, 110, 180);
pub const HOTPINK2: RGB = (238, 106, 167);
pub const HOTPINK3: RGB = (205, 96, 144);
pub const HOTPINK4: RGB = (139, 58, 98);
pub const PINK1: RGB = (255, 181, 197);
pub const PINK2: RGB = (238, 169, 184);
pub const PINK3: RGB = (205, 145, 158);
pub const PINK4: RGB = (139, 99, 108);
pub const LIGHTPINK1: RGB = (255, 174, 185);
pub const LIGHTPINK2: RGB = (238, 162, 173);
pub const LIGHTPINK3: RGB = (205, 140, 149);
pub const LIGHTPINK4: RGB = (139, 95, 101);
pub const PALEVIOLETRED1: RGB = (255, 130, 171);
pub const PALEVIOLETRED2: RGB = (238, 121, 159);
pub const PALEVIOLETRED3: RGB = (205, 104, 137);
pub const PALEVIOLETRED4: RGB = (139, 71, 93);
pub const MAROON1: RGB = (255, 52, 179);
pub const MAROON2: RGB = (238, 48, 167);
pub const MAROON3: RGB = (205, 41, 144);
pub const MAROON4: RGB = (139, 28, 98);
pub const VIOLETRED1: RGB = (255, 62, 150);
pub const VIOLETRED2: RGB = (238, 58, 140);
pub const VIOLETRED3: RGB = (205, 50, 120);
pub const VIOLETRED4: RGB = (139, 34, 82);
pub const MAGENTA1: RGB = (255, 0, 255);
pub const MAGENTA2: RGB = (238, 0, 238);
pub const MAGENTA3: RGB = (205, 0, 205);
pub const MAGENTA4: RGB = (139, 0, 139);
pub const ORCHID1: RGB = (255, 131, 250);
pub const ORCHID2: RGB = (238, 122, 233);
pub const ORCHID3: RGB = (205, 105, 201);
pub const ORCHID4: RGB = (139, 71, 137);
pub const PLUM1: RGB = (255, 187, 255);
pub const PLUM2: RGB = (238, 174, 238);
pub const PLUM3: RGB = (205, 150, 205);
pub const PLUM4: RGB = (139, 102, 139);
pub const MEDIUMORCHID1: RGB = (224, 102, 255);
pub const MEDIUMORCHID2: RGB = (209, 95, 238);
pub const MEDIUMORCHID3: RGB = (180, 82, 205);
pub const MEDIUMORCHID4: RGB = (122, 55, 139);
pub const DARKORCHID1: RGB = (191, 62, 255);
pub const DARKORCHID2: RGB = (178, 58, 238);
pub const DARKORCHID3: RGB = (154, 50, 205);
pub const DARKORCHID4: RGB = (104, 34, 139);
pub const PURPLE1: RGB = (155, 48, 255);
pub const PURPLE2: RGB = (145, 44, 238);
pub const PURPLE3: RGB = (125, 38, 205);
pub const PURPLE4: RGB = (85, 26, 139);
pub const MEDIUMPURPLE1: RGB = (171, 130, 255);
pub const MEDIUMPURPLE2: RGB = (159, 121, 238);
pub const MEDIUMPURPLE3: RGB = (137, 104, 205);
pub const MEDIUMPURPLE4: RGB = (93, 71, 139);
pub const THISTLE1: RGB = (255, 225, 255);
pub const THISTLE2: RGB = (238, 210, 238);
pub const THISTLE3: RGB = (205, 181, 205);
pub const THISTLE4: RGB = (139, 123, 139);
pub const GRAY0: RGB = (0, 0, 0);
pub const GREY0: RGB = (0, 0, 0);
pub const GRAY1: RGB = (3, 3, 3);
pub const GREY1: RGB = (3, 3, 3);
pub const GRAY2: RGB = (5, 5, 5);
pub const GREY2: RGB = (5, 5, 5);
pub const GRAY3: RGB = (8, 8, 8);
pub const GREY3: RGB = (8, 8, 8);
pub const GRAY4: RGB = (10, 10, 10);
pub const GREY4: RGB = (10, 10, 10);
pub const GRAY5: RGB = (13, 13, 13);
pub const GREY5: RGB = (13, 13, 13);
pub const GRAY6: RGB = (15, 15, 15);
pub const GREY6: RGB = (15, 15, 15);
pub const GRAY7: RGB = (18, 18, 18);
pub const GREY7: RGB = (18, 18, 18);
pub const GRAY8: RGB = (20, 20, 20);
pub const GREY8: RGB = (20, 20, 20);
pub const GRAY9: RGB = (23, 23, 23);
pub const GREY9: RGB = (23, 23, 23);
pub const GRAY10: RGB = (26, 26, 26);
pub const GREY10: RGB = (26, 26, 26);
pub const GRAY11: RGB = (28, 28, 28);
pub const GREY11: RGB = (28, 28, 28);
pub const GRAY12: RGB = (31, 31, 31);
pub const GREY12: RGB = (31, 31, 31);
pub const GRAY13: RGB = (33, 33, 33);
pub const GREY13: RGB = (33, 33, 33);
pub const GRAY14: RGB = (36, 36, 36);
pub const GREY14: RGB = (36, 36, 36);
pub const GRAY15: RGB = (38, 38, 38);
pub const GREY15: RGB = (38, 38, 38);
pub const GRAY16: RGB = (41, 41, 41);
pub const GREY16: RGB = (41, 41, 41);
pub const GRAY17: RGB = (43, 43, 43);
pub const GREY17: RGB = (43, 43, 43);
pub const GRAY18: RGB = (46, 46, 46);
pub const GREY18: RGB = (46, 46, 46);
pub const GRAY19: RGB = (48, 48, 48);
pub const GREY19: RGB = (48, 48, 48);
pub const GRAY20: RGB = (51, 51, 51);
pub const GREY20: RGB = (51, 51, 51);
pub const GRAY21: RGB = (54, 54, 54);
pub const GREY21: RGB = (54, 54, 54);
pub const GRAY22: RGB = (56, 56, 56);
pub const GREY22: RGB = (56, 56, 56);
pub const GRAY23: RGB = (59, 59, 59);
pub const GREY23: RGB = (59, 59, 59);
pub const GRAY24: RGB = (61, 61, 61);
pub const GREY24: RGB = (61, 61, 61);
pub const GRAY25: RGB = (64, 64, 64);
pub const GREY25: RGB = (64, 64, 64);
pub const GRAY26: RGB = (66, 66, 66);
pub const GREY26: RGB = (66, 66, 66);
pub const GRAY27: RGB = (69, 69, 69);
pub const GREY27: RGB = (69, 69, 69);
pub const GRAY28: RGB = (71, 71, 71);
pub const GREY28: RGB = (71, 71, 71);
pub const GRAY29: RGB = (74, 74, 74);
pub const GREY29: RGB = (74, 74, 74);
pub const GRAY30: RGB = (77, 77, 77);
pub const GREY30: RGB = (77, 77, 77);
pub const GRAY31: RGB = (79, 79, 79);
pub const GREY31: RGB = (79, 79, 79);
pub const GRAY32: RGB = (82, 82, 82);
pub const GREY32: RGB = (82, 82, 82);
pub const GRAY33: RGB = (84, 84, 84);
pub const GREY33: RGB = (84, 84, 84);
pub const GRAY34: RGB = (87, 87, 87);
pub const GREY34: RGB = (87, 87, 87);
pub const GRAY35: RGB = (89, 89, 89);
pub const GREY35: RGB = (89, 89, 89);
pub const GRAY36: RGB = (92, 92, 92);
pub const GREY36: RGB = (92, 92, 92);
pub const GRAY37: RGB = (94, 94, 94);
pub const GREY37: RGB = (94, 94, 94);
pub const GRAY38: RGB = (97, 97, 97);
pub const GREY38: RGB = (97, 97, 97);
pub const GRAY39: RGB = (99, 99, 99);
pub const GREY39: RGB = (99, 99, 99);
pub const GRAY40: RGB = (102, 102, 102);
pub const GREY40: RGB = (102, 102, 102);
pub const GRAY41: RGB = (105, 105, 105);
pub const GREY41: RGB = (105, 105, 105);
pub const GRAY42: RGB = (107, 107, 107);
pub const GREY42: RGB = (107, 107, 107);
pub const GRAY43: RGB = (110, 110, 110);
pub const GREY43: RGB = (110, 110, 110);
pub const GRAY44: RGB = (112, 112, 112);
pub const GREY44: RGB = (112, 112, 112);
pub const GRAY45: RGB = (115, 115, 115);
pub const GREY45: RGB = (115, 115, 115);
pub const GRAY46: RGB = (117, 117, 117);
pub const GREY46: RGB = (117, 117, 117);
pub const GRAY47: RGB = (120, 120, 120);
pub const GREY47: RGB = (120, 120, 120);
pub const GRAY48: RGB = (122, 122, 122);
pub const GREY48: RGB = (122, 122, 122);
pub const GRAY49: RGB = (125, 125, 125);
pub const GREY49: RGB = (125, 125, 125);
pub const GRAY50: RGB = (127, 127, 127);
pub const GREY50: RGB = (127, 127, 127);
pub const GRAY51: RGB = (130, 130, 130);
pub const GREY51: RGB = (130, 130, 130);
pub const GRAY52: RGB = (133, 133, 133);
pub const GREY52: RGB = (133, 133, 133);
pub const GRAY53: RGB = (135, 135, 135);
pub const GREY53: RGB = (135, 135, 135);
pub const GRAY54: RGB = (138, 138, 138);
pub const GREY54: RGB = (138, 138, 138);
pub const GRAY55: RGB = (140, 140, 140);
pub const GREY55: RGB = (140, 140, 140);
pub const GRAY56: RGB = (143, 143, 143);
pub const GREY56: RGB = (143, 143, 143);
pub const GRAY57: RGB = (145, 145, 145);
pub const GREY57: RGB = (145, 145, 145);
pub const GRAY58: RGB = (148, 148, 148);
pub const GREY58: RGB = (148, 148, 148);
pub const GRAY59: RGB = (150, 150, 150);
pub const GREY59: RGB = (150, 150, 150);
pub const GRAY60: RGB = (153, 153, 153);
pub const GREY60: RGB = (153, 153, 153);
pub const GRAY61: RGB = (156, 156, 156);
pub const GREY61: RGB = (156, 156, 156);
pub const GRAY62: RGB = (158, 158, 158);
pub const GREY62: RGB = (158, 158, 158);
pub const GRAY63: RGB = (161, 161, 161);
pub const GREY63: RGB = (161, 161, 161);
pub const GRAY64: RGB = (163, 163, 163);
pub const GREY64: RGB = (163, 163, 163);
pub const GRAY65: RGB = (166, 166, 166);
pub const GREY65: RGB = (166, 166, 166);
pub const GRAY66: RGB = (168, 168, 168);
pub const GREY66: RGB = (168, 168, 168);
pub const GRAY67: RGB = (171, 171, 171);
pub const GREY67: RGB = (171, 171, 171);
pub const GRAY68: RGB = (173, 173, 173);
pub const GREY68: RGB = (173, 173, 173);
pub const GRAY69: RGB = (176, 176, 176);
pub const GREY69: RGB = (176, 176, 176);
pub const GRAY70: RGB = (179, 179, 179);
pub const GREY70: RGB = (179, 179, 179);
pub const GRAY71: RGB = (181, 181, 181);
pub const GREY71: RGB = (181, 181, 181);
pub const GRAY72: RGB = (184, 184, 184);
pub const GREY72: RGB = (184, 184, 184);
pub const GRAY73: RGB = (186, 186, 186);
pub const GREY73: RGB = (186, 186, 186);
pub const GRAY74: RGB = (189, 189, 189);
pub const GREY74: RGB = (189, 189, 189);
pub const GRAY75: RGB = (191, 191, 191);
pub const GREY75: RGB = (191, 191, 191);
pub const GRAY76: RGB = (194, 194, 194);
pub const GREY76: RGB = (194, 194, 194);
pub const GRAY77: RGB = (196, 196, 196);
pub const GREY77: RGB = (196, 196, 196);
pub const GRAY78: RGB = (199, 199, 199);
pub const GREY78: RGB = (199, 199, 199);
pub const GRAY79: RGB = (201, 201, 201);
pub const GREY79: RGB = (201, 201, 201);
pub const GRAY80: RGB = (204, 204, 204);
pub const GREY80: RGB = (204, 204, 204);
pub const GRAY81: RGB = (207, 207, 207);
pub const GREY81: RGB = (207, 207, 207);
pub const GRAY82: RGB = (209, 209, 209);
pub const GREY82: RGB = (209, 209, 209);
pub const GRAY83: RGB = (212, 212, 212);
pub const GREY83: RGB = (212, 212, 212);
pub const GRAY84: RGB = (214, 214, 214);
pub const GREY84: RGB = (214, 214, 214);
pub const GRAY85: RGB = (217, 217, 217);
pub const GREY85: RGB = (217, 217, 217);
pub const GRAY86: RGB = (219, 219, 219);
pub const GREY86: RGB = (219, 219, 219);
pub const GRAY87: RGB = (222, 222, 222);
pub const GREY87: RGB = (222, 222, 222);
pub const GRAY88: RGB = (224, 224, 224);
pub const GREY88: RGB = (224, 224, 224);
pub const GRAY89: RGB = (227, 227, 227);
pub const GREY89: RGB = (227, 227, 227);
pub const GRAY90: RGB = (229, 229, 229);
pub const GREY90: RGB = (229, 229, 229);
pub const GRAY91: RGB = (232, 232, 232);
pub const GREY91: RGB = (232, 232, 232);
pub const GRAY92: RGB = (235, 235, 235);
pub const GREY92: RGB = (235, 235, 235);
pub const GRAY93: RGB = (237, 237, 237);
pub const GREY93: RGB = (237, 237, 237);
pub const GRAY94: RGB = (240, 240, 240);
pub const GREY94: RGB = (240, 240, 240);
pub const GRAY95: RGB = (242, 242, 242);
pub const GREY95: RGB = (242, 242, 242);
pub const GRAY96: RGB = (245, 245, 245);
pub const GREY96: RGB = (245, 245, 245);
pub const GRAY97: RGB = (247, 247, 247);
pub const GREY97: RGB = (247, 247, 247);
pub const GRAY98: RGB = (250, 250, 250);
pub const GREY98: RGB = (250, 250, 250);
pub const GRAY99: RGB = (252, 252, 252);
pub const GREY99: RGB = (252, 252, 252);
pub const GRAY100: RGB = (255, 255, 255);
pub const GREY100: RGB = (255, 255, 255);
pub const DARK_GREY: RGB = (169, 169, 169);
pub const DARKGREY: RGB = (169, 169, 169);
pub const DARK_GRAY: RGB = (169, 169, 169);
pub const DARKGRAY: RGB = (169, 169, 169);
pub const DARK_BLUE: RGB = (0, 0, 139);
pub const DARKBLUE: RGB = (0, 0, 139);
pub const DARK_CYAN: RGB = (0, 139, 139);
pub const DARKCYAN: RGB = (0, 139, 139);
pub const DARK_MAGENTA: RGB = (139, 0, 139);
pub const DARKMAGENTA: RGB = (139, 0, 139);
pub const DARK_RED: RGB = (139, 0, 0);
pub const DARKRED: RGB = (139, 0, 0);
pub const LIGHT_GREEN: RGB = (144, 238, 144);
pub const LIGHTGREEN: RGB = (144, 238, 144);
pub const CRIMSON: RGB = (220, 20, 60);
pub const INDIGO: RGB = (75, 0, 130);
pub const OLIVE: RGB = (128, 128, 0);
pub const REBECCA_PURPLE: RGB = (102, 51, 153);
pub const REBECCAPURPLE: RGB = (102, 51, 153);
pub const SILVER: RGB = (192, 192, 192);
pub const TEAL: RGB = (0, 128, 128);

macro_rules! w3c_color_helper {
    ( $( $n:literal, $name:expr ),* ) => {
        let mut plock = COLORS.lock().unwrap();
        $(
            plock.insert($n.to_string(), $name.into());
        )*
    };
}

// --- Below here was generated with a script ---

/// Insert all named W3C colors into the palette
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::too_many_lines)]
pub fn add_named_colors_to_palette() {
    w3c_color_helper!(
        "none",
        NONE,
        "snow",
        SNOW,
        "ghost_white",
        GHOST_WHITE,
        "ghostwhite",
        GHOSTWHITE,
        "white_smoke",
        WHITE_SMOKE,
        "whitesmoke",
        WHITESMOKE,
        "gainsboro",
        GAINSBORO,
        "floral_white",
        FLORAL_WHITE,
        "floralwhite",
        FLORALWHITE,
        "old_lace",
        OLD_LACE,
        "oldlace",
        OLDLACE,
        "linen",
        LINEN,
        "antique_white",
        ANTIQUE_WHITE,
        "antiquewhite",
        ANTIQUEWHITE,
        "papaya_whip",
        PAPAYA_WHIP,
        "papayawhip",
        PAPAYAWHIP,
        "blanched_almond",
        BLANCHED_ALMOND,
        "blanchedalmond",
        BLANCHEDALMOND,
        "bisque",
        BISQUE,
        "peach_puff",
        PEACH_PUFF,
        "peachpuff",
        PEACHPUFF,
        "navajo_white",
        NAVAJO_WHITE,
        "navajowhite",
        NAVAJOWHITE,
        "moccasin",
        MOCCASIN,
        "cornsilk",
        CORNSILK,
        "ivory",
        IVORY,
        "lemon_chiffon",
        LEMON_CHIFFON,
        "lemonchiffon",
        LEMONCHIFFON,
        "seashell",
        SEASHELL,
        "honeydew",
        HONEYDEW,
        "mint_cream",
        MINT_CREAM,
        "mintcream",
        MINTCREAM,
        "azure",
        AZURE,
        "alice_blue",
        ALICE_BLUE,
        "aliceblue",
        ALICEBLUE,
        "lavender",
        LAVENDER,
        "lavender_blush",
        LAVENDER_BLUSH,
        "lavenderblush",
        LAVENDERBLUSH,
        "misty_rose",
        MISTY_ROSE,
        "mistyrose",
        MISTYROSE,
        "white",
        WHITE,
        "black",
        BLACK,
        "dark_slate",
        DARK_SLATE,
        "darkslategray",
        DARKSLATEGRAY,
        "darkslategrey",
        DARKSLATEGREY,
        "dim_gray",
        DIM_GRAY,
        "dimgray",
        DIMGRAY,
        "dim_grey",
        DIM_GREY,
        "dimgrey",
        DIMGREY,
        "slate_gray",
        SLATE_GRAY,
        "slategray",
        SLATEGRAY,
        "slate_grey",
        SLATE_GREY,
        "slategrey",
        SLATEGREY,
        "light_slate",
        LIGHT_SLATE,
        "lightslategray",
        LIGHTSLATEGRAY,
        "lightslategrey",
        LIGHTSLATEGREY,
        "gray",
        GRAY,
        "grey",
        GREY,
        "x11_gray",
        X11_GRAY,
        "x11gray",
        X11GRAY,
        "x11_grey",
        X11_GREY,
        "x11grey",
        X11GREY,
        "web_gray",
        WEB_GRAY,
        "webgray",
        WEBGRAY,
        "web_grey",
        WEB_GREY,
        "webgrey",
        WEBGREY,
        "light_grey",
        LIGHT_GREY,
        "lightgrey",
        LIGHTGREY,
        "light_gray",
        LIGHT_GRAY,
        "lightgray",
        LIGHTGRAY,
        "midnight_blue",
        MIDNIGHT_BLUE,
        "midnightblue",
        MIDNIGHTBLUE,
        "navy",
        NAVY,
        "navy_blue",
        NAVY_BLUE,
        "navyblue",
        NAVYBLUE,
        "cornflower_blue",
        CORNFLOWER_BLUE,
        "cornflowerblue",
        CORNFLOWERBLUE,
        "darkslateblue",
        DARKSLATEBLUE,
        "slate_blue",
        SLATE_BLUE,
        "slateblue",
        SLATEBLUE,
        "medium_slate",
        MEDIUM_SLATE,
        "mediumslateblue",
        MEDIUMSLATEBLUE,
        "lightslateblue",
        LIGHTSLATEBLUE,
        "medium_blue",
        MEDIUM_BLUE,
        "mediumblue",
        MEDIUMBLUE,
        "royal_blue",
        ROYAL_BLUE,
        "royalblue",
        ROYALBLUE,
        "blue",
        BLUE,
        "dodger_blue",
        DODGER_BLUE,
        "dodgerblue",
        DODGERBLUE,
        "deep_sky",
        DEEP_SKY,
        "deepskyblue",
        DEEPSKYBLUE,
        "sky_blue",
        SKY_BLUE,
        "skyblue",
        SKYBLUE,
        "light_sky",
        LIGHT_SKY,
        "lightskyblue",
        LIGHTSKYBLUE,
        "steel_blue",
        STEEL_BLUE,
        "steelblue",
        STEELBLUE,
        "light_steel",
        LIGHT_STEEL,
        "lightsteelblue",
        LIGHTSTEELBLUE,
        "light_blue",
        LIGHT_BLUE,
        "lightblue",
        LIGHTBLUE,
        "powder_blue",
        POWDER_BLUE,
        "powderblue",
        POWDERBLUE,
        "pale_turquoise",
        PALE_TURQUOISE,
        "paleturquoise",
        PALETURQUOISE,
        "dark_turquoise",
        DARK_TURQUOISE,
        "darkturquoise",
        DARKTURQUOISE,
        "medium_turquoise",
        MEDIUM_TURQUOISE,
        "mediumturquoise",
        MEDIUMTURQUOISE,
        "turquoise",
        TURQUOISE,
        "cyan",
        CYAN,
        "aqua",
        AQUA,
        "light_cyan",
        LIGHT_CYAN,
        "lightcyan",
        LIGHTCYAN,
        "cadet_blue",
        CADET_BLUE,
        "cadetblue",
        CADETBLUE,
        "medium_aquamarine",
        MEDIUM_AQUAMARINE,
        "mediumaquamarine",
        MEDIUMAQUAMARINE,
        "aquamarine",
        AQUAMARINE,
        "dark_green",
        DARK_GREEN,
        "darkgreen",
        DARKGREEN,
        "dark_olive",
        DARK_OLIVE,
        "darkolivegreen",
        DARKOLIVEGREEN,
        "dark_sea",
        DARK_SEA,
        "darkseagreen",
        DARKSEAGREEN,
        "sea_green",
        SEA_GREEN,
        "seagreen",
        SEAGREEN,
        "medium_sea",
        MEDIUM_SEA,
        "mediumseagreen",
        MEDIUMSEAGREEN,
        "light_sea",
        LIGHT_SEA,
        "lightseagreen",
        LIGHTSEAGREEN,
        "pale_green",
        PALE_GREEN,
        "palegreen",
        PALEGREEN,
        "spring_green",
        SPRING_GREEN,
        "springgreen",
        SPRINGGREEN,
        "lawn_green",
        LAWN_GREEN,
        "lawngreen",
        LAWNGREEN,
        "green",
        GREEN,
        "lime",
        LIME,
        "x11_green",
        X11_GREEN,
        "x11green",
        X11GREEN,
        "web_green",
        WEB_GREEN,
        "webgreen",
        WEBGREEN,
        "chartreuse",
        CHARTREUSE,
        "medium_spring",
        MEDIUM_SPRING,
        "mediumspringgreen",
        MEDIUMSPRINGGREEN,
        "green_yellow",
        GREEN_YELLOW,
        "greenyellow",
        GREENYELLOW,
        "lime_green",
        LIME_GREEN,
        "limegreen",
        LIMEGREEN,
        "yellow_green",
        YELLOW_GREEN,
        "yellowgreen",
        YELLOWGREEN,
        "forest_green",
        FOREST_GREEN,
        "forestgreen",
        FORESTGREEN,
        "olive_drab",
        OLIVE_DRAB,
        "olivedrab",
        OLIVEDRAB,
        "dark_khaki",
        DARK_KHAKI,
        "darkkhaki",
        DARKKHAKI,
        "khaki",
        KHAKI,
        "pale_goldenrod",
        PALE_GOLDENROD,
        "palegoldenrod",
        PALEGOLDENROD,
        "light_goldenrod",
        LIGHT_GOLDENROD,
        "lightgoldenrodyellow",
        LIGHTGOLDENRODYELLOW,
        "light_yellow",
        LIGHT_YELLOW,
        "lightyellow",
        LIGHTYELLOW,
        "yellow",
        YELLOW,
        "gold",
        GOLD,
        "lightgoldenrod",
        LIGHTGOLDENROD,
        "goldenrod",
        GOLDENROD,
        "dark_goldenrod",
        DARK_GOLDENROD,
        "darkgoldenrod",
        DARKGOLDENROD,
        "rosy_brown",
        ROSY_BROWN,
        "rosybrown",
        ROSYBROWN,
        "indian_red",
        INDIAN_RED,
        "indianred",
        INDIANRED,
        "saddle_brown",
        SADDLE_BROWN,
        "saddlebrown",
        SADDLEBROWN,
        "sienna",
        SIENNA,
        "peru",
        PERU,
        "burlywood",
        BURLYWOOD,
        "beige",
        BEIGE,
        "wheat",
        WHEAT,
        "sandy_brown",
        SANDY_BROWN,
        "sandybrown",
        SANDYBROWN,
        "tan",
        TAN,
        "chocolate",
        CHOCOLATE,
        "firebrick_34",
        FIREBRICK_34,
        "brown_42",
        BROWN_42,
        "dark_salmon",
        DARK_SALMON,
        "darksalmon",
        DARKSALMON,
        "salmon",
        SALMON,
        "light_salmon",
        LIGHT_SALMON,
        "lightsalmon",
        LIGHTSALMON,
        "orange",
        ORANGE,
        "dark_orange",
        DARK_ORANGE,
        "darkorange",
        DARKORANGE,
        "coral",
        CORAL,
        "light_coral",
        LIGHT_CORAL,
        "lightcoral",
        LIGHTCORAL,
        "tomato",
        TOMATO,
        "orange_red",
        ORANGE_RED,
        "orangered",
        ORANGERED,
        "red",
        RED,
        "hot_pink",
        HOT_PINK,
        "hotpink",
        HOTPINK,
        "deep_pink",
        DEEP_PINK,
        "deeppink",
        DEEPPINK,
        "pink",
        PINK,
        "light_pink",
        LIGHT_PINK,
        "lightpink",
        LIGHTPINK,
        "pale_violet",
        PALE_VIOLET,
        "palevioletred",
        PALEVIOLETRED,
        "maroon",
        MAROON,
        "x11_maroon",
        X11_MAROON,
        "x11maroon",
        X11MAROON,
        "web_maroon",
        WEB_MAROON,
        "webmaroon",
        WEBMAROON,
        "medium_violet",
        MEDIUM_VIOLET,
        "mediumvioletred",
        MEDIUMVIOLETRED,
        "violet_red",
        VIOLET_RED,
        "violetred",
        VIOLETRED,
        "magenta",
        MAGENTA,
        "fuchsia",
        FUCHSIA,
        "violet",
        VIOLET,
        "plum",
        PLUM,
        "orchid",
        ORCHID,
        "medium_orchid",
        MEDIUM_ORCHID,
        "mediumorchid",
        MEDIUMORCHID,
        "dark_orchid",
        DARK_ORCHID,
        "darkorchid",
        DARKORCHID,
        "dark_violet",
        DARK_VIOLET,
        "darkviolet",
        DARKVIOLET,
        "blue_violet",
        BLUE_VIOLET,
        "blueviolet",
        BLUEVIOLET,
        "purple",
        PURPLE,
        "x11_purple",
        X11_PURPLE,
        "x11purple",
        X11PURPLE,
        "web_purple",
        WEB_PURPLE,
        "webpurple",
        WEBPURPLE,
        "medium_purple",
        MEDIUM_PURPLE,
        "mediumpurple",
        MEDIUMPURPLE,
        "thistle",
        THISTLE,
        "snow1",
        SNOW1,
        "snow2",
        SNOW2,
        "snow3",
        SNOW3,
        "snow4",
        SNOW4,
        "seashell1",
        SEASHELL1,
        "seashell2",
        SEASHELL2,
        "seashell3",
        SEASHELL3,
        "seashell4",
        SEASHELL4,
        "antiquewhite1",
        ANTIQUEWHITE1,
        "antiquewhite2",
        ANTIQUEWHITE2,
        "antiquewhite3",
        ANTIQUEWHITE3,
        "antiquewhite4",
        ANTIQUEWHITE4,
        "bisque1",
        BISQUE1,
        "bisque2",
        BISQUE2,
        "bisque3",
        BISQUE3,
        "bisque4",
        BISQUE4,
        "peachpuff1",
        PEACHPUFF1,
        "peachpuff2",
        PEACHPUFF2,
        "peachpuff3",
        PEACHPUFF3,
        "peachpuff4",
        PEACHPUFF4,
        "navajowhite1",
        NAVAJOWHITE1,
        "navajowhite2",
        NAVAJOWHITE2,
        "navajowhite3",
        NAVAJOWHITE3,
        "navajowhite4",
        NAVAJOWHITE4,
        "lemonchiffon1",
        LEMONCHIFFON1,
        "lemonchiffon2",
        LEMONCHIFFON2,
        "lemonchiffon3",
        LEMONCHIFFON3,
        "lemonchiffon4",
        LEMONCHIFFON4,
        "cornsilk1",
        CORNSILK1,
        "cornsilk2",
        CORNSILK2,
        "cornsilk3",
        CORNSILK3,
        "cornsilk4",
        CORNSILK4,
        "ivory1",
        IVORY1,
        "ivory2",
        IVORY2,
        "ivory3",
        IVORY3,
        "ivory4",
        IVORY4,
        "honeydew1",
        HONEYDEW1,
        "honeydew2",
        HONEYDEW2,
        "honeydew3",
        HONEYDEW3,
        "honeydew4",
        HONEYDEW4,
        "lavenderblush1",
        LAVENDERBLUSH1,
        "lavenderblush2",
        LAVENDERBLUSH2,
        "lavenderblush3",
        LAVENDERBLUSH3,
        "lavenderblush4",
        LAVENDERBLUSH4,
        "mistyrose1",
        MISTYROSE1,
        "mistyrose2",
        MISTYROSE2,
        "mistyrose3",
        MISTYROSE3,
        "mistyrose4",
        MISTYROSE4,
        "azure1",
        AZURE1,
        "azure2",
        AZURE2,
        "azure3",
        AZURE3,
        "azure4",
        AZURE4,
        "slateblue1",
        SLATEBLUE1,
        "slateblue2",
        SLATEBLUE2,
        "slateblue3",
        SLATEBLUE3,
        "slateblue4",
        SLATEBLUE4,
        "royalblue1",
        ROYALBLUE1,
        "royalblue2",
        ROYALBLUE2,
        "royalblue3",
        ROYALBLUE3,
        "royalblue4",
        ROYALBLUE4,
        "blue1",
        BLUE1,
        "blue2",
        BLUE2,
        "blue3",
        BLUE3,
        "blue4",
        BLUE4,
        "dodgerblue1",
        DODGERBLUE1,
        "dodgerblue2",
        DODGERBLUE2,
        "dodgerblue3",
        DODGERBLUE3,
        "dodgerblue4",
        DODGERBLUE4,
        "steelblue1",
        STEELBLUE1,
        "steelblue2",
        STEELBLUE2,
        "steelblue3",
        STEELBLUE3,
        "steelblue4",
        STEELBLUE4,
        "deepskyblue1",
        DEEPSKYBLUE1,
        "deepskyblue2",
        DEEPSKYBLUE2,
        "deepskyblue3",
        DEEPSKYBLUE3,
        "deepskyblue4",
        DEEPSKYBLUE4,
        "skyblue1",
        SKYBLUE1,
        "skyblue2",
        SKYBLUE2,
        "skyblue3",
        SKYBLUE3,
        "skyblue4",
        SKYBLUE4,
        "lightskyblue1",
        LIGHTSKYBLUE1,
        "lightskyblue2",
        LIGHTSKYBLUE2,
        "lightskyblue3",
        LIGHTSKYBLUE3,
        "lightskyblue4",
        LIGHTSKYBLUE4,
        "slategray1",
        SLATEGRAY1,
        "slategray2",
        SLATEGRAY2,
        "slategray3",
        SLATEGRAY3,
        "slategray4",
        SLATEGRAY4,
        "lightsteelblue1",
        LIGHTSTEELBLUE1,
        "lightsteelblue2",
        LIGHTSTEELBLUE2,
        "lightsteelblue3",
        LIGHTSTEELBLUE3,
        "lightsteelblue4",
        LIGHTSTEELBLUE4,
        "lightblue1",
        LIGHTBLUE1,
        "lightblue2",
        LIGHTBLUE2,
        "lightblue3",
        LIGHTBLUE3,
        "lightblue4",
        LIGHTBLUE4,
        "lightcyan1",
        LIGHTCYAN1,
        "lightcyan2",
        LIGHTCYAN2,
        "lightcyan3",
        LIGHTCYAN3,
        "lightcyan4",
        LIGHTCYAN4,
        "paleturquoise1",
        PALETURQUOISE1,
        "paleturquoise2",
        PALETURQUOISE2,
        "paleturquoise3",
        PALETURQUOISE3,
        "paleturquoise4",
        PALETURQUOISE4,
        "cadetblue1",
        CADETBLUE1,
        "cadetblue2",
        CADETBLUE2,
        "cadetblue3",
        CADETBLUE3,
        "cadetblue4",
        CADETBLUE4,
        "turquoise1",
        TURQUOISE1,
        "turquoise2",
        TURQUOISE2,
        "turquoise3",
        TURQUOISE3,
        "turquoise4",
        TURQUOISE4,
        "cyan1",
        CYAN1,
        "cyan2",
        CYAN2,
        "cyan3",
        CYAN3,
        "cyan4",
        CYAN4,
        "darkslategray1",
        DARKSLATEGRAY1,
        "darkslategray2",
        DARKSLATEGRAY2,
        "darkslategray3",
        DARKSLATEGRAY3,
        "darkslategray4",
        DARKSLATEGRAY4,
        "aquamarine1",
        AQUAMARINE1,
        "aquamarine2",
        AQUAMARINE2,
        "aquamarine3",
        AQUAMARINE3,
        "aquamarine4",
        AQUAMARINE4,
        "darkseagreen1",
        DARKSEAGREEN1,
        "darkseagreen2",
        DARKSEAGREEN2,
        "darkseagreen3",
        DARKSEAGREEN3,
        "darkseagreen4",
        DARKSEAGREEN4,
        "seagreen1",
        SEAGREEN1,
        "seagreen2",
        SEAGREEN2,
        "seagreen3",
        SEAGREEN3,
        "seagreen4",
        SEAGREEN4,
        "palegreen1",
        PALEGREEN1,
        "palegreen2",
        PALEGREEN2,
        "palegreen3",
        PALEGREEN3,
        "palegreen4",
        PALEGREEN4,
        "springgreen1",
        SPRINGGREEN1,
        "springgreen2",
        SPRINGGREEN2,
        "springgreen3",
        SPRINGGREEN3,
        "springgreen4",
        SPRINGGREEN4,
        "green1",
        GREEN1,
        "green2",
        GREEN2,
        "green3",
        GREEN3,
        "green4",
        GREEN4,
        "chartreuse1",
        CHARTREUSE1,
        "chartreuse2",
        CHARTREUSE2,
        "chartreuse3",
        CHARTREUSE3,
        "chartreuse4",
        CHARTREUSE4,
        "olivedrab1",
        OLIVEDRAB1,
        "olivedrab2",
        OLIVEDRAB2,
        "olivedrab3",
        OLIVEDRAB3,
        "olivedrab4",
        OLIVEDRAB4,
        "darkolivegreen1",
        DARKOLIVEGREEN1,
        "darkolivegreen2",
        DARKOLIVEGREEN2,
        "darkolivegreen3",
        DARKOLIVEGREEN3,
        "darkolivegreen4",
        DARKOLIVEGREEN4,
        "khaki1",
        KHAKI1,
        "khaki2",
        KHAKI2,
        "khaki3",
        KHAKI3,
        "khaki4",
        KHAKI4,
        "lightgoldenrod1",
        LIGHTGOLDENROD1,
        "lightgoldenrod2",
        LIGHTGOLDENROD2,
        "lightgoldenrod3",
        LIGHTGOLDENROD3,
        "lightgoldenrod4",
        LIGHTGOLDENROD4,
        "lightyellow1",
        LIGHTYELLOW1,
        "lightyellow2",
        LIGHTYELLOW2,
        "lightyellow3",
        LIGHTYELLOW3,
        "lightyellow4",
        LIGHTYELLOW4,
        "yellow1",
        YELLOW1,
        "yellow2",
        YELLOW2,
        "yellow3",
        YELLOW3,
        "yellow4",
        YELLOW4,
        "gold1",
        GOLD1,
        "gold2",
        GOLD2,
        "gold3",
        GOLD3,
        "gold4",
        GOLD4,
        "goldenrod1",
        GOLDENROD1,
        "goldenrod2",
        GOLDENROD2,
        "goldenrod3",
        GOLDENROD3,
        "goldenrod4",
        GOLDENROD4,
        "darkgoldenrod1",
        DARKGOLDENROD1,
        "darkgoldenrod2",
        DARKGOLDENROD2,
        "darkgoldenrod3",
        DARKGOLDENROD3,
        "darkgoldenrod4",
        DARKGOLDENROD4,
        "rosybrown1",
        ROSYBROWN1,
        "rosybrown2",
        ROSYBROWN2,
        "rosybrown3",
        ROSYBROWN3,
        "rosybrown4",
        ROSYBROWN4,
        "indianred1",
        INDIANRED1,
        "indianred2",
        INDIANRED2,
        "indianred3",
        INDIANRED3,
        "indianred4",
        INDIANRED4,
        "sienna1",
        SIENNA1,
        "sienna2",
        SIENNA2,
        "sienna3",
        SIENNA3,
        "sienna4",
        SIENNA4,
        "burlywood1",
        BURLYWOOD1,
        "burlywood2",
        BURLYWOOD2,
        "burlywood3",
        BURLYWOOD3,
        "burlywood4",
        BURLYWOOD4,
        "wheat1",
        WHEAT1,
        "wheat2",
        WHEAT2,
        "wheat3",
        WHEAT3,
        "wheat4",
        WHEAT4,
        "tan1",
        TAN1,
        "tan2",
        TAN2,
        "tan3",
        TAN3,
        "tan4",
        TAN4,
        "chocolate1",
        CHOCOLATE1,
        "chocolate2",
        CHOCOLATE2,
        "chocolate3",
        CHOCOLATE3,
        "chocolate4",
        CHOCOLATE4,
        "firebrick1",
        FIREBRICK1,
        "firebrick2",
        FIREBRICK2,
        "firebrick3",
        FIREBRICK3,
        "firebrick4",
        FIREBRICK4,
        "brown1",
        BROWN1,
        "brown2",
        BROWN2,
        "brown3",
        BROWN3,
        "brown4",
        BROWN4,
        "salmon1",
        SALMON1,
        "salmon2",
        SALMON2,
        "salmon3",
        SALMON3,
        "salmon4",
        SALMON4,
        "lightsalmon1",
        LIGHTSALMON1,
        "lightsalmon2",
        LIGHTSALMON2,
        "lightsalmon3",
        LIGHTSALMON3,
        "lightsalmon4",
        LIGHTSALMON4,
        "orange1",
        ORANGE1,
        "orange2",
        ORANGE2,
        "orange3",
        ORANGE3,
        "orange4",
        ORANGE4,
        "darkorange1",
        DARKORANGE1,
        "darkorange2",
        DARKORANGE2,
        "darkorange3",
        DARKORANGE3,
        "darkorange4",
        DARKORANGE4,
        "coral1",
        CORAL1,
        "coral2",
        CORAL2,
        "coral3",
        CORAL3,
        "coral4",
        CORAL4,
        "tomato1",
        TOMATO1,
        "tomato2",
        TOMATO2,
        "tomato3",
        TOMATO3,
        "tomato4",
        TOMATO4,
        "orangered1",
        ORANGERED1,
        "orangered2",
        ORANGERED2,
        "orangered3",
        ORANGERED3,
        "orangered4",
        ORANGERED4,
        "red1",
        RED1,
        "red2",
        RED2,
        "red3",
        RED3,
        "red4",
        RED4,
        "deeppink1",
        DEEPPINK1,
        "deeppink2",
        DEEPPINK2,
        "deeppink3",
        DEEPPINK3,
        "deeppink4",
        DEEPPINK4,
        "hotpink1",
        HOTPINK1,
        "hotpink2",
        HOTPINK2,
        "hotpink3",
        HOTPINK3,
        "hotpink4",
        HOTPINK4,
        "pink1",
        PINK1,
        "pink2",
        PINK2,
        "pink3",
        PINK3,
        "pink4",
        PINK4,
        "lightpink1",
        LIGHTPINK1,
        "lightpink2",
        LIGHTPINK2,
        "lightpink3",
        LIGHTPINK3,
        "lightpink4",
        LIGHTPINK4,
        "palevioletred1",
        PALEVIOLETRED1,
        "palevioletred2",
        PALEVIOLETRED2,
        "palevioletred3",
        PALEVIOLETRED3,
        "palevioletred4",
        PALEVIOLETRED4,
        "maroon1",
        MAROON1,
        "maroon2",
        MAROON2,
        "maroon3",
        MAROON3,
        "maroon4",
        MAROON4,
        "violetred1",
        VIOLETRED1,
        "violetred2",
        VIOLETRED2,
        "violetred3",
        VIOLETRED3,
        "violetred4",
        VIOLETRED4,
        "magenta1",
        MAGENTA1,
        "magenta2",
        MAGENTA2,
        "magenta3",
        MAGENTA3,
        "magenta4",
        MAGENTA4,
        "orchid1",
        ORCHID1,
        "orchid2",
        ORCHID2,
        "orchid3",
        ORCHID3,
        "orchid4",
        ORCHID4,
        "plum1",
        PLUM1,
        "plum2",
        PLUM2,
        "plum3",
        PLUM3,
        "plum4",
        PLUM4,
        "mediumorchid1",
        MEDIUMORCHID1,
        "mediumorchid2",
        MEDIUMORCHID2,
        "mediumorchid3",
        MEDIUMORCHID3,
        "mediumorchid4",
        MEDIUMORCHID4,
        "darkorchid1",
        DARKORCHID1,
        "darkorchid2",
        DARKORCHID2,
        "darkorchid3",
        DARKORCHID3,
        "darkorchid4",
        DARKORCHID4,
        "purple1",
        PURPLE1,
        "purple2",
        PURPLE2,
        "purple3",
        PURPLE3,
        "purple4",
        PURPLE4,
        "mediumpurple1",
        MEDIUMPURPLE1,
        "mediumpurple2",
        MEDIUMPURPLE2,
        "mediumpurple3",
        MEDIUMPURPLE3,
        "mediumpurple4",
        MEDIUMPURPLE4,
        "thistle1",
        THISTLE1,
        "thistle2",
        THISTLE2,
        "thistle3",
        THISTLE3,
        "thistle4",
        THISTLE4,
        "gray0",
        GRAY0,
        "grey0",
        GREY0,
        "gray1",
        GRAY1,
        "grey1",
        GREY1,
        "gray2",
        GRAY2,
        "grey2",
        GREY2,
        "gray3",
        GRAY3,
        "grey3",
        GREY3,
        "gray4",
        GRAY4,
        "grey4",
        GREY4,
        "gray5",
        GRAY5,
        "grey5",
        GREY5,
        "gray6",
        GRAY6,
        "grey6",
        GREY6,
        "gray7",
        GRAY7,
        "grey7",
        GREY7,
        "gray8",
        GRAY8,
        "grey8",
        GREY8,
        "gray9",
        GRAY9,
        "grey9",
        GREY9,
        "gray10",
        GRAY10,
        "grey10",
        GREY10,
        "gray11",
        GRAY11,
        "grey11",
        GREY11,
        "gray12",
        GRAY12,
        "grey12",
        GREY12,
        "gray13",
        GRAY13,
        "grey13",
        GREY13,
        "gray14",
        GRAY14,
        "grey14",
        GREY14,
        "gray15",
        GRAY15,
        "grey15",
        GREY15,
        "gray16",
        GRAY16,
        "grey16",
        GREY16,
        "gray17",
        GRAY17,
        "grey17",
        GREY17,
        "gray18",
        GRAY18,
        "grey18",
        GREY18,
        "gray19",
        GRAY19,
        "grey19",
        GREY19,
        "gray20",
        GRAY20,
        "grey20",
        GREY20,
        "gray21",
        GRAY21,
        "grey21",
        GREY21,
        "gray22",
        GRAY22,
        "grey22",
        GREY22,
        "gray23",
        GRAY23,
        "grey23",
        GREY23,
        "gray24",
        GRAY24,
        "grey24",
        GREY24,
        "gray25",
        GRAY25,
        "grey25",
        GREY25,
        "gray26",
        GRAY26,
        "grey26",
        GREY26,
        "gray27",
        GRAY27,
        "grey27",
        GREY27,
        "gray28",
        GRAY28,
        "grey28",
        GREY28,
        "gray29",
        GRAY29,
        "grey29",
        GREY29,
        "gray30",
        GRAY30,
        "grey30",
        GREY30,
        "gray31",
        GRAY31,
        "grey31",
        GREY31,
        "gray32",
        GRAY32,
        "grey32",
        GREY32,
        "gray33",
        GRAY33,
        "grey33",
        GREY33,
        "gray34",
        GRAY34,
        "grey34",
        GREY34,
        "gray35",
        GRAY35,
        "grey35",
        GREY35,
        "gray36",
        GRAY36,
        "grey36",
        GREY36,
        "gray37",
        GRAY37,
        "grey37",
        GREY37,
        "gray38",
        GRAY38,
        "grey38",
        GREY38,
        "gray39",
        GRAY39,
        "grey39",
        GREY39,
        "gray40",
        GRAY40,
        "grey40",
        GREY40,
        "gray41",
        GRAY41,
        "grey41",
        GREY41,
        "gray42",
        GRAY42,
        "grey42",
        GREY42,
        "gray43",
        GRAY43,
        "grey43",
        GREY43,
        "gray44",
        GRAY44,
        "grey44",
        GREY44,
        "gray45",
        GRAY45,
        "grey45",
        GREY45,
        "gray46",
        GRAY46,
        "grey46",
        GREY46,
        "gray47",
        GRAY47,
        "grey47",
        GREY47,
        "gray48",
        GRAY48,
        "grey48",
        GREY48,
        "gray49",
        GRAY49,
        "grey49",
        GREY49,
        "gray50",
        GRAY50,
        "grey50",
        GREY50,
        "gray51",
        GRAY51,
        "grey51",
        GREY51,
        "gray52",
        GRAY52,
        "grey52",
        GREY52,
        "gray53",
        GRAY53,
        "grey53",
        GREY53,
        "gray54",
        GRAY54,
        "grey54",
        GREY54,
        "gray55",
        GRAY55,
        "grey55",
        GREY55,
        "gray56",
        GRAY56,
        "grey56",
        GREY56,
        "gray57",
        GRAY57,
        "grey57",
        GREY57,
        "gray58",
        GRAY58,
        "grey58",
        GREY58,
        "gray59",
        GRAY59,
        "grey59",
        GREY59,
        "gray60",
        GRAY60,
        "grey60",
        GREY60,
        "gray61",
        GRAY61,
        "grey61",
        GREY61,
        "gray62",
        GRAY62,
        "grey62",
        GREY62,
        "gray63",
        GRAY63,
        "grey63",
        GREY63,
        "gray64",
        GRAY64,
        "grey64",
        GREY64,
        "gray65",
        GRAY65,
        "grey65",
        GREY65,
        "gray66",
        GRAY66,
        "grey66",
        GREY66,
        "gray67",
        GRAY67,
        "grey67",
        GREY67,
        "gray68",
        GRAY68,
        "grey68",
        GREY68,
        "gray69",
        GRAY69,
        "grey69",
        GREY69,
        "gray70",
        GRAY70,
        "grey70",
        GREY70,
        "gray71",
        GRAY71,
        "grey71",
        GREY71,
        "gray72",
        GRAY72,
        "grey72",
        GREY72,
        "gray73",
        GRAY73,
        "grey73",
        GREY73,
        "gray74",
        GRAY74,
        "grey74",
        GREY74,
        "gray75",
        GRAY75,
        "grey75",
        GREY75,
        "gray76",
        GRAY76,
        "grey76",
        GREY76,
        "gray77",
        GRAY77,
        "grey77",
        GREY77,
        "gray78",
        GRAY78,
        "grey78",
        GREY78,
        "gray79",
        GRAY79,
        "grey79",
        GREY79,
        "gray80",
        GRAY80,
        "grey80",
        GREY80,
        "gray81",
        GRAY81,
        "grey81",
        GREY81,
        "gray82",
        GRAY82,
        "grey82",
        GREY82,
        "gray83",
        GRAY83,
        "grey83",
        GREY83,
        "gray84",
        GRAY84,
        "grey84",
        GREY84,
        "gray85",
        GRAY85,
        "grey85",
        GREY85,
        "gray86",
        GRAY86,
        "grey86",
        GREY86,
        "gray87",
        GRAY87,
        "grey87",
        GREY87,
        "gray88",
        GRAY88,
        "grey88",
        GREY88,
        "gray89",
        GRAY89,
        "grey89",
        GREY89,
        "gray90",
        GRAY90,
        "grey90",
        GREY90,
        "gray91",
        GRAY91,
        "grey91",
        GREY91,
        "gray92",
        GRAY92,
        "grey92",
        GREY92,
        "gray93",
        GRAY93,
        "grey93",
        GREY93,
        "gray94",
        GRAY94,
        "grey94",
        GREY94,
        "gray95",
        GRAY95,
        "grey95",
        GREY95,
        "gray96",
        GRAY96,
        "grey96",
        GREY96,
        "gray97",
        GRAY97,
        "grey97",
        GREY97,
        "gray98",
        GRAY98,
        "grey98",
        GREY98,
        "gray99",
        GRAY99,
        "grey99",
        GREY99,
        "gray100",
        GRAY100,
        "grey100",
        GREY100,
        "dark_grey",
        DARK_GREY,
        "darkgrey",
        DARKGREY,
        "dark_gray",
        DARK_GRAY,
        "darkgray",
        DARKGRAY,
        "dark_blue",
        DARK_BLUE,
        "darkblue",
        DARKBLUE,
        "dark_cyan",
        DARK_CYAN,
        "darkcyan",
        DARKCYAN,
        "dark_magenta",
        DARK_MAGENTA,
        "darkmagenta",
        DARKMAGENTA,
        "dark_red",
        DARK_RED,
        "darkred",
        DARKRED,
        "light_green",
        LIGHT_GREEN,
        "lightgreen",
        LIGHTGREEN,
        "crimson",
        CRIMSON,
        "indigo",
        INDIGO,
        "olive",
        OLIVE,
        "rebecca_purple",
        REBECCA_PURPLE,
        "rebeccapurple",
        REBECCAPURPLE,
        "silver",
        SILVER,
        "teal",
        TEAL
    );
}
