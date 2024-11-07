use std::usize;

use burn::{
    data,
    tensor::{Tensor, TensorData},
};
use opencv::{
    core::{Mat, MatTraitConstManual},
    videoio::{self, VideoCaptureTrait, VideoCaptureTraitConst},
};

#[derive(Debug)]
pub struct Camara {
    cam: opencv::videoio::VideoCapture,
    frame: Option<opencv::core::Mat>,
}

impl Default for Camara {
    fn default() -> Self {
        let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("any camera yo have ?");
        let opened = videoio::VideoCapture::is_opened(&cam).expect("camera is no open");
        if !opened {
            panic!("if you have a camera!!!");
        }
        Self { cam, frame: None }
    }
}

impl Camara {
    pub fn capture(&mut self) -> opencv::Result<()> {
        let mut frame = Mat::default();
        self.cam.read(&mut frame)?;
        self.frame = Some(frame);
        Ok(())
    }

    pub fn raw_data(&mut self) -> opencv::Result<(Vec<u8>, usize, usize)> {
        self.capture()?;
        let frame = self.frame.take().unwrap();
        let data = frame.to_vec_2d::<rgb::RGB8>().unwrap();
        let (height, width) = (data.len(), data[0].len());
        let data = data
            .into_iter()
            .flatten()
            .collect::<Vec<rgb::RGB8>>()
            .iter()
            .flat_map(|&x| vec![x.b, x.g, x.r])
            .collect();
        Ok((data, width, height))
    }
}
