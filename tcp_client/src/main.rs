use std::io::{self, prelude::*, Write,BufReader};
use std::str;
use std::net::TcpStream;


fn main() -> io::Result<()> { 
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;        //创建一个连接到127.0.0.1:7878的stream(服务端地址)
    for _ in 0..5 {                                                //执行5次循环，可以发送5次请求
        let mut input = String::new();                             //创建input变量，用于存储从终端输入的数据
        io::stdin().read_line(&mut input).expect("Failed to read");//从终端输入读取数据
        stream.write(input.as_bytes()).expect("Failed to write");  //将数据写入stream

        let mut reader = BufReader::new(&stream);                  //创建一个读取
        let mut buffer: Vec<u8> = Vec::new();                      //创建一个用来接收数据的缓冲区
        
        reader                                                     //将stream中的数据放入缓冲区 
            .read_until(b'\n', &mut buffer)
            .expect("Failed to read into buffer");
        println!("read  from server: {}", str::from_utf8(&buffer).unwrap());//打印读取结果
        
        println!("*--------------------------------------------------*");
    }
    Ok(())
}
