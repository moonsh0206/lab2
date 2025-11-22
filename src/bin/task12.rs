use dbus::blocking::Connection;
use dbus_crossroads::Crossroads;
use dbus::channel::MatchingReceiver; // ★ 아까 이거 없어서 에러 났던 겁니다. 추가함!
use std::env;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return Ok(()); }
    
    let mode = &args[1];

    if mode == "server" {
        // 1. 서버 연결 및 이름 등록
        let conn = Connection::new_session()?;
        conn.request_name("com.example.lab2", false, true, false)?;
        
        // 2. "Hello" 메소드 만들기 (친구분 C코드와 동일 로직)
        let mut cr = Crossroads::new();
        let token = cr.register("com.example.lab2", |b| {
            b.method("Hello", (), ("reply",), |_, _, _: ()| {
                println!("서버: 누군가 'Hello'를 호출했습니다!"); 
                Ok(("서버: 그래, 반갑다! (응답 성공)",))
            });
        });
        cr.insert("/", &[token], ());
        
        println!("=== DBUS 서버 실행 중 ===");
        println!("클라이언트의 요청을 기다리고 있습니다...");
        
        // 3. 요청 처리 시작
        conn.start_receive(dbus::message::MatchRule::new_method_call(), Box::new(move |msg, conn| {
            // 들어오는 메시지를 Crossroads가 처리하도록 넘김
            cr.handle_message(msg, conn).unwrap();
            true
        }));

        // 무한 대기
        loop { conn.process(Duration::from_millis(1000))?; }
        
    } else if mode == "client" {
        // 서버가 켜질 때까지 1초 대기
        thread::sleep(Duration::from_millis(1000));

        let conn = Connection::new_session()?;
        
        // 서버 위치 찾기
        let proxy = conn.with_proxy("com.example.lab2", "/", Duration::from_secs(2));
        
        println!("클라이언트: 서버에 'Hello'라고 말해보겠습니다...");
        
        // "Hello" 함수 호출하고 대답 기다리기
        let (reply,): (String,) = proxy.method_call("com.example.lab2", "Hello", ())?;
        
        println!("---------------------------------------");
        println!("서버로부터 온 응답: {}", reply);
        println!("---------------------------------------");
    }
    Ok(())
}
