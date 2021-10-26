use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::time;
use std::thread;



//This is the handle function which can delt the request from client
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];                                 //创建一个缓冲区
    for _ in 0..100 {                                       //我们可以从stream中读取100次
        let bytes_read = stream.read(&mut buf)?;            //从stream中读取数据
        if bytes_read == 0 {                                //如果没有从stream中读取出来数据，就返回
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;                  //将读取的内容写入stream
        thread::sleep(time::Duration::from_secs(1));        //等待一秒钟
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;             //创建一个tco socket server 监听127.0.0.1:7878的连接
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();    //创建一个vec来存储线程。
    for stream in listener.incoming() {                              //处理来自tcp服务器的连接传入的数据
        let stream = stream.expect("failed");                        //创建一个线程来处理来自客户端的请求并返回线程处理程序
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        thread_vec.push(handle);                                     //将线程处理程序写入到thread_vec
    }
    for handle in thread_vec {                                       //等待所有的线程关闭，若有错误进行错误处理
        match handle.join() {
            Ok(_) => {                                               //如果线程正常关闭
                println!("Close the thread success!");
            }
            Err(_) => {                                              //如果线程关闭异常 
                println!("Some error when close the thread!");
            }
        }
    }
    Ok(())
}
