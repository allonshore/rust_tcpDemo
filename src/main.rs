use std ::net::{TcpListener,TcpStream};
use std::thread;
use std::time;
use std::io;
use std::io::{Read,Write};

//处理接收字节流得函数
fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    //以填充一个长度为512字节buffer
    let mut buf = [0;512];
    //循环
    for _ in 0..1000{
        //获取请求字节流得长度
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0{
            return Ok(());
        }
        //回写字节流
        stream.write(&buf[..bytes_read])?;
        //间隔1秒
        thread::sleep(time::Duration::from_secs(1));

    }
    Ok(())
}
//主函数 执行得部分
fn main() -> io::Result<()>{
    //创建监听 绑定地址
    let listener = TcpListener::bind("127.0.0.1:1000").unwrap();
    //监听成功  绑定端口
    println!("Server listening on port {}",1000);

    //轮询接收得字节流请求
    for stream in listener.incoming() {
        //模式匹配错误得字节流或者错误
        match stream {
            //正确得情况
            Ok(stream) =>{
                //起一个新的thread进行连接处理
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
                });

            }
            // 匹配错误的case
            Err(e)=>{
                // 打印捕获的错误
                println!("Error : {}",e);
            }
        }
    }

    Ok(())


}
