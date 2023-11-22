use std::{fs, thread, time::Duration};

use super::LoadImages;

pub struct LocalImage {
    framerate: f32,
    path: String,
    img_read: usize,
}

impl Default for LocalImage {
    fn default() -> Self {
        Self {
            framerate: 15.0,
            path: "/home/carter/Documents/Internship/Images/daySequence2/daySequence2/frames"
                .into(),
            img_read: 0,
        }
    }
}

impl LoadImages for LocalImage {
    fn get_image(&mut self) -> image::DynamicImage {
        thread::sleep(Duration::from_secs_f32(
            (1.0 / 60.0) * (60.0 / self.framerate),
        ));
        let mut count = 0;

        // Read the contents of the folder
        if let Ok(entries) = fs::read_dir(self.path.clone()) {
            for entry in entries {
                if count == self.img_read {
                    //not the best but should work
                    if let Ok(entry) = entry {
                        let img = match image::open(entry.path()) {
                            Ok(image) => image,
                            Err(error) => {
                                println!("Image could not be read, {}", error);
                                continue;
                            }
                        };

                        self.img_read += 1;
                        if self.img_read % 100 == 0 {
                            println!("Images Read: {}", self.img_read);
                        }
                        return img;
                    }
                }
                count += 1;
            }

            println!("No Items left in the folder provided");
            thread::sleep(Duration::from_secs_f32(100000.0));
            panic!();
        } else {
            panic!("Folder could not be read");
        }
    }
}
