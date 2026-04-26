use sha2::{Sha256, Digest};

fn main() {
    // 1. 테스트 데이터:패턴이 아주 뚜렷한 문장 (AAAA... 같은 느낌)
    // 'E'가 가장 많이 나오고, 'L'이 두 번 연속 반복되는 패턴이 있음
    let input = "HELLO HELLO HELLO";

    println!("--- [실험 1] 고전 암호 (치환 암호) ---");
    println!("원본 데이터: {}", input);

    // 3칸씩 뒤로 무리는 '시저 암호' 적용
    let weak_encrypted = caesar_cipher(input, 3);
    println!("암호화 결과: {}", weak_encrypted);

    println!("\n[분석]");
    println!("원본의 'HELLO'가 매번 '{}'로 똑같이 변환됨.", "KHOOR");
    println!("-> 해커 왈: 'O'가 계속 나오네? 이거 원래 'L'이었겠구나! (빈도 분석 가능 🚨)");


    println!("\n--------------------------------------------");

    println!("--- [실험 2] 블록체인 기술 (SHA-256 해시) ---");

    // 똑같은 데이터를 해시 함수에 넣음
    let hash_result1 = get_hash(input);
    println!("원본 데이터: {}", input);
    println!("해시 결과 1: {}", hash_result1);


    // 2-1. 쇄도 효과(Avalanche Effect) 테스트
    // 원본에서 딱 한 글자만 바꿔봄 (마지막 'O' -> '!')
    let input_modified = "HELLO HELLO HELL!";
    let hash_result2 = get_hash(input_modified);

    println!("\n[쇄도 효과 태스트] 원본에서 글자 하나만 바꿈 ('O' -> '!')");
    println!("수정 데이터: {}", input_modified);
    println!("해시 결과 2: {}", hash_result2);

    println!("\n[분석]");
    println!("1. 결과값 길이는 64자로 똑같음.");
    println!("2. 하지만 결과 1과 결과 2는 완전히 다르게 생김.");
    println!("-> 해커 왈: 입력값이 아주 조금 바뀌었는데 결과가 뒤집어졌네? 패턴을 못 찾겠다! (빈도 분석 불가 🔒)");

}

// [도구 1] 고전 암0호 함수 (글자를 shift만큼 이동)
fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_uppercase() {b'A'} else {b'a'};
                // (현재 글자 + shift) % 26 계산
                let shifted = (c as u8 - first + shift) % 26 + first;
                shifted as char
            } else {
                c // 공백이나 특수문자는 그대로 둠
            }
            
        })
        .collect()
}
// [도구 2] 블록체인 해시 함수 (SHA-256)
fn get_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    hex::encode(hasher.finalize())
}