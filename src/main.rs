mod dracula;
use dracula::{hex, rgb};
use std::io;
use std::fmt::{self,Formatter,Display};

#[derive(Debug)]
struct RGB {
    rd: u8,
    gr: u8,
    bl: u8,
}

const HEX_array: [&str; 11] = [     hex::BG, 
                                    hex::CL,
                                    hex::FG,
                                    hex::CM,
                                    hex::CY,
                                    hex::GR,
                                    hex::OR,
                                    hex::PN,
                                    hex::PR,
                                    hex::RD,
                                    hex::YL,
];

const RGB_array: [[u8;3]; 11] = [   rgb::BG, 
                                    rgb::CL,
                                    rgb::FG,
                                    rgb::CM,
                                    rgb::CY,
                                    rgb::GR,
                                    rgb::OR,
                                    rgb::PN,
                                    rgb::PR,
                                    rgb::RD,
                                    rgb::YL,
];

fn getType() -> bool {
    let mut H : bool = false;
    let mut color_type = String::new();
    println!("Enter number for color format (0 - RGB) (1 - HEX)");  
    io::stdin().read_line(&mut color_type).expect("Your std is infected");
    let T = color_type.chars().nth(0).unwrap();
    if T == '0' {
        H = false;
    } else if T == '1' {
        H = true;
    } else {
        println!("{} is not an accepted input", color_type);
    }
    return H; }
fn getHex() -> RGB { 
    

    // Will need to do some null return checking
    let mut rd : u8 = 0x0;
    let mut gr : u8 = 0x0;
    let mut bl : u8 = 0x0;

    let mut init_value = String::new();
    println!("Enter your hex value without the '#' (IE: #FFFFFF would be FFFFFF)");  
    io::stdin().read_line(&mut init_value).expect("Your std is infected");
    //let hex_vec : Vec<char> = init_value.chars().collect();
    //println!("{:?}", hex_vec);
    rd = u8::from_str_radix(&init_value[0..2],16).unwrap();
    gr = u8::from_str_radix(&init_value[2..4],16).unwrap();
    bl = u8::from_str_radix(&init_value[4..6],16).unwrap();
    let RGB_value: RGB = RGB{rd,gr,bl};
    return RGB_value;
}
fn getRGB() -> RGB {
    let mut rd : u8;
    let mut gr : u8;
    let mut bl : u8;


    let mut input_str = String::new();
    println!("Separate your RGB with Spaces (IE: 255 255 255)");  
    io::stdin().read_line(&mut input_str).expect("Your std is infected");

    let parsed = input_str.split(" ").collect::<Vec<_>>();
    // Var at [0] is reallocated to rd  
    rd = parsed[0].parse().unwrap();
    gr = parsed[0].parse().unwrap();
    bl = parsed[0].parse().unwrap();

    let RGB_value: RGB = RGB{rd,gr,bl};
    return RGB_value;
     
    }

fn checkSim(RGB_Val : RGB) -> RGB {
    let bl = RGB_Val.bl;
    let gr = RGB_Val.gr;
    let rd = RGB_Val.rd;
    return RGB_Val; 
    /* 
     * Will need to parse each rdg value into 3 array representing the dracula values for each one 
     * Then compare the rgb values to them storing a sep array with how much they are sep ie 12
     * and 10 == 2 whereas 12-6 = 6. 
     *
     * Loop through each one comparing the index of the biggest sep
     * If not we go through and check the diffrence for each one to find the semi closest one
     *
     */ 

}
    
fn main() {
    let mut out_HEX = String::new();
    let mut out_RBG = String::new();
    let is_hex = getType();
    if is_hex == true {
       let init_hex = getHex();
       println!("{:#?}", init_hex);
    } else if is_hex == false {
        let init_RGB = getRGB();
       println!("{:#?}", init_RGB);
    }
}
