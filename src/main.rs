use std::process::{Command, Child};
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::io::{self, Read};

fn main() -> std::io::Result<()> {
    let project_path = Path::new(r"./landmark");
    let mut processes: Vec<Child> = Vec::new();
    
    // cargo run 명령어를 실행하는 cmd 창 열기
    let cargo_cmd = Command::new("cmd")
        .args(["/C", "start", "cmd", "/K", 
            &format!("cd /d {} && cargo run", project_path.to_str().unwrap())])
        .spawn()?;
    processes.push(cargo_cmd);

    // cargo run이 시작될 때까지 잠시 대기
    thread::sleep(Duration::from_secs(2));

    // python 스크립트를 실행하는 cmd 창 열기
    let python_cmd = Command::new("cmd")
        .args(["/C", "start", "cmd", "/K", 
            &format!("cd /d {} && python py_client.py", project_path.to_str().unwrap())])
        .spawn()?;
    processes.push(python_cmd);

    println!("모든 프로세스가 시작되었습니다.");
    println!("Enter를 누르면 모든 프로세스가 종료됩니다...");

    // Enter 키 입력 대기
    let _ = io::stdin().read(&mut [0u8]).unwrap();

    println!("종료 신호를 감지했습니다. 프로세스들을 종료합니다...");
            
    // Windows에서 실행 중인 모든 관련 프로세스 강제 종료
    Command::new("taskkill")
        .args(["/F", "/IM", "cargo.exe"])
        .spawn()?;
    Command::new("taskkill")
        .args(["/F", "/IM", "python.exe"])
        .spawn()?;
    Command::new("taskkill")
        .args(["/F", "/IM", "cmd.exe"])
        .spawn()?;

    // 프로세스 벡터의 모든 프로세스 종료
    for mut process in processes {
        let _ = process.kill();
    }

    println!("프로그램이 정상적으로 종료되었습니다.");
    Ok(())
}
