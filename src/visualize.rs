use std::io::Write;
use std::fmt;
use std::process::{Command, Stdio};

use image::{DynamicImage, ImageResult};
use petgraph::graph::Graph;
use petgraph::dot::{Dot, Config};
use rand::{Rng, thread_rng};
use crate::organism::NeuralNetwork;

pub(crate) fn visualize_nnet(nnet: &NeuralNetwork) {

// pass graph representation to the dot command and read bytes
    let dot_graph = Dot::new(&nnet.0).to_string();
    let mut cmd = Command::new("dot")
        .args(["-Tpng"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("dot command failed to start");

    let mut stdin = cmd.stdin.take().expect("Failed to open cmd stdin");
    std::thread::spawn(move || { stdin.write_all(dot_graph.as_bytes()).expect("Failed to write to stdin"); });
    let output = cmd.wait_with_output().expect("Failed to read stdout");

    let res = match output.status.code() {
        Some(0) => {
            Some(image::load_from_memory_with_format(&output.stdout, image::ImageFormat::Png))
        }
        Some(code) => {
            println!("Exited with status code {}\n {}", code, String::from_utf8_lossy(&output.stderr));
            None
        }
        _ => unreachable!(),
    };

    let image = match res.unwrap().unwrap() {
        DynamicImage::ImageRgba8(image) => Some(image),
        _ => None,
    }.unwrap();

    let mut rng = thread_rng();
    let image_path = format!("visualizations/img-{}.png", rng.gen_range(0..999));
    image.save(&image_path).unwrap();
    //
    // let mut open_viewer = Command::new("xdg-open")
    //     .arg(image_path)
    //     .spawn()
    //     .expect("Failed to use xdg-open to view graph");
}