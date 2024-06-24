use nsfw::{model::Classification, Model};
use shellexpand::tilde;
use std::fs::File;

use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_NsfwPredictor_predict<'whatever>(
    mut env: JNIEnv,
    _jclass: JClass<'whatever>,
    filename: JString<'whatever>,
) -> bool {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let filename: String = env
        .get_string(&filename)
        .expect("Couldn't get string")
        .into();

    log::trace!("{:?}", &filename);

    let Some(image) = open_image(&filename) else {
        log::error!("Couldn't open_image(&filename)");
        return false;
    };

    let Some(model) = load_model() else {
        log::error!("Couldn't load_model()");
        return false;
    };

    let Some(classifications) = classif(&model, &image) else {
        log::error!("Couldn't classif(&model, &image)");
        return false;
    };

    estimate(&classifications)
}

fn open_image(filename: &str) -> Option<image::RgbaImage> {
    let filename = tilde(&filename);

    log::trace!("{:?}", &filename);

    let filename = match std::fs::canonicalize(std::borrow::Cow::as_ref(&filename)) {
        Ok(filename) => filename,
        Err(error) => {
            log::error!("{:?}", error);
            return None;
        }
    };

    let image = match image::open(filename) {
        Ok(image) => image,
        Err(error) => {
            log::error!("{:?}", error);
            return None;
        }
    };

    let image = image.to_rgba8();

    Some(image)
}

fn load_model() -> Option<Model> {
    let Ok(modelfile) = File::open("model.onnx") else {
        log::error!("Couldn't File::open(\"model.onnx\")");
        return None;
    };

    let Ok(model) = nsfw::create_model(modelfile) else {
        log::error!("Couldn't nsfw::create_model(modelfile)");
        return None;
    };

    Some(model)
}

fn classif(model: &Model, image: &image::RgbaImage) -> Option<Vec<Classification>> {
    let Ok(classifications) = nsfw::examine(model, image) else {
        log::error!("Couldn't nsfw::examine(model, image)");
        return None;
    };

    Some(classifications)
}

fn estimate(classifications: &[Classification]) -> bool {
    let mut normal_sum = 0.0;
    let mut abnormal_sum = 0.0;

    for classif in classifications {
        match &classif.metric {
            nsfw::model::Metric::Neutral => {
                log::trace!("Neutral: {}", classif.score);
                normal_sum += classif.score;
            }
            nsfw::model::Metric::Drawings => {
                log::trace!("Drawings: {}", classif.score);
                normal_sum += classif.score;
            }
            other_metric => {
                log::trace!("{other_metric:?}: {}", classif.score);
                abnormal_sum += classif.score;
            }
        }
    }

    normal_sum >= 10.0 * abnormal_sum
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{classif, estimate, load_model, open_image};

    fn bench<Ret>(fun: impl FnOnce() -> Ret) -> Ret {
        let timer = Instant::now();
        let ret = fun();
        println!(": {:?}", timer.elapsed());
        ret
    }

    #[test]
    fn model_benches() {
        print!("open_image");
        let Some(image) = bench(|| open_image("test-images/nsfw_image_2.jpeg")) else {
            return;
        };

        print!("load_model");
        let Some(model) = bench(load_model) else {
            return;
        };

        print!("classif");
        let Some(classifications) = bench(|| classif(&model, &image)) else {
            return;
        };

        print!("estimate");
        let estimation = bench(|| estimate(&classifications));
        println!("Estimated: {:?}", estimation);
    }
}
