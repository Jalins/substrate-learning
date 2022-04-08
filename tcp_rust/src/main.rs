/*
主要使用std::net库下的TcpListener和TcpStream
*/

use std::net::{TcpListener, TcpStream};
use std::io::{self,Write,Read};
use std::str;

fn handle_client(mut stream: TcpStream){
    // 创建一个1024个字节长度的buf数组，用户存储读取到的数据流
    let mut buf = [0;1024];
    // 开启一个无限循环读取数据
    loop {
        // 将数据流读取存储到buf数组中
        let read_bytes = stream.read(&mut buf[..]).expect("读取失败");
        // 如果读取内容为空这说明数据已经接收完毕，那么就退出整个循环
        if read_bytes == 0 {
            println!("内容接收完毕");
            // 退出当前循环
            break;
        }

        // 向客户端返回输入的数据
        match  stream.write(&buf[..read_bytes]){
            // Result匹配到OK时
            Ok(v)=> {},
            // Result匹配到Err时
            Err(e) => println!("{}",e),
        };

        // 将byte[]转化为string，unwrap表示不处理错误，有错误直接panic，这里可以使用match模式匹配来处理错误
        let str_from_utf8 = str::from_utf8(&buf[..read_bytes]).unwrap();
        // 打印一下接收到的数据
        println!("接收到的数据为：{}", str_from_utf8);
    }
}

fn main() -> io::Result<()>{
    println!("正在启动一个tcp服务");
    // 开启一个监听器，绑定本地ip与8080端口
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    // 接受连接并处理它们
    for stream in listener.incoming(){
        // 使用模式匹配进行错误处理
        match stream {
            // Result<TcpStream> 匹配到OK时
            Ok(stream) => {
                // 接收客户端请求并进行具体的操作
                handle_client(stream);
            }
            // Result<TcpStream> 匹配到Err时
            Err(e) => {
                // 使用占位符打印err
                println!("连接异常{:?}",e);
            }
        }
    }
    // 关闭tcp连接
    drop(listener);
    Ok(())
}
