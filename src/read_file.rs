use std::fs::{File, metadata};
use std::io::Read;
use std::io;

pub struct file 
{
    bytes: Vec<Vec<u8>>,
  

}

impl file 
{
    pub fn new(filename: &str, chunks: i32) ->  io::Result<(file)>
    {

        
        let mut open_file = File::open(&filename).unwrap();
        let chunk_size = metadata(&filename).unwrap().len() as i32 / chunks;
        let mut buffer = vec![0; chunk_size as usize];
        let mut chunks = Vec::new();

        

        loop {
            match open_file.read(&mut buffer)
            {
                Ok(0) => break,
            Ok(n) => {
                
                let chunk = buffer[..n].to_vec();
                chunks.push(chunk);
                
            
            }
                
                ,
                Err(_) =>  break,

            }            
        }  
        return Ok(file { bytes: chunks})
    }
    pub fn get_bytes(&self) -> &Vec<Vec<u8>> 
    {
        &self.bytes
        
    }
}