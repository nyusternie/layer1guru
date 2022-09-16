pub fn get_version() -> &'static str {
    return "22.9.16 (alpha)";
}

/**
 * Welcome Banner
 * 
 * Prints a welcome banner when the CLI is executed.
 */
pub fn display_banner() {
    println!(r"
    _________    ___.                  __      ________                    
   /   _____/__ _\_ |__   ____   _____/  |_   /  _____/ __ _________ __ __ 
   \_____  \|  |  \ __ \ /    \_/ __ \   __\ /   \  ___|  |  \_  __ \  |  \
   /        \  |  / \_\ \   |  \  ___/|  |   \    \_\  \  |  /|  | \/  |  /
  /_______  /____/|___  /___|  /\___  >__|    \______  /____/ |__|  |____/ 
          \/          \/     \/     \/               \/                    ");

    println!("                                                      v{}\n", get_version());
}

/**
 * Welcome Banner (Alternate)
 * 
 * Prints a welcome banner when the CLI is executed.
 */
pub fn display_banner_alt() {
    println!(r"

   ███████╗██╗   ██╗██████╗ ███╗   ██╗███████╗████████╗     ██████╗ ██╗   ██╗██████╗ ██╗   ██╗
   ██╔════╝██║   ██║██╔══██╗████╗  ██║██╔════╝╚══██╔══╝    ██╔════╝ ██║   ██║██╔══██╗██║   ██║
   ███████╗██║   ██║██████╔╝██╔██╗ ██║█████╗     ██║       ██║  ███╗██║   ██║██████╔╝██║   ██║
   ╚════██║██║   ██║██╔══██╗██║╚██╗██║██╔══╝     ██║       ██║   ██║██║   ██║██╔══██╗██║   ██║
   ███████║╚██████╔╝██████╔╝██║ ╚████║███████╗   ██║       ╚██████╔╝╚██████╔╝██║  ██║╚██████╔╝
   ╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝   ╚═╝        ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ");

    println!("                                                                    v{}\n", get_version());
}

/**
 * Welcome Banner (Alternate #2)
 * 
 * Prints a welcome banner when the CLI is executed.
 */
pub fn display_banner_alt_2() {
    println!(r"
    _____ __ __  ____   ____     ___ ______       ____  __ __  ____  __ __ 
   / ___/|  |  ||    \ |    \   /  _]      |     /    ||  |  ||    \|  |  |
  (   \_ |  |  ||  o  )|  _  | /  [_|      |    |   __||  |  ||  D  )  |  |
   \__  ||  |  ||     ||  |  ||    _]_|  |_|    |  |  ||  |  ||    /|  |  |
   /  \ ||  :  ||  O  ||  |  ||   [_  |  |      |  |_ ||  :  ||    \|  :  |
   \    ||     ||     ||  |  ||     | |  |      |     ||     ||  .  \     |
    \___| \__,_||_____||__|__||_____| |__|      |___,_| \__,_||__|\_|\__,_|");

     println!("                                                     v{}\n", get_version());
}