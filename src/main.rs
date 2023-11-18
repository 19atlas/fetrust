pub mod resource;

use another_json_minimal::*;
use ansi_term::Colour::*;
use std::fs;
use std::path::Path;
use tinyrand::Rand;
use tinyrand_std::thread_rand;

pub fn get_banner(bannered: String) -> String {
    let config_file = format!("{}/.config/fetrust/{}", env!("HOME"), "font.flf");
    if !Path::new(&config_file).exists() {
        println!("Creating default font ({})", config_file);
        if fs::create_dir_all(format!("{}/.config/fetrust", env!("HOME"))).is_err() {
            println!(
                "Error: Something happened wrong (creating {})",
                format!("{}/.config/", env!("HOME"))
            )
        }
        //http://www.figlet.org/fonts/smslant.flf
        if fs::write(&config_file,"flf2a$ 5 4 14 15 10 0 22415\nSmSlant by Glenn Chappell 6/93 - based on Small & Slant\nIncludes ISO Latin-1\nfiglet release 2.1 -- 12 Aug 1994\nPermission is hereby given to modify this font, as long as the\nmodifier's name is placed on a comment line.\n\nModified by Paul Burton <solution@earthlink.net> 12/96 to include new parameter\nsupported by FIGlet and FIGWin.  May also be slightly modified for better use\nof new full-width/kern/smush alternatives, but default output is NOT changed.\n\n    $@\n   $ @\n  $  @\n $   @\n$    @@\n   __@\n  / /@\n /_/ @\n(_)  @\n     @@\n _ _ @\n( | )@\n|/|/ @\n$    @\n     @@\n     ____ @\n  __/ / /_@\n /_  . __/@\n/_    __/ @\n /_/_/    @@\n     @\n  _//@\n (_-<@\n/ __/@\n//   @@\n _   __@\n(_)_/_/@\n _/_/_ @\n/_/ (_)@\n       @@\n  ____   @\n / __/___@\n > _/_ _/@\n|_____/  @\n         @@\n _ @\n( )@\n|/ @\n$  @\n   @@\n    __@\n  _/_/@\n / /  @\n/ /   @\n|_|   @@\n    _ @\n   | |@\n   / /@\n _/_/ @\n/_/   @@\n    @\n _/|@\n> _<@\n|/  @\n    @@\n    __ @\n __/ /_@\n/_  __/@\n /_/   @\n       @@\n   @\n   @\n _ @\n( )@\n|/ @@\n     @\n ____@\n/___/@\n $   @\n     @@\n   @\n   @\n _ @\n(_)@\n   @@\n     __@\n   _/_/@\n _/_/  @\n/_/    @\n       @@\n  ___ @\n / _ \\@\n/ // /@\n\\___/ @\n      @@\n  ___@\n <  /@\n / / @\n/_/  @\n     @@\n   ___ @\n  |_  |@\n / __/ @\n/____/ @\n       @@\n   ____@\n  |_  /@\n _/_ < @\n/____/ @\n       @@\n  ____@\n / / /@\n/_  _/@\n /_/  @\n      @@\n   ____@\n  / __/@\n /__ \\ @\n/____/ @\n       @@\n  ____@\n / __/@\n/ _ \\ @\n\\___/ @\n      @@\n ____@\n/_  /@\n / / @\n/_/  @\n     @@\n  ___ @\n ( _ )@\n/ _  |@\n\\___/ @\n      @@\n  ___ @\n / _ \\@\n \\_, /@\n/___/ @\n      @@\n   _ @\n  (_)@\n _   @\n(_)  @\n     @@\n   _ @\n  (_)@\n _   @\n( )  @\n|/   @@\n  __@\n / /@\n< < @\n \\_\\@\n    @@\n      @\n  ____@\n /___/@\n/___/ @\n      @@\n__  @\n\\ \\ @\n > >@\n/_/ @\n    @@\n ___ @\n/__ \\@\n /__/@\n(_)  @\n     @@\n  _____ @\n / ___ \\@\n/ / _ `/@\n\\ \\_,_/ @\n \\___/  @@\n   ___ @\n  / _ |@\n / __ |@\n/_/ |_|@\n       @@\n   ___ @\n  / _ )@\n / _  |@\n/____/ @\n       @@\n  _____@\n / ___/@\n/ /__  @\n\\___/  @\n       @@\n   ___ @\n  / _ \\@\n / // /@\n/____/ @\n       @@\n   ____@\n  / __/@\n / _/  @\n/___/  @\n       @@\n   ____@\n  / __/@\n / _/  @\n/_/    @\n       @@\n  _____@\n / ___/@\n/ (_ / @\n\\___/  @\n       @@\n   __ __@\n  / // /@\n / _  / @\n/_//_/  @\n        @@\n   ____@\n  /  _/@\n _/ /  @\n/___/  @\n       @@\n     __@\n __ / /@\n/ // / @\n\\___/  @\n       @@\n   __ __@\n  / //_/@\n / ,<   @\n/_/|_|  @\n        @@\n   __ @\n  / / @\n / /__@\n/____/@\n      @@\n   __  ___@\n  /  |/  /@\n / /|_/ / @\n/_/  /_/  @\n          @@\n   _  __@\n  / |/ /@\n /    / @\n/_/|_/  @\n        @@\n  ____ @\n / __ \\@\n/ /_/ /@\n\\____/ @\n       @@\n   ___ @\n  / _ \\@\n / ___/@\n/_/    @\n       @@\n  ____ @\n / __ \\@\n/ /_/ /@\n\\___\\_\\@\n       @@\n   ___ @\n  / _ \\@\n / , _/@\n/_/|_| @\n       @@\n   ____@\n  / __/@\n _\\ \\  @\n/___/  @\n       @@\n ______@\n/_  __/@\n / /   @\n/_/    @\n       @@\n  __  __@\n / / / /@\n/ /_/ / @\n\\____/  @\n        @@\n  _   __@\n | | / /@\n | |/ / @\n |___/  @\n        @@\n  _      __@\n | | /| / /@\n | |/ |/ / @\n |__/|__/  @\n           @@\n   _  __@\n  | |/_/@\n _>  <  @\n/_/|_|  @\n        @@\n __  __@\n \\ \\/ /@\n  \\  / @\n  /_/  @\n       @@\n  ____@\n /_  /@\n  / /_@\n /___/@\n      @@\n    ___@\n   / _/@\n  / /  @\n / /   @\n/__/   @@\n__   @\n\\ \\  @\n \\ \\ @\n  \\_\\@\n     @@\n    ___@\n   /  /@\n   / / @\n _/ /  @\n/__/   @@\n //|@\n|/||@\n $  @\n$   @\n    @@\n     @\n     @\n     @\n ____@\n/___/@@\n _ @\n( )@\n V @\n$  @\n   @@\n      @\n ___ _@\n/ _ `/@\n\\_,_/ @\n      @@\n   __ @\n  / / @\n / _ \\@\n/_.__/@\n      @@\n     @\n ____@\n/ __/@\n\\__/ @\n     @@\n     __@\n ___/ /@\n/ _  / @\n\\_,_/  @\n       @@\n     @\n ___ @\n/ -_)@\n\\__/ @\n     @@\n   ___@\n  / _/@\n / _/ @\n/_/   @\n      @@\n       @\n  ___ _@\n / _ `/@\n \\_, / @\n/___/  @@\n   __ @\n  / / @\n / _ \\@\n/_//_/@\n      @@\n   _ @\n  (_)@\n / / @\n/_/  @\n     @@\n      _ @\n     (_)@\n    / / @\n __/ /  @\n|___/   @@\n   __  @\n  / /__@\n /  '_/@\n/_/\\_\\ @\n       @@\n   __@\n  / /@\n / / @\n/_/  @\n     @@\n       @\n  __ _ @\n /  ' \\@\n/_/_/_/@\n       @@\n      @\n  ___ @\n / _ \\@\n/_//_/@\n      @@\n     @\n ___ @\n/ _ \\@\n\\___/@\n     @@\n       @\n   ___ @\n  / _ \\@\n / .__/@\n/_/    @@\n      @\n ___ _@\n/ _ `/@\n\\_, / @\n /_/  @@\n      @\n  ____@\n / __/@\n/_/   @\n      @@\n     @\n  ___@\n (_-<@\n/___/@\n     @@\n  __ @\n / /_@\n/ __/@\n\\__/ @\n     @@\n      @\n __ __@\n/ // /@\n\\_,_/ @\n      @@\n      @\n _  __@\n| |/ /@\n|___/ @\n      @@\n        @\n _    __@\n| |/|/ /@\n|__,__/ @\n        @@\n      @\n __ __@\n \\ \\ /@\n/_\\_\\ @\n      @@\n       @\n  __ __@\n / // /@\n \\_, / @\n/___/  @@\n    @\n ___@\n/_ /@\n/__/@\n    @@\n    __@\n  _/_/@\n_/ /  @\n/ /   @\n\\_\\   @@\n    __@\n   / /@\n  / / @\n / /  @\n/_/   @@\n   __  @\n   \\ \\ @\n   / /_@\n _/_/  @\n/_/    @@\n /\\//@\n//\\/ @\n $   @\n$    @\n     @@\n   _  _ @\n  (_)(_)@\n / - |  @\n/_/|_|  @\n        @@\n  _   _ @\n (_)_(_)@\n/ __ \\  @\n\\____/  @\n        @@\n  _   _ @\n (_) (_)@\n/ /_/ / @\n\\____/  @\n        @@\n  _  _ @\n (_)(_)@\n/ _ `/ @\n\\_,_/  @\n       @@\n  _  _ @\n (_)(_)@\n/ _ \\  @\n\\___/  @\n       @@\n  _  _ @\n (_)(_)@\n/ // / @\n\\_,_/  @\n       @@\n    ____ @\n   / _  )@\n  / /< < @\n / //__/ @\n/_/      @@\n160  NO-BREAK SPACE\n    $@\n   $ @\n  $  @\n $   @\n$    @@\n161  INVERTED EXCLAMATION MARK\n   _ @\n  (_)@\n / / @\n/_/  @\n     @@\n162  CENT SIGN\n     @\n __//@\n/ __/@\n\\ _/ @\n//   @@\n163  POUND SIGN\n     __ @\n  __/__|@\n /_ _/_ @\n(_,___/ @\n        @@\n164  CURRENCY SIGN\n   /|_/|@\n  | . / @\n /_  |  @\n|/ |/   @\n        @@\n165  YEN SIGN\n    ____@\n  _| / /@\n /_  __/@\n/_  __/ @\n /_/    @@\n166  BROKEN BAR\n    __@\n   / /@\n  /_/ @\n / /  @\n/_/   @@\n167  SECTION SIGN\n     __ @\n   _/ _)@\n  / | | @\n | |_/  @\n(__/    @@\n168  DIAERESIS\n _   _ @\n(_) (_)@\n $   $ @\n$   $  @\n       @@\n169  COPYRIGHT SIGN\n   ____  @\n  / ___\\ @\n / / _/ |@\n| |__/ / @\n \\____/  @@\n170  FEMININE ORDINAL INDICATOR\n   ___ _@\n  / _ `/@\n _\\_,_/ @\n/____/  @\n        @@\n171  LEFT-POINTING DOUBLE ANGLE QUOTATION MARK\n  ____@\n / / /@\n< < < @\n \\_\\_\\@\n      @@\n172  NOT SIGN\n     @\n ____@\n/_  /@\n /_/ @\n     @@\n173  SOFT HYPHEN\n    @\n ___@\n/__/@\n $  @\n    @@\n174  REGISTERED SIGN\n   ____  @\n  / __ \\ @\n / / -) |@\n| //\\\\ / @\n \\____/  @@\n175  MACRON\n ____@\n/___/@\n $   @\n$    @\n     @@\n176  DEGREE SIGN\n  __ @\n /. |@\n|__/ @\n $   @\n     @@\n177  PLUS-MINUS SIGN\n      __ @\n   __/ /_@\n  /_  __/@\n __/_/_  @\n/_____/  @@\n178  SUPERSCRIPT TWO\n  __ @\n |_ )@\n/__| @\n $   @\n     @@\n179  SUPERSCRIPT THREE\n  ___@\n |_ /@\n/__) @\n $   @\n     @@\n180  ACUTE ACCENT\n __@\n/_/@\n $ @\n$  @\n   @@\n181  MICRO SIGN\n        @\n   __ __@\n  / // /@\n / .,_/ @\n/_/     @@\n182  PILCROW SIGN\n  _____@\n /    /@\n|_ / / @\n/_/_/  @\n       @@\n183  MIDDLE DOT\n   @\n _ @\n(_)@\n$  @\n   @@\n184  CEDILLA\n   @\n   @\n   @\n _ @\n/_)@@\n185  SUPERSCRIPT ONE\n  __@\n < /@\n/_/ @\n$   @\n    @@\n186  MASCULINE ORDINAL INDICATOR\n   ___ @\n  / _ \\@\n _\\___/@\n/____/ @\n       @@\n187  RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK\n____  @\n\\ \\ \\ @\n > > >@\n/_/_/ @\n      @@\n188  VULGAR FRACTION ONE QUARTER\n  __  __   @\n < /_/_/___@\n/_//_//_' /@\n /_/   /_/ @\n           @@\n189  VULGAR FRACTION ONE HALF\n  __  __  @\n < /_/_/_ @\n/_//_/|_ )@\n /_/ /__| @\n          @@\n190  VULGAR FRACTION THREE QUARTERS\n  ___  __   @\n |_ /_/_/___@\n/__)/_//_' /@\n  /_/   /_/ @\n            @@\n191  INVERTED QUESTION MARK\n   _ @\n _(_)@\n/ _/_@\n\\___/@\n     @@\n192  LATIN CAPITAL LETTER A WITH GRAVE\n   __ @\n  _\\_\\@\n / - |@\n/_/|_|@\n      @@\n193  LATIN CAPITAL LETTER A WITH ACUTE\n    __@\n  _/_/@\n / - |@\n/_/|_|@\n      @@\n194  LATIN CAPITAL LETTER A WITH CIRCUMFLEX\n    //|@\n  _|/||@\n / - | @\n/_/|_| @\n       @@\n195  LATIN CAPITAL LETTER A WITH TILDE\n    /\\//@\n  _//\\/ @\n / - |  @\n/_/|_|  @\n        @@\n196  LATIN CAPITAL LETTER A WITH DIAERESIS\n   _  _ @\n  (_)(_)@\n / - |  @\n/_/|_|  @\n        @@\n197  LATIN CAPITAL LETTER A WITH RING ABOVE\n   (())@\n  / _ |@\n / __ |@\n/_/ |_|@\n       @@\n198  LATIN CAPITAL LETTER AE\n   _______@\n  / _  __/@\n / _  _/  @\n/_//___/  @\n          @@\n199  LATIN CAPITAL LETTER C WITH CEDILLA\n  _____@\n / ___/@\n/ /__  @\n\\___/  @\n/_)    @@\n200  LATIN CAPITAL LETTER E WITH GRAVE\n  __ @\n  \\_\\@\n / -<@\n/__< @\n     @@\n201  LATIN CAPITAL LETTER E WITH ACUTE\n    __@\n  _/_/@\n / -< @\n/__<  @\n      @@\n202  LATIN CAPITAL LETTER E WITH CIRCUMFLEX\n   //|@\n  |/||@\n / -< @\n/__<  @\n      @@\n203  LATIN CAPITAL LETTER E WITH DIAERESIS\n  _  _ @\n (_)(_)@\n / -<  @\n/__<   @\n       @@\n204  LATIN CAPITAL LETTER I WITH GRAVE\n   __  @\n  _\\_\\ @\n /_ __/@\n/____/ @\n       @@\n205  LATIN CAPITAL LETTER I WITH ACUTE\n     __@\n  __/_/@\n /_ __/@\n/____/ @\n       @@\n206  LATIN CAPITAL LETTER I WITH CIRCUMFLEX\n    //|@\n  _|/||@\n /_ __/@\n/____/ @\n       @@\n207  LATIN CAPITAL LETTER I WITH DIAERESIS\n   _  _ @\n  (_)(_)@\n /_ __/ @\n/____/  @\n        @@\n208  LATIN CAPITAL LETTER ETH\n   ____ @\n _/ __ \\@\n/_ _// /@\n/_____/ @\n        @@\n209  LATIN CAPITAL LETTER N WITH TILDE\n     /\\//@\n  __//\\/ @\n /  |/ / @\n/_/|__/  @\n         @@\n210  LATIN CAPITAL LETTER O WITH GRAVE\n  __  @\n _\\_\\ @\n/ __ \\@\n\\____/@\n      @@\n211  LATIN CAPITAL LETTER O WITH ACUTE\n    __@\n __/_/@\n/ __ \\@\n\\____/@\n      @@\n212  LATIN CAPITAL LETTER O WITH CIRCUMFLEX\n   //|@\n _|/||@\n/ __ \\@\n\\____/@\n      @@\n213  LATIN CAPITAL LETTER O WITH TILDE\n   /\\//@\n _//\\/ @\n/ __ \\ @\n\\____/ @\n       @@\n214  LATIN CAPITAL LETTER O WITH DIAERESIS\n  _   _ @\n (_)_(_)@\n/ __ \\  @\n\\____/  @\n        @@\n215  MULTIPLICATION SIGN\n     @\n /|/|@\n > < @\n|/|/ @\n     @@\n216  LATIN CAPITAL LETTER O WITH STROKE\n  _____ @\n / _// \\@\n/ //// /@\n\\_//__/ @\n        @@\n217  LATIN CAPITAL LETTER U WITH GRAVE\n   __  @\n __\\_\\ @\n/ /_/ /@\n\\____/ @\n       @@\n218  LATIN CAPITAL LETTER U WITH ACUTE\n     __@\n __ /_/@\n/ /_/ /@\n\\____/ @\n       @@\n219  LATIN CAPITAL LETTER U WITH CIRCUMFLEX\n    //|@\n __|/||@\n/ /_/ /@\n\\____/ @\n       @@\n220  LATIN CAPITAL LETTER U WITH DIAERESIS\n  _   _ @\n (_) (_)@\n/ /_/ / @\n\\____/  @\n        @@\n221  LATIN CAPITAL LETTER Y WITH ACUTE\n   __@\n__/_/@\n\\ V /@\n /_/ @\n     @@\n222  LATIN CAPITAL LETTER THORN\n   __ @\n  / / @\n / -_)@\n/_/   @\n      @@\n223  LATIN SMALL LETTER SHARP S\n    ____ @\n   / _  )@\n  / /< < @\n / //__/ @\n/_/      @@\n224  LATIN SMALL LETTER A WITH GRAVE\n  __  @\n _\\_\\_@\n/ _ `/@\n\\_,_/ @\n      @@\n225  LATIN SMALL LETTER A WITH ACUTE\n    __@\n __/_/@\n/ _ `/@\n\\_,_/ @\n      @@\n226  LATIN SMALL LETTER A WITH CIRCUMFLEX\n   //|@\n _|/||@\n/ _ `/@\n\\_,_/ @\n      @@\n227  LATIN SMALL LETTER A WITH TILDE\n   /\\//@\n _//\\/ @\n/ _ `/ @\n\\_,_/  @\n       @@\n228  LATIN SMALL LETTER A WITH DIAERESIS\n  _  _ @\n (_)(_)@\n/ _ `/ @\n\\_,_/  @\n       @@\n229  LATIN SMALL LETTER A WITH RING ABOVE\n   __ @\n _(())@\n/ _ `/@\n\\_,_/ @\n      @@\n230  LATIN SMALL LETTER AE\n         @\n ___ ___ @\n/ _ ` -_)@\n\\_,____/ @\n         @@\n231  LATIN SMALL LETTER C WITH CEDILLA\n     @\n ____@\n/ __/@\n\\__/ @\n/_)  @@\n232  LATIN SMALL LETTER E WITH GRAVE\n  __ @\n _\\_\\@\n/ -_)@\n\\__/ @\n     @@\n233  LATIN SMALL LETTER E WITH ACUTE\n   __@\n _/_/@\n/ -_)@\n\\__/ @\n     @@\n234  LATIN SMALL LETTER E WITH CIRCUMFLEX\n  //|@\n |/||@\n/ -_)@\n\\__/ @\n     @@\n235  LATIN SMALL LETTER E WITH DIAERESIS\n _  _ @\n(_)(_)@\n/ -_) @\n\\__/  @\n      @@\n236  LATIN SMALL LETTER I WITH GRAVE\n  __ @\n  \\_\\@\n / / @\n/_/  @\n     @@\n237  LATIN SMALL LETTER I WITH ACUTE\n   __@\n  /_/@\n / / @\n/_/  @\n     @@\n238  LATIN SMALL LETTER I WITH CIRCUMFLEX\n   //|@\n  |/||@\n / /  @\n/_/   @\n      @@\n239  LATIN SMALL LETTER I WITH DIAERESIS\n _   _ @\n(_)_(_)@\n / /   @\n/_/    @\n       @@\n240  LATIN SMALL LETTER ETH\n   _||_@\n __ || @\n/ _` | @\n\\___/  @\n       @@\n241  LATIN SMALL LETTER N WITH TILDE\n    /\\//@\n  _//\\/ @\n / _ \\  @\n/_//_/  @\n        @@\n242  LATIN SMALL LETTER O WITH GRAVE\n  __ @\n _\\_\\@\n/ _ \\@\n\\___/@\n     @@\n243  LATIN SMALL LETTER O WITH ACUTE\n   __@\n _/_/@\n/ _ \\@\n\\___/@\n     @@\n244  LATIN SMALL LETTER O WITH CIRCUMFLEX\n   //|@\n _|/||@\n/ _ \\ @\n\\___/ @\n      @@\n245  LATIN SMALL LETTER O WITH TILDE\n   /\\//@\n _//\\/ @\n/ _ \\  @\n\\___/  @\n       @@\n246  LATIN SMALL LETTER O WITH DIAERESIS\n  _  _ @\n (_)(_)@\n/ _ \\  @\n\\___/  @\n       @@\n247  DIVISION SIGN\n   _ @\n _(_)@\n/___/@\n(_)  @\n     @@\n248  LATIN SMALL LETTER O WITH STROKE\n     @\n ___ @\n/ //\\@\n\\//_/@\n     @@\n249  LATIN SMALL LETTER U WITH GRAVE\n   __ @\n __\\_\\@\n/ // /@\n\\_,_/ @\n      @@\n250  LATIN SMALL LETTER U WITH ACUTE\n    __@\n __/_/@\n/ // /@\n\\_,_/ @\n      @@\n251  LATIN SMALL LETTER U WITH CIRCUMFLEX\n   //|@\n _|/||@\n/ // /@\n\\_,_/ @\n      @@\n252  LATIN SMALL LETTER U WITH DIAERESIS\n  _  _ @\n (_)(_)@\n/ // / @\n\\_,_/  @\n       @@\n253  LATIN SMALL LETTER Y WITH ACUTE\n     __@\n  __/_/@\n / // /@\n \\_, / @\n/___/  @@\n254  LATIN SMALL LETTER THORN\n    __ @\n   / / @\n  / _ \\@\n / .__/@\n/_/    @@\n255  LATIN SMALL LETTER Y WITH DIAERESIS\n   _  _ @\n  (_)(_)@\n / // / @\n \\_, /  @\n/___/   @@").is_err(){
			println!("Error: Something happened wrong (writing {})", config_file)}
    }
    let slant = figlet_rs::FIGfont::from_file(config_file.as_str()).unwrap();
    let bannered = slant.convert(&bannered.replace("Linux", ""));
    assert!(bannered.is_some());
    bannered.unwrap().to_string()
}

pub fn random_color_codes() -> (u8, u8, u8) {
    let mut rand = thread_rand();
    (
        rand.next_u16() as u8,
        rand.next_u16() as u8,
        rand.next_u16() as u8,
    )
}

fn main() {
    let infos = crate::resource::sys::init();
    let config_file = format!("{}/.config/fetrust/{}", env!("HOME"), "config.json");
    if !Path::new(&config_file).exists() {
        println!("Creating default config ({})", config_file);
        if fs::create_dir_all(format!("{}/.config/fetrust", env!("HOME"))).is_err() {
            println!(
                "Error: Something happened wrong (creating {})",
                format!("{}/.config/", env!("HOME"))
            )
        }
        if fs::write(&config_file, b"{\n\t\"banner\":\t\t\t[[\"os\"], \"rand\"],\n\t\"user_a_host_name\":\t[[\"          \",\"username\",\"@\",\"hostname\",\"          \"], \"yellow\"],\n\t\"brace\":\t\t\t[[\"____________________________________\"], null],\n\t\"os\":\t\t\t\t[[\"os\t==> \",\"os\",\" \",\"release\"], null],\n\t\"kernel\":\t\t\t[[\"Kernel\t==> \",\"kernel_name\",\" \",\"kernel\"], \"green\"],\n\t\"shell\":\t\t\t[[\"Shell\t==> \",\"shell\"], \"purple\"],\n\t\"family\":\t\t\t[[\"Family\t==> \",\"family\"], \"cyan\"],\n\t\"uptime\":\t\t\t[[\"Uptime\t==> \",\"uptime\"], null],\n\t\"cpu_type\":\t\t\t[[\"CPUt\t==> \",\"cpu_type\"], null]\n}").is_err() {
			println!("Error: Something happened wrong (writing {})", config_file)}
    }
    let config_json = &fs::read(config_file).unwrap();
    let json = Json::parse(config_json);
    for info in [
        "banner",
        "user_a_host_name",
        "brace",
        "os",
        "kernel",
        "shell",
        "family",
        "uptime",
        "cpu_type",
    ] {
        match json.as_ref().unwrap().get(info) {
            Some(info) => {
                match info {
                    Json::OBJECT { name, value } => {
                        let mut printing = "".to_string();
                        let mut printingc = "".to_string();
                        match value.unbox() {
                            Json::ARRAY(value) => {
                                match value[0].unbox() {
                                    Json::ARRAY(value) => {
                                        let mut printingl = vec!["".to_string(); value.len()];
                                        let mut i = 0;
                                        for string in value {
                                            printingl[i] = string.print();
                                            i += 1;
                                        }
                                        i = 0;

                                        let bprintingl = printingl.clone();

                                        for getter in bprintingl {
                                            match getter.as_str() {
                                                "os" => printingl[i] = infos.os.clone(),
                                                "os_release" => {
                                                    printingl[i] = infos.os_release.clone()
                                                }
                                                "username" => printingl[i] = infos.username.clone(),
                                                "hostname" => printingl[i] = infos.hostname.clone(),
                                                "kernel_name" => {
                                                    printingl[i] = infos.kernel_name.clone()
                                                }
                                                "kernel" => printingl[i] = infos.kernel.clone(),
                                                "shell" => printingl[i] = infos.shell.clone(),
                                                "family" => printingl[i] = infos.family.clone(),
                                                "uptime" => printingl[i] = infos.uptime.clone(),
                                                "cpu_type" => printingl[i] = infos.cpu_type.clone(),
                                                _ => {}
                                            }
                                            i += 1;
                                        }
                                        printingc = printingl.join("");
                                    }
                                    _ => {}
                                }
                                match value[1].unbox() {
                                    Json::STRING(value) => match value.as_str() {
                                        "black" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = Black.paint(&printingc).to_string();
                                        }
                                        "red" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = Red.paint(&printingc).to_string();
                                        }
                                        "green" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = Green.paint(&printingc).to_string();
                                        }
                                        "yellow" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = Yellow.paint(&printingc).to_string();
                                        }
                                        "blue" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = Blue.paint(&printingc).to_string();
                                        }
                                        "purple" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = Purple.paint(&printingc).to_string();
                                        }
                                        "cyan" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = Cyan.paint(&printingc).to_string();
                                        }
                                        "white" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            printing = White.paint(&printingc).to_string();
                                        }
                                        "rand" | "random" => {
                                            if name.as_str() == "banner" {
                                                printingc = get_banner(printingc.to_string());
                                            }
                                            let (r, g, b) = random_color_codes();
                                            printing = RGB(r, g, b).paint(&printingc).to_string();
                                        }
                                        _ => {
                                            printing = printingc;
                                            println!("{}", Yellow.paint(format!("Warning: Color \"{}\" isn't defined, so it's default color.", value.as_str())));
                                        }
                                    },
                                    Json::NULL => {
                                        printing = printingc;
                                    }
                                    _ => {}
                                }
                                //" idk to should I delete this
                                match name.as_str() {
                                    "banner" => {
                                        println!("{}", printing);
                                    }
                                    "user_a_host_name" => {
                                        println!("{}", printing);
                                    }
                                    "brace" => {
                                        println!("{}", printing);
                                    }
                                    "os" => {
                                        println!("{}", printing);
                                    }
                                    "kernel" => {
                                        println!("{}", printing);
                                    }
                                    "shell" => {
                                        println!("{}", printing);
                                    }
                                    "family" => {
                                        println!("{}", printing);
                                    }
                                    "uptime" => {
                                        println!("{}", printing);
                                    }
                                    "cpu_type" => {
                                        println!("{}", printing);
                                    }
                                    _ => {}
                                }
                                //"
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
