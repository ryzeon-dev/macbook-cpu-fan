#![allow(non_snake_case, unused_must_use)]

use rsjson;

const DAEMON_DIR: &str = "/etc/macbook-cpu-fan";
const DRIVER_DIR: &str = "/sys/devices/platform/applesmc.768";
const TEMP_DIR: &str = "/sys/class/thermal/thermal_zone1";

struct Configuration {
    lowTemp: usize,
    midTemp: usize,
    highTemp: usize,
    
    lowSpeed: usize,
    midSpeed: usize,
    highSpeed: usize
}

impl Configuration {
    fn new() -> Configuration {
        let confName = std::fs::read_to_string(
            format!("{}/conf.txt", DAEMON_DIR)
        ).unwrap();
        let confFilePath = format!("{}/conf/{}", DAEMON_DIR, confName);

        let json = rsjson::Json::fromFile(confFilePath.trim().to_string());
        let temp = json.get("temp".to_string()).unwrap().toJson().unwrap();
        let speed = json.get("speed".to_string()).unwrap().toJson().unwrap();

        Configuration {
            lowTemp: temp.get("low".to_string()).unwrap().toUsize().unwrap(),
            midTemp: temp.get("mid".to_string()).unwrap().toUsize().unwrap(),
            highTemp: temp.get("high".to_string()).unwrap().toUsize().unwrap(),

            lowSpeed: speed.get("low".to_string()).unwrap().toUsize().unwrap(),
            midSpeed: speed.get("mid".to_string()).unwrap().toUsize().unwrap(),
            highSpeed: speed.get("high".to_string()).unwrap().toUsize().unwrap()
        }
    }
}

#[derive(Debug)]
struct Fan {
    fan: String,
    min: usize,
    max: usize
}

impl Fan {
    fn setSpeedRpm(&self, rpm: usize) -> bool {
       std::process::Command::new("/bin/sh").arg(format!("sudo chmod 666 {}/{}", DRIVER_DIR, self.fan));

        let enabler: String = {
            let tmp = self.fan.split("_");
            let collected = tmp.collect::<Vec<&str>>();
            collected.get(0).unwrap().to_string()
        };
        std::fs::write(format!("{}_manual", enabler), "1");

        match std::fs::write(&self.fan, format!("{}", rpm)) {
            Err(_) => {
                return false
            },
            Ok(_) => {
                return true
            }
        }
    }

    fn setSpeedPercent(&self, percent: usize) -> bool {
        let rpmValue: usize = percent * (self.max - self.min) / 100 + self.min;
        self.setSpeedRpm(rpmValue)
    }
}

struct Fans {
    fans: Vec<Fan>
}

impl Fans {
    fn new() -> Fans {
        let dirContent = std::fs::read_dir(
            std::path::Path::new(
                format!("{}", DRIVER_DIR).as_str()
            )
        ).unwrap();

        let mut fans = Vec::<Fan>::new();
        for element in dirContent {
            let tmp = element.unwrap().path();
            let strElement = tmp.to_str().unwrap().clone();

            if ! strElement.contains(&"_output".to_string()) {
                continue
            }

            let fanName = strElement.to_string();
            let splitted = fanName.split("_").collect::<Vec<&str>>();
            let fanTag = splitted.first().unwrap();

            let fanMin = std::fs::read_to_string(
                format!("{}_min", fanTag)
            ).unwrap().trim().parse::<usize>().unwrap();

            let fanMax = std::fs::read_to_string(
                format!("{}_max", fanTag)
            ).unwrap().trim().parse::<usize>().unwrap();

            fans.push(
                Fan {
                    fan: fanName,
                    min: fanMin, 
                    max: fanMax
                }
            );
        }
        return Fans { fans: fans };
    }

    fn setSpeedPercent(&self, percent: usize) {
        for fan in &self.fans {
            let status = fan.setSpeedPercent(percent);
            
            if !status {
                println!("problem setting speed for {:?}", fan);
            }
        }
    }
}

fn readTemperature() -> usize {
    match std::fs::read_to_string(format!("{}/temp", TEMP_DIR)) {
        Err(_) => {
            return 0_usize
        },
        Ok(value) => {
            return value.trim().parse::<usize>().unwrap() / 1000_usize
        }
    }
}

fn main() {
    let config = Configuration::new();
    let fans = Fans::new();

    loop {
        let temp = readTemperature();

        if temp < config.lowTemp {
            println!("below low: {}", temp);
            fans.setSpeedPercent(config.lowSpeed);
        
        } else if temp >= config.lowTemp && temp < config.midTemp {
            let speed = (temp - config.lowTemp) * 
                (config.midSpeed - config.lowSpeed) / 
                (config.midTemp - config.lowTemp) + config.lowSpeed;
            fans.setSpeedPercent(speed);

            println!("mid-low: {}C -> {}%", temp, speed);
        
        } else if temp >= config.midTemp && temp < config.highTemp {
            let speed = (temp - config.midTemp) * 
                (config.highSpeed - config.midSpeed) / 
                (config.highTemp - config.midTemp) + config.midSpeed;
            fans.setSpeedPercent(speed);
            println!("mid-high: {}C -> {}%", temp, speed);

        } else {
            println!("over high: {}", temp);
            fans.setSpeedPercent(config.highSpeed);
        }

        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
