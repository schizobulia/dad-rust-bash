use std::{env};
use std::fs::File;
use std::io::{BufReader};
use serde::Deserialize;
use serde::Serialize;
use std::process::Command;
use std::path::Path;

fn now_dir_path() -> String {
    env::current_dir().unwrap().as_path().to_string_lossy().to_string()
}

#[derive(Deserialize, Serialize)]
struct Movie {
    title: String,
    url: String,
    img: String,
}

// nohup ./dad-rust-bash >/dev/null 2>&1 &
fn main() {
    if cfg!(target_os = "windows") {
        println!("该脚本只可以在Linux系统下使用!");
        return;
    }

    if !Path::new("data.json").exists() {
        println!("请在项目根目录下创建data.json文件，\r\n格式请参考：https://github.com/schizobulia/dad-rust-bash/data.json ");
        return;
    }

    let root_path = format!("{}/{}", now_dir_path(), "data.json");
    let file = File::open(root_path).unwrap();
    let reader = BufReader::new(file);
    let mut data: Vec<Movie> = serde_json::from_reader(reader).unwrap();
    let mut tag = true;

    while tag && data.len() > 0 {
        tag = false;
        let movie = data.pop().unwrap();
        let step1 = format!("mkdir {}", movie.title);
        let step2 = format!("ffmpeg -y -i {}  ./{}/{}.mp4", movie.url, movie.title, movie.title);
        let step3 = format!("wget {} -O ./{}/cover.jpg", movie.img, movie.title);

        Command::new("sh").arg("-c").arg(step1).output().unwrap();
        Command::new("sh").arg("-c").arg(step2).output().unwrap();
        Command::new("sh").arg("-c").arg(step3).output().unwrap();
        tag = true;
    }
}

