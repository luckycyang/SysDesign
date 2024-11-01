use burn::backend::wgpu::{init_sync, AutoGraphicsApi, Wgpu, WgpuDevice};
use burn::{
    module::Module,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
    tensor::{Tensor, TensorData},
};

pub type Backend = Wgpu<f32, i32>;

use crate::model::Model;

pub fn build_and_load_model(
    path: &std::path::Path,
    device: &WgpuDevice,
) -> Result<Model<Backend>, Box<dyn std::error::Error>> {
    let model = Model::new(device);
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    let model = model.load_file(path, &recorder, device)?;
    Ok(model)
}

#[derive(Default)]
pub struct LowLight {
    model: Option<crate::model::Model<Backend>>,
    device: burn::backend::wgpu::WgpuDevice,
    input: Option<Tensor<Backend, 4>>,
    output: Option<Tensor<Backend, 4>>,
}

impl LowLight {
    pub fn new() -> Self {
        let device = WgpuDevice::default();
        init_sync::<AutoGraphicsApi>(&device, Default::default());
        LowLight {
            model: None,
            device,
            input: None,
            output: None,
        }
    }

    pub fn get_device(&self) -> &WgpuDevice {
        &self.device
    }

    pub fn load_model(&mut self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let model = build_and_load_model(path, self.get_device())?;
        self.model = Some(model);
        Ok(())
    }

    pub fn load_image(&mut self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let image = image::ImageReader::open(path)?.decode()?.into_rgb32f();
        let (width, height) = image.dimensions();
        let tensor = Tensor::<Backend, 3>::from_data(
            TensorData::new(image.into_raw(), [width as usize, height as usize, 3]),
            self.get_device(),
        );
        self.input = Some(tensor.permute([2, 0, 1]).unsqueeze::<4>());
        Ok(())
    }

    pub fn inference(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let input = self.input.take().unwrap();
        let (out, _) = self
            .model
            .clone()
            .map(|model| model.forward(input))
            .ok_or("推测失败!!!")?;
        self.output = Some(out);
        Ok(())
    }

    pub fn get_image(&mut self) -> Result<slint::Image, Box<dyn std::error::Error>> {
        let tensor = self.output.take().unwrap();
        let [_, _, width, height] = tensor.dims();
        let data = tensor
            .squeeze::<3>(0)
            .permute([1, 2, 0])
            .to_data()
            .to_vec::<f32>()
            .unwrap();
        let data: Vec<u8> = data.iter().map(|&x| (x * 255.0) as u8).collect::<Vec<u8>>();
        let mut buffer =
            slint::SharedPixelBuffer::<slint::Rgb8Pixel>::new(width as u32, height as u32);
        let buf: &mut [u8] = buffer.make_mut_bytes();
        buf.copy_from_slice(&data);
        let x = slint::Image::from_rgb8(buffer);
        Ok(x)
    }

    pub fn model_valid(&self) -> bool {
        self.model.is_some()
    }
}
