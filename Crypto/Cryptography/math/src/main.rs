use rand::Rng; // 주사위 굴리는 도구
use sha2::{Sha256, Digest}; // 데이터를 암호화하는 도구

fn main() {
    println!("--- [개념 1] 균등 분포 (완벽한 주사위로 비밀키 만들기) ---");
    let private_key = generate_secure_key();
    println!("생성된 비밀키 (Hex): {}", hex::encode(private_key));

    println!("\n--- [개념 2] 확률적 알고리즘 (주사위 굴려서 황금 캔 찾기 / 채굴) ---");
    let data = "내 지갑에서 니 지갑으로 100 BTC 전송!";
    mine_block(data, 4); // 난이도 2 (앞자리 00 찾기)
}

// [개념 1 구현] 균등 분포: 예측 불가능한 256비트(32바이트) 비밀키 생성
fn generate_secure_key() -> [u8; 32] {
    let mut rng = rand::thread_rng(); // 운영체제의 보안 난수 생성기를 가져옴 (공평한 주사위)
    let mut key = [0u8; 32]; // 32칸짜리 빈 상자 준비
    rng.fill(&mut key); // 상자를 랜덤한 숫자로 가득 채움(0~255 사이의 숫자가 균등하게 분포됨)
    // 만약 여기서 'rand'가 엉터리라면(특정 숫자가 더 잘나온다면), 해커가 키를 예측할 수 있음!
    key
}

// [개념 2 구현] 확률적 알고리즘: 입력값 + 랜덤값(Nonce) = 결과가 매번 달라짐
fn mine_block(data: &str, difficulty: usize) {
    let mut nonce = 0; // 우리가 굴릴 주사위 값 (0부터 시작해서 계속 바꿈)
    let target = "0".repeat(difficulty); // 목표: 해시값이 "00"으로 시작해야함
    
    loop {
        // 1. 입력 데이터와 주사위 값(Nonce)을 합침\
        let input = format!("{}{}", data, nonce);

        // 2. 해시 함수 통과 (결과를 예측할 수 없음)
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let hash_string = hex::encode(result);

        // 3. 조건 확인 (원하는 결과가 나왔나?)
        if hash_string.starts_with(&target) {
            println!("성공! 주사위를 {}번 굴려서 찾았습니다.", nonce);
            println!("해시값: {}", hash_string);
            break; // 찾았으니 종료
        }

        // 실패하면 주사위 값을 바꾸고 다시 시도 (이게 바로 확률적 알고리즘의 핵심)
        nonce += 1;
    }
}