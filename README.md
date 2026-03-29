
# Title
Quỹ Học Bổng Đại Học (University Scholarship Fund)

# Description
Dự án này xây dựng một ứng dụng phân tán (dApp) cho quỹ học bổng dùng smart contract Soroban trên mạng Stellar. Mục đích giúp cấp bậc học quản lý quyên góp minh bạch, phát hành học bổng dạng token nội bộ, và theo dõi lịch sử quyên góp một cách an toàn trên blockchain. Ý tưởng này giải quyết vấn đề tin cậy và minh bạch trong quản lý quỹ học bổng.

# Tính năng
- **Phát hành học bổng**: Quản trị viên có thể mint scholarship tokens cho sinh viên thông qua `mint_scholarship`
- **Quyên góp minh bạch**: Ghi lại từng khoản quyên góp với lời nhắn bằng hàm `donate`
- **Tra cứu lịch sử**: Lưu trữ và truy vấn từng bản ghi quyên góp bằng `donation(donation_id)`
- **Thống kê**: Cung cấp hàm lấy tổng số tiền quyên góp, tổng số token phát hành, số lượng quyên góp
- **Sự kiện**: Phát hành event `init`, `mint`, `donate` để theo dõi hoạt động

# Contract
**Địa chỉ Contract (Testnet):**  
`https://stellar.expert/explorer/testnet/contract/CBTWARZRMWPISS5NX33EUEIRPH6BW7EDKI4S64GJZBWNYORMIU22OS5S
**Ảnh chụp màn hình:**  
<img width="1910" height="995" alt="image" src="Screenshot 2026-03-26 000912.png" />

**Chi tiết kỹ thuật:**
- File contract: `contracts/src/lib.rs`
- Tên crate: `hoc_bong`
- WASM build: `target/wasm32v1-none/release/hoc_bong.wasm`

**Các hàm chính:**
- `__constructor(admin: Address)` - Khởi tạo contract
- `mint_scholarship(to: Address, amount: i128)` - Phát hành học bổng
- `donate(donor: Address, amount: i128, note: String) -> u32` - Ghi nhận quyên góp
- `balance(student: Address) -> i128` - Kiểm tra số dư
- `total_supply() -> i128` - Tổng token phát hành
- `total_donated() -> i128` - Tổng tiền quyên góp
- `donation_count() -> u32` - Số lượng quyên góp
- `donation(donation_id: u32) -> DonationRecord` - Chi tiết quyên góp

**Build & Deploy:**
```powershell
# Build contract
cd contracts
stellar contract build

# Deploy to Testnet
stellar contract deploy --wasm target\wasm32v1-none\release\hoc_bong.wasm --source-account nhat --network testnet -- --admin nhat
```

# Future Scopes
- **Mở rộng quản lý**: Hỗ trợ nhiều quỹ học bổng độc lập, phân quyền chi tiết cho cán bộ quản lý
- **Integrations**: Kết nối với ví Freighter, xây dựng giao diện Frontend hoàn chỉnh
- **Tính năng nâng cao**: Thêm chức năng hoàn lại tiền kế hoạch, phê duyệt tự động, báo cáo tổng hợp
- **Mở rộng blockchain**: Triển khai trên Mainnet khi hoàn thiện, tạo chuỗi quản lý quỹ học bổng theo mô hình quốc gia
- **Cộng đồng**: Xây dựng ứng dụng mã nguồn mở cho các trường đại học khác tham gia

# Profile
**Nickname/Tên:** nsn
**Kỹ năng:**
- Rust smart contract development
- Soroban SDK v22
- Stellar network & CLI
- Blockchain dApp architecture
- Smart contract testing & deployment

