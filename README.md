<img width="1910" height="995" alt="image" src="Screenshot 2026-03-26 000912.png" />


# Quỹ Học Bổng Đại Học

Contract Soroban tối giản cho dApp quỹ học bổng:

- Phát hành `scholarship token` nội bộ bằng `mint_scholarship`
- Theo dõi quyên góp minh bạch bằng `donate`
- Lưu từng khoản quyên góp để tra cứu bằng `donation(donation_id)`
- Phát event `init`, `mint`, `donate`

## Cấu trúc

- File contract duy nhất: `contracts/src/libs.rs`
- Tên crate: `hoc_bong`
- File wasm sau khi build: `target/wasm32v1-none/release/hoc_bong.wasm`

## Hàm chính

- `__constructor(admin: Address)`
- `mint_scholarship(to: Address, amount: i128)`
- `donate(donor: Address, amount: i128, note: String) -> u32`
- `balance(student: Address) -> i128`
- `total_supply() -> i128`
- `total_donated() -> i128`
- `donation_count() -> u32`
- `donation(donation_id: u32) -> DonationRecord`

## Build

```powershell
cd contracts
stellar contract build
```

## Deploy

```powershell
stellar contract deploy --wasm target\wasm32v1-none\release\hoc_bong.wasm --source-account nhat --network testnet -- --admin nhat
```

Lệnh trên sẽ deploy contract và gọi constructor với `admin = nhat`.

## Gợi ý invoke sau deploy

```powershell
stellar contract invoke --id <CONTRACT_ID> --source-account nhat --network testnet -- mint_scholarship --to <STUDENT_ADDRESS> --amount 1000
stellar contract invoke --id <CONTRACT_ID> --source-account nhat --network testnet -- donate --donor nhat --amount 5000000 --note "Ung ho quy hoc bong dot 1"
stellar contract invoke --id <CONTRACT_ID> --network testnet -- total_donated
stellar contract invoke --id <CONTRACT_ID> --network testnet -- donation --donation-id 1
```
