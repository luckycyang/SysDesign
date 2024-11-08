use lowlight_lib::lowlight::LowLight;
use opencv::highgui;
use slint::{Timer, TimerMode};

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut camera = lowlight_lib::capture::Camara::default();
    let mut model = LowLight::new();
    let app = App::new()?;
    model.load_model(std::path::Path::new(
        "/home/dingduck/Public/rs/SysDesign/model.mpk",
    ))?;

    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(1), {
        let app = app.as_weak().unwrap();
        move || {
            model.load_frame(&mut camera).unwrap();
            model.inference().unwrap();
            if let Ok(img) = model.get_image() {
                app.set_frame(img);
            }
        }
    });
    app.run()?;
    Ok(())
}
