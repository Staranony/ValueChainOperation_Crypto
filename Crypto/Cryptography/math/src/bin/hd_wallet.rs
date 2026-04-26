use sha2::{Sha256, Digest};

// [지갑 구조체] 마치 주민등록증 발급기 같은 역할
struct HDWallet{
    master_seed: String,
}

impl HDWallet {
    // 1. 지갑 생성 (마스터 시드 초기화)
    fn new(seed: &str) -> Self{
        HDWallet {
            master_seed: seed.to_string(),
        }
    }


    // 2. 자식 키 생성 (Derivation)
    // 원리: Hash(마스터 시드 + 순서 번호) = 자식 비밀키
    fn derive_child_key(&self, index: u32) -> String {
        let mut hasher = Sha256::new();
        // 부모의 유전자(Seed)에 자식의 순서(Index)를 섞음
        let input_data = format!("{}{}", self.master_seed, index);
        hasher.update(input_data);

        hex::encode(hasher.finalize())
    }
}

fn main() {
    println!("--- [HD 지갑 시뮬레이션] ---");
    // 1. 딱 하나의 마스터 시드만 기억하면 됨 (이것만 백업!)
    let my_master_seed = "iron_man_secret_password_1234";
    let my_wallet = HDWallet::new(my_master_seed);

    println!("마스터 시드(뿌리): {}", my_master_seed);
    println!("----------------------------------------------------------------");

    // 2. 상황극: 월급날, 친구에게 돈 받을 때, 편의점 갈 때
    // 매번 다른 주소를 쓰지만, 사실은 다 내 지갑임.

    // 첫 번째 자식 (월급 통장용)
    let child_key_1 = my_wallet.derive_child_key(1);
    println!("[거래 1] 월급 수령용 주소 (Index 1):");
    println!("-> {}", child_key_1);

    // 두 번째 자식 (친구 송금용)
    let child_key_2 = my_wallet.derive_child_key(2);
    println!("\n[거래 2] 친구에게 받을 주소 (Index 2):");
    println!(" -> {}", child_key_2);

    let child_key_3 = my_wallet.derive_child_key(3);
     println!("\n[거래 3] 치킨 사먹을 주소 (Index 3):");
     println!("-> {}", child_key_3);

     println!("--------------------------------------------------------------------");
     println!("[해커의 시선]");
     println!("해커: '이 주소 3개는 서로 완전히 다르게 생겼는데?'");
     println!("해커: 'Index 1이 치킨을 사 먹었는지 알 수가 없네!' (추적 실패 ❌)");

     println!("\n[나의 시선]");
     println!("나: '핸드폰 잃어버려도 마스터 시드만 있으면 위 3개 키를 다시 계산해날 수 있어! (복구 가능 ✅)");
    
}