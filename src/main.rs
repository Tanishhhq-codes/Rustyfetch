use sysinfo::System;
use colored::*;
use whoami;
use std::env;

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    // ===== USER + HOST =====
    let user = whoami::username().unwrap_or_default();
    let host = whoami::hostname().unwrap_or_default();

    // ===== SYSTEM INFO =====
    let os = System::name().unwrap_or("Unknown".to_string());
    let kernel = System::kernel_version().unwrap_or("Unknown".to_string());

    let cpu = system
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or("Unknown CPU".to_string());

    let total_ram = system.total_memory() / 1024;
    let used_ram = system.used_memory() / 1024;

    let uptime_mins = System::uptime() / 60;
    let uptime_hours = uptime_mins / 60;
    let uptime_final = format!("{}h {}m", uptime_hours, uptime_mins % 60);

    // ===== EXTRA FEATURES =====
    let shell = env::var("SHELL").unwrap_or("unknown".into());
    let shell = shell.split('/').last().unwrap_or("unknown");

    let wm = env::var("XDG_CURRENT_DESKTOP")
        .or(env::var("DESKTOP_SESSION"))
        .unwrap_or("unknown".into());

    // ===== YOUR CRAB LOGO =====
    let logo = r#"
                                           +     +++     +                                          
                                          ++++  +++++  ++++                                         
                                   ++++  +++++++++++++++++++  ++++                                  
                                   +++++++++++++++++++++++++++++++                                  
                             +++++++++++++++++++++++++++++++++++++++++++                            
                             +++++++++++++++++++++++++++++++++++++++++++                  ++        
        ++              ++++ +++++++++++++++++++++++++++++++++++++++++++++++++          ++++     +  
   ++   ++++            +++++++++++++++++++++++++++++++++++++++++++++++++++++         ++++++    +++ 
  +++   ++++++          +++++++++++++++++++++++++++++++++++++++++++++++++++++        +++++++   ++++ 
 +++++   ++++++    +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++   +++++++  +++++ 
 ++++++  ++++++    +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++   +++++++++++++  
  +++++++++++++     +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++    ++++++++++++   
   +++++++++++     +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++    +++++++++     
     ++++++++  +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ ++++++       
         ++++    +++++++++++++++++++++++++   @@@%+++++++@   @@@++++++++++++++++++++++ ++++          
          ++++++  +++++++++++++++++++++++@   @@@@*+++++#@   @@@++++++++++++++++++++ +++++           
            +++++++++++++++++++++++++++++@@=@@@@@@+++++@@@@@@@@@+++++++++++++++++++++++             
              +++++++++++++++++++++++++++@@@@@@@@+++++++@@@@@@@+++++++++++++++++++++++++            
             ++++++++++++++++++++++++++++++@@@@@+++++++++@@@@*++++++++++++++++++++++++++++          
           +++++++ #####++++++++++++++++++++++++++++++++++++++++++++++++++++*### ###++++++          
            ++++++  ###  ####*++++++++++++++++++++@@@@@@+++++++++++++++*#####   ### +++++           
              +++++   ##      #######*++++++++++++++*+++++++++++*#######       ###  ++++            
                ++++   ##               ########################              ##   ++++             
                  +++    #                                                    #   +++               
                   ++++   #                                                  #   +++                
                     +++                                                         ++                 
                       ++                                                       ++                  
                         +                                                                          
"#;

   let logo_lines: Vec<&str> = logo.lines().collect();

    // ===== INFO =====
    let info_lines = vec![
        format!("{}", format!("{}@{}", user, host).bold().green()),
        "──────────────".bright_black().to_string(),
        format!("{} {}", "󰣇 OS:".cyan(), os),
        format!("{} {}", " Kernel:".cyan(), kernel),
        format!("{} {}", " CPU:".cyan(), cpu),
        format!(
            "{} {}MB / {}MB",
            "󰍛 RAM:".cyan(),
            used_ram,
            total_ram
        ),
        format!("{} {}", "󰅐 Uptime:".cyan(), uptime_final),
        format!("{} {}", " Shell:".cyan(), shell),
        format!("{} {}", " WM:".cyan(), wm),
    ];

    let max_lines = std::cmp::max(logo_lines.len(), info_lines.len());

    // ===== PRINT (BETTER ALIGNMENT) =====
    for i in 0..max_lines {
        let logo_part = logo_lines.get(i).unwrap_or(&"");
        let info_part = info_lines
            .get(i)
            .map(|s| s.as_str())
            .unwrap_or("");

        println!(
            "{:<35}   {}",
            logo_part.truecolor(255, 102, 0),
            info_part
        );
    }
}