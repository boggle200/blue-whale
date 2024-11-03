use std::{
    net::{TcpListener, TcpStream},
    io::{BufRead, BufReader, Write},
    thread,
    sync::mpsc,
    time::Duration,
};

pub fn run() {
    let server_addr = "127.0.0.1:8888";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(server_addr).expect("서버 실행 실패");
    server.set_nonblocking(true).expect("알 수 없는 에러");
    println!("{}에서 서버가 실행 중입니다.", server_addr);

    loop {
        if let Ok((client, addr)) = server.accept() {
            println!("클라이언트 접속: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        if let Ok(msg) = rx.try_recv() {
            println!("전원에게 보내기 : {}", msg.trim());
            clients = send_all(clients, &msg);
            
            // 랜드마크 값을 출력
            if let Ok(landmarks) = parse_landmarks(&msg) {
                match landmarks.as_slice() {
                    [Some(left), Some(right)] => {
                        println!("왼손 랜드마크: {:?}", left);
                        println!("오른손 랜드마크: {:?}", right);
                    }
                    _ => {
                        println!("손 랜드마크가 감지되지 않았습니다.");
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

pub fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            if n > 0 { 
                tx.send(msg).unwrap(); 
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

pub fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];

    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("전송 에러 : {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector
}

pub fn parse_landmarks(msg: &str) -> Result<[Option<Vec<Vec<f32>>>; 2], &'static str> {
    // msg에서 왼손과 오른손 랜드마크를 파싱
    let trimmed = msg.trim();
    if trimmed.is_empty() {
        return Ok([None, None]);
    }

    let parts: Vec<&str> = trimmed.split("],").collect();
    if parts.len() != 2 {
        return Err("잘못된 형식");
    }

    let left_hand = parts[0].replace("[", "").replace("]", "");
    let right_hand = parts[1].replace("[", "").replace("]", "");

    let left_hand_landmarks = if left_hand.is_empty() {
        None
    } else {
        Some(parse_landmark_list(&left_hand))
    };

    let right_hand_landmarks = if right_hand.is_empty() {
        None
    } else {
        Some(parse_landmark_list(&right_hand))
    };

    Ok([left_hand_landmarks, right_hand_landmarks])
}

pub fn parse_landmark_list(data: &str) -> Vec<Vec<f32>> {
    data.split("],")
        .map(|s| {
            s.split(',')
                .filter_map(|x| x.trim().parse::<f32>().ok())
                .collect()
        })
        .collect()
}
