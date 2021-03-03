use std::{env, thread, time};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
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
    id: String,
}

// nohup ./dad-rust-bash >/dev/null 2>&1 &
fn main() {
    if !Path::new("data.json").exists() {
        println!("请在项目根目录下创建data.json文件，\r\n格式请参考：https://github.com/schizobulia/dad-rust-bash/data.json ");
        return;
    }

    let root_path = format!("{}/{}", now_dir_path(), "data.json");
    let file = File::open(root_path).unwrap();
    let reader = BufReader::new(file);
    let mut data: Vec<Movie> = serde_json::from_reader(reader).unwrap();
    let mut create_data = Vec::new();
    let mut tag = true;
    while tag && data.len() > 0 {
        tag = false;
        let mut movie = data.pop().unwrap();
        let step1 = format!("mkdir {}", movie.id);
        let step2 = format!("ffmpeg -y -i {}  -vcodec copy -acodec copy -vbsf h264_mp4toannexb ./{}/output.ts", movie.url, movie.id);
        let step3 = format!("ffmpeg -i ./{}/output.ts -c copy -map 0 -f segment -segment_list ./{}/b.m3u8 -segment_time 120 ./{}/player-%03d.ts",
            movie.id, movie.id, movie.id);
        let step4 = format!("rm -rf ./{}/output.ts", movie.id);
        let step5 = format!("wget {} -O ./{}/output.jpg", movie.img, movie.id);
        Command::new("sh").arg("-c").arg(step1).output().unwrap();
        Command::new("sh").arg("-c").arg(step2).output().unwrap();
        Command::new("sh").arg("-c").arg(step3).output().unwrap();
        Command::new("sh").arg("-c").arg(step4).output().unwrap();
        Command::new("sh").arg("-c").arg(step5).output().unwrap();
        movie.img = format!("/{}/output.jpg", movie.id);
        movie.url = format!("/{}/b.m3u8", movie.id);
        create_data.push(movie);
        tag = true;
    }

    if Path::new("create.json").exists() {
        fs::remove_file("create.json").expect("remove file failed");
    }
    let mut file = std::fs::File::create("create.json").expect("create failed");
    file.write_all(serde_json::to_string(&create_data).unwrap().as_str().as_bytes()).expect("write failed");
}

