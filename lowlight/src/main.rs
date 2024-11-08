slint::include_modules!();
use lowlight_lib::lowlight::LowLight;
use opencv::core::Mat;
use opencv::highgui;
use slint::{ComponentHandle, Image};
use std::cell::RefCell;
use std::rc::Rc;

static IMAGE_TYPES: &[&str] = &["png", "jpg"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = App::new()?;
    let lowlight = Rc::new(RefCell::new(LowLight::new()));
    let mut camera = lowlight_lib::capture::Camara::default();

    // select image
    ui.on_select_image({
        let ui_handle = ui.as_weak();
        let lowlight = Rc::clone(&lowlight);
        move || {
            if !lowlight.borrow().model_valid() {
                let _btn = rfd::MessageDialog::new()
                    .set_title("Warnning")
                    .set_description("模型未加载")
                    .show();
            } else if let Some(path) = rfd::FileDialog::new()
                .add_filter("image", IMAGE_TYPES)
                .pick_file()
            {
                if path.is_file() {
                    {
                        let ui = ui_handle.unwrap();
                        let mut lowlight = lowlight.borrow_mut();
                        // 获取图像
                        lowlight.load_image(&path).unwrap();
                        lowlight.inference().unwrap();
                        if let Ok(out_image) = lowlight.get_image() {
                            ui.set_input_image(Image::load_from_path(path.as_path()).unwrap());
                            ui.set_output_image(out_image);
                            ui.set_top(false);
                        }
                    }
                }
            }
        }
    });

    ui.on_select_model({
        let ui_handle = ui.as_weak();
        let lowlight = Rc::clone(&lowlight);
        move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("model", &["mpk"])
                .pick_file()
            {
                if path.is_file() {
                    let ui = ui_handle.unwrap();
                    let mut lowlight = lowlight.borrow_mut();
                    lowlight.load_model(&path).unwrap();
                    ui.set_modelVaild(true);
                }
            }
        }
    });

    ui.on_paly({
        let ui_handle = ui.as_weak();
        let lowlight = Rc::clone(&lowlight);
        move || {
            let ui = ui_handle.unwrap();
            let mut lowlight = lowlight.borrow_mut();
            if lowlight.load_frame(&mut camera).is_ok() {
                lowlight.inference().unwrap();
                if let Ok(img) = lowlight.get_image() {
                    ui.set_playing(true);
                    ui.set_frame(img);
                }
            }
        }
    });

    ui.run()?;
    Ok(())
}
