use std::fs::{File, metadata};
use std::io::Read;
use std::io;

pub struct file 
{
    bytes: Vec<u8>,
  

}

impl file 
{
    pub fn new(filename: &str, chunks: i32) ->  io::Result<(file)>
    {

        
        let mut open_file = File::open(&filename).unwrap();
        let chunk_size = metadata(&filename).unwrap().len() as i32 / chunks;
        let mut buffer = vec![0; chunk_size as usize];
        let mut file_in_chunks = Vec::new();

        loop {
            match open_file.read(&mut buffer)
            {
                Ok(0) => break,
            Ok(n) => {
                
                file_in_chunks.extend_from_slice(&buffer[..n]);
                println!("test {:?}", file_in_chunks);

            
            }
                
                ,
                Err(_) =>  break,

            }            
        }  
        return Ok(file { bytes: file_in_chunks})
    }
    pub fn get_bytes(&self) -> &Vec<u8> 
    {
        &self.bytes
        
    }
}