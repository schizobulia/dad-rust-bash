use std::{env};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use serde::Deserialize;
use serde::Serialize;
use std::process::Command;
use std::path::Path;
use futures::channel::mpsc;
use futures::executor;
use futures::executor::ThreadPool;
use futures::StreamExt;

fn now_dir_path() -> String {
    env::current_dir().unwrap().as_path().to_string_lossy().to_string()
}

#[derive(Deserialize, Serialize)]
#[derive(Clone)]
struct Movie {
    title: String,
    url: String,
    img: String,
    id: String,
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
    let data: Vec<Movie> = serde_json::from_reader(reader).unwrap();
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<Movie>();

    let fut_values = async {
        let fut_tx_result = async move {
            for mut movie in data {
                set_work(movie.clone());
                movie.img = format!("/{}/output.jpg", movie.id);
                movie.url = format!("/{}/b.m3u8", movie.id);
                tx.unbounded_send(movie).expect("Failed to send")
            }
        };

        pool.spawn_ok(fut_tx_result);

        let fut_values = rx
            .map(|mv| mv)
            .collect();

        fut_values.await
    };

    let create_data: Vec<Movie> = executor::block_on(fut_values);

    if Path::new("create.json").exists() {
        fs::remove_file("create.json").expect("remove file failed");
    }
    let mut file = std::fs::File::create("create.json").expect("create failed");
    file.write_all(serde_json::to_string(&create_data).unwrap().as_str().as_bytes()).expect("write failed");
}


fn set_work(movie: Movie) {
    let step1 = format!("mkdir {}", movie.id);
    let step3 = format!("ffmpeg -y -i {} -c copy -map 0 -f segment -segment_list ./{}/b.m3u8 -segment_time 120 ./{}/player-%03d.ts",
                        movie.url, movie.id, movie.id);
    let step5 = format!("wget {} -O ./{}/output.jpg", movie.img, movie.id);
    Command::new("sh").arg("-c").arg(step1).spawn().unwrap();
    Command::new("sh").arg("-c").arg(step3).spawn().unwrap();
    Command::new("sh").arg("-c").arg(step5).spawn().unwrap();
}