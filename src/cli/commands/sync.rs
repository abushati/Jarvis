use super::utils::{CLICommand,CliAction};
use std::fs::{read_dir,read,OpenOptions,write};
use std::io::Write;
use std::path;
use std::str::from_utf8;
use image;
use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use encoding_rs::{Encoding, UTF_16LE};
use reqwest::blocking::Client;



use image::{ColorType, GenericImageView, ImageFormat};
#[derive(Default)]

/*why are the bytes returned from image open different then file read

The bytes returned from Image open and File read may differ due to the encoding and/or compression used by the image file format.

When you open an image file using an image library like image in Rust, it is typically decoded into an uncompressed bitmap representation in memory. This is done to make it easier to manipulate the image data, for example, to resize or modify the image.

On the other hand, when you read a file using the std::fs::File API, you get the raw byte data of the file without any decoding or decompression. This means that if the image file is compressed or encoded in a way that is not natively supported by Rust, you may not be able to manipulate the image data until you decode it into an uncompressed bitmap representation.

Therefore, it's important to choose the appropriate approach depending on the task at hand. If you need to manipulate the image data, it's best to use an image library like image to open and decode the image file. If you just need to read the raw byte data of a file without any modification, using std::fs::File would be appropriate.




Arvid Bushati
how can i get the encoding of a file
To get the encoding of a file, you can use a Rust library like encoding_rs. Here's an example code snippet that shows how to detect the encoding of a file:

rust
Copy code
use std::fs::File;
use std::io::Read;
use encoding_rs::{Encoding, UTF_8};

fn main() {
    let file_path = "/path/to/file";
    let mut file = File::open(file_path).expect("Failed to open file");

    // Read the entire contents of the file into a Vec<u8>
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Failed to read file");

    // Detect the encoding of the file
    let (encoding, _) = Encoding::for_bom(&contents).unwrap_or((UTF_8, 0));

    println!("The file encoding is: {}", encoding.name());
}
In this example, we first open the file using File::open and read its contents into a Vec<u8> using read_to_end. We then use the for_bom method of the Encoding struct from the encoding_rs library to detect the encoding of the file. The for_bom method returns a tuple containing the detected encoding and the length of the byte order mark (BOM), if present. If no BOM is present, it returns a default encoding (in this case, UTF-8).

We then print the name of the detected encoding using the name method of the Encoding struct.




Arvid Bushati
can you save the last two responses
Sure, here are the last two responses:

Q: Why are the bytes returned from image open different than file read?

A: The bytes returned from image::open may be different from those returned by File::read because image::open reads and decodes the image data, whereas File::read just reads the raw binary data of the file. The decoded image data may have a different format or encoding than the raw binary data, depending on the image format and compression used.

Q: How can I get the encoding of a file?

A: In Rust, there is no straightforward way to determine the encoding of a file with 100% accuracy, as the same sequence of bytes can be interpreted differently depending on the encoding used. However, there are libraries such as encoding_rs that can make an educated guess at the encoding by analyzing the byte sequence and looking for common patterns. The encoding_rs library provides a decode function that attempts to decode a byte sequence using a specific encoding, and returns the decoded string and the number of bytes consumed. By trying different encodings in order of likelihood, you can often find the correct encoding for a given file.




Arvid Bushati
can you download it for me as a txt file
I'm sorry, but as an AI language model, I don't have the capability to download files. However, you can easily copy and paste the previous responses into a text editor and save them as a text file on your local machine.




Arvid Bushati
okay can you rewrite it in a markdown language 
Sure! Here are the previous two responses in markdown format:

To make the function generic, you can accept the file path, the output format, and the output path as arguments. Then, you can use a match statement to determine the format of the input file and use the corresponding method to open it with the image crate. Here's an example implementation:

rust
Copy code
use image::{GenericImageView, ImageBuffer, ImageFormat};
use std::fs::{File, OpenOptions};
use std::io::Write;

fn convert_image_format(input_path: &str, output_format: ImageFormat, output_path: &str) {
    let input_file = File::open(input_path).expect("Failed to open input file");
    let image = image::guess_format(&input_file)
        .expect("Failed to guess image format")
        .read_from(&mut std::io::BufReader::new(input_file))
        .expect("Failed to read image data");

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)
        .expect("Failed to create output file");
    let mut encoder = output_format.encoder();
    encoder
        .write_image(&image, &mut output_file)
        .expect("Failed to write output file");
}
With this implementation, you can convert any image file to any format by passing in the input file path, the output format, and the output file path.

Here's an example implementation of a function that detects the encoding of a file:

rust
Copy code
use encoding_rs::Encoding;
use std::fs::File;
use std::io::Read;

fn detect_encoding(file_path: &str) -> Option<Encoding> {
    let mut file = File::open(file_path).ok()?;
    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer).ok()?;
    Encoding::for_bytes(&buffer[..bytes_read]).0
}
This function uses the encoding_rs crate to detect the encoding of a file. It takes the file path as an argument, opens the file, reads the first 1024 bytes into a buffer, and uses the Encoding::for_bytes() function to detect the encoding of the buffer. If successful, it returns the detected encoding as an Option<Encoding>. If unsuccessful, it returns None.

You can use this function to detect the encoding of any file by passing in the file path as an argument.


Yes, there are more robust ways to determine if a file is an image or not without relying solely on the file extension. One way is to look at the file signature or "magic number" of the file, which is a sequence of bytes at the beginning of the file that identifies its type. For example, the magic number for a JPEG file is 0xFFD8FF.

You can use the magic crate in Rust to determine the file type using the magic number. Here is an example:

rust
Copy code
use magic::flags::{MIME_TYPE, MIME_ENCODING};
use magic::{Cookie, CookieFlags};

fn is_image(file_path: &str) -> bool {
    let cookie_flags = CookieFlags::MIME_TYPE | CookieFlags::MIME_ENCODING;
    let cookie = Cookie::open(cookie_flags).unwrap();
    let mime_type = cookie
        .get_mime_type(file_path)
        .unwrap_or("application/octet-stream".to_string());
    mime_type.starts_with("image/")
}
This code uses the magic::Cookie struct to open a new magic cookie with the MIME_TYPE and MIME_ENCODING flags set. The get_mime_type method of the cookie is then used to get the MIME type of the file, which is checked to see if it starts with the "image/" prefix. If it does, then the file is an image.

Note that the magic crate requires the installation of the libmagic library on your system.


 */

//Todo: add enum for type
pub struct sync_cmd {
    type_arg: String,
    path: String
}
impl CLICommand for sync_cmd {
    fn run(&self) {
        // let i = vec!["d","ds"].contains(x);
        let e = format!("Hello from run of sync type {:?}, path: {:?}",&self.type_arg, &self.path);
        println!("{}",e);
        if ["d","directory"].contains(&self.type_arg.as_str()) {
            let path = path::Path::new(&self.path);
            let i = read_dir(path);
            match i {
                Ok(..) => {
                    for s in i.unwrap() {
                        let entry = s.unwrap();
                        let file_name = entry.file_name();
                        let file_path = entry.path();
                        let file_meta = entry.metadata().unwrap();
                        let input_image = image::open(&file_path).unwrap();
                        let formater = image::ImageFormat::from_path(&file_path).unwrap();
                        println!("Status: {:?}", &input_image.as_bytes());
                        let (width, height) =input_image.dimensions();
                        println!("Status: {},{}",width,height);
                        let mut output_path = "/Users/arvidbushati/Desktop/Projects/Jarvis/here.jpg";
                        let client = Client::new();
                        let post = client.post("http://127.0.0.1:8080/upload_file_data/adfasf")
                        .body(input_image.clone().into_bytes())
                        .send().unwrap();
                        println!("Status: {}", post.status());
                        // image::save_buffer_with_format(
                        //     &mut output_path,
                        //     &input_image.into_bytes(),
                        //     width,
                        //     height,
                        //     ColorType::Rgb8,
                        //     formater,
                        // ).unwrap();
                        

                        // let mut output_file = OpenOptions::new().write(true).truncate(true).create(true).open(output_path).expect("Failed to create output file");
                        // output_file.write_all(&string.as_bytes()).expect("Failed to write output file");
                    
                        println!("Successfully created file at {}", output_path);

                        // println!("{:?},{:?},{:?}, {:?}",file_name,&file_path,file_meta, formater);
        
                    }
                }
                Err(e) => {println!("what is u doing? {:?}",e)}
            }
            
        }
        
    }
    fn get_cmd (&self, args: Vec<String>) -> Result<CliAction,String> {
        let type_arg = args.get(2).unwrap().to_string();
        let path = args.get(3).unwrap().to_string();
        let cmd = sync_cmd{type_arg:type_arg,
                                    path:path};
        Ok(CliAction{cmd:Box::new(cmd)})
    
    }
}

