# SPEC: TỰ ĐỘNG KIỂM TRA & CẬP NHẬT PHIÊN BẢN (AUTO-UPDATE) CHO CROSS-KVM

**Task ID:** TASK-20261122-AUTOUPDATE  
**Tác giả:** @phan-tich  
**Trạng thái:** DRAFT / PENDING_REVIEW  
**Phiên bản:** 1.0  

---

## 1. MỤC TIÊU & BỐI CẢNH
- Khi có bản phát hành (Release / Tag) mới trên GitHub repo `kipkip11/cross_kvm`:
  - Ứng dụng client & server (macOS / Windows) tự động phát hiện phiên bản mới.
  - Thông báo cho người dùng hoặc tự động tải về file thực thi tương ứng (`cross_kvm` trên macOS hoặc `cross_kvm.exe` trên Windows).
  - Thay thế an toàn file binary hiện tại và khởi động lại dịch vụ.

---

## 2. THIẾT KẾ KIẾN TRÚC KỸ THUẬT
1. **GitHub Releases Polling / Check:**
   - Sử dụng GitHub REST API: `GET https://api.github.com/repos/kipkip11/cross_kvm/releases/latest`.
   - Đối chiếu trường `tag_name` (ví dụ `v1.0.1`) với hằng số `env!("CARGO_PKG_VERSION")`.
   - Chu kỳ kiểm tra: Khi ứng dụng vừa khởi động và định kỳ mỗi 6 giờ ngầm.

2. **Cơ chế Tải về & Cài đặt an toàn (In-place Replacement):**
   - Xác định OS và kiến trúc CPU hiện tại (macOS aarch64/x86_64 hoặc Windows x86_64).
   - Tìm kiếm asset đính kèm tương ứng trên Release.
   - Tải file về thư mục tạm (`tempfile` / `.download`).
   - Kiểm tra SHA-256 Checksum (nếu có) để chống giả mạo file.
   - **Thay thế binary:**
     - Trên **macOS / Linux**: Ghi đè file binary hiện tại và cấp quyền thực thi `chmod +x`.
     - Trên **Windows**: File binary đang chạy bị khóa không thể ghi đè trực tiếp -> sử dụng kỹ thuật rename sang `.old` hoặc script thay thế tự động trước khi restart.

3. **Giao tiếp KVM Peer-to-Peer:**
   - Khi Server phát hiện có bản update mới, Server có thể gửi gói tin `Packet::UpdateAvailable { version, url }` qua socket LAN để thông báo tức thời cho máy Client đối diện cùng cập nhật.

---

## 3. ACCEPTANCE CRITERIA (AC)
- **AC-1:** Khi có release mới trên GitHub, app log thông báo rõ ràng: phiên bản hiện tại vs phiên bản mới.
- **AC-2:** Tải đúng file binary theo hệ điều hành (macOS hoặc Windows).
- **AC-3:** Cơ chế thay thế file không làm hỏng app nếu quá trình tải bị đứt quãng giữa chừng (Atomic Swap).
- **AC-4:** Khi không có mạng hoặc GitHub API bị rate-limit, app vẫn hoạt động bình thường, không crash (Graceful degradation).
