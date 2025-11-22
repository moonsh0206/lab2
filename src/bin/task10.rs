use std::ffi::CString;
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::thread;
use std::os::raw::{c_int, c_long, c_void};

// 리눅스 시스템 콜 사용
const KEY: i32 = 9999; // 메시지 큐 키값
const MAX_SIZE: usize = 1024;

#[repr(C)]
struct MsgBuf {
    mtype: c_long,
    mtext: [i8; MAX_SIZE],
}

extern "C" {
    fn msgget(key: c_int, msgflg: c_int) -> c_int;
    fn msgsnd(msqid: c_int, msgp: *const c_void, msgsz: usize, msgflg: c_int) -> c_int;
    fn msgrcv(msqid: c_int, msgp: *mut c_void, msgsz: usize, msgtyp: c_long, msgflg: c_int) -> isize;
}

fn main() {
    println!("=== [과제 10] 메시지 큐 채팅 프로그램 ===");
    println!("채팅방에 입장했습니다. 메시지를 입력하세요.");
    
    // 1. 큐 생성 (없으면 만들고 있으면 가져옴)
    let msgid = unsafe { msgget(KEY, 0o666 | 0o1000) }; 
    let msgid = if msgid == -1 {
        unsafe { msgget(KEY, 0o666) }
    } else {
        msgid
    };

    if msgid == -1 {
        panic!("메시지 큐 생성 실패! (리눅스 문제)");
    }

    // 2. 수신 스레드 (상대방 말 듣기)
    thread::spawn(move || {
        loop {
            let mut msg: MsgBuf = unsafe { mem::zeroed() };
            // 아무 메시지나 기다림 (mtype = 0)
            let len = unsafe { msgrcv(msgid, &mut msg as *mut _ as *mut c_void, MAX_SIZE, 0, 0) };
            
            if len > 0 {
                let text = unsafe { CString::from_raw(msg.mtext.as_mut_ptr()) };
                let s = text.to_string_lossy();
                
                // 내가 보낸 게 아닐 때만 출력하면 좋겠지만, 간단히 다 출력
                if !s.is_empty() {
                     print!("\r[상대방]: {}\n나: ", s);
                     io::stdout().flush().unwrap();
                }
                mem::forget(text); 
            }
        }
    });

    // 3. 송신 루프 (내가 말하기)
    loop {
        print!("나: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() { continue; }

        let mut msg: MsgBuf = unsafe { mem::zeroed() };
        msg.mtype = 1; 
        
        let c_str = CString::new(input).unwrap();
        let bytes = c_str.as_bytes_with_nul();
        
        if bytes.len() <= MAX_SIZE {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), msg.mtext.as_mut_ptr() as *mut u8, bytes.len());
                // 메시지 전송
                msgsnd(msgid, &msg as *const _ as *const c_void, MAX_SIZE, 0);
            }
        }
    }
}
