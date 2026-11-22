# REVIEW: KỸ THUẬT & AN TOÀN CHO DỰ ÁN CROSS-KVM

**Task ID:** TASK-20261122-KVM  
**Người đánh giá:** @phan-bien  
**Tài liệu rà soát:** `SPEC.md` v1.0  
**Kết luận phản biện:** `APPROVE_WITH_CONDITIONS` (Đồng ý triển khai nếu tuân thủ 3 yêu cầu an toàn bắt buộc)

---

## 1. ĐÁNH GIÁ RỦI RO & BẢO MẬT (SECURITY AUDIT)

| Mã | Phát hiện | Mức độ | Tác động | Giải pháp bắt buộc |
|---|---|---|---|---|
| **REV-01** | Bắt & truyền keystroke qua mạng LAN | **Critical** | Nếu không mã hóa, kẻ tấn công trên cùng mạng Wi-Fi có thể bắt trộm mật khẩu/token người dùng gõ. | Bắt buộc mã hóa luồng socket bằng TLS 1.3 hoặc ChaCha20-Poly1305 với Pre-shared Key (PSK) hoặc cặp khóa Ephemeral. |
| **REV-02** | Xung đột với Barrier / Input-Leap đang chạy | **Medium** | Barrier đang chiếm port `24800` và giữ hook CGEventTap, có thể gây đơ chuột hoặc drop event. | Tách biệt port mặc định sang dải khác (ví dụ `24900`), kiểm tra xung đột trước khi bind. |
| **REV-03** | Tràn bộ nhớ (OOM) khi Copy-Paste File lớn | **High** | Khi copy file dung lượng vài GB, nếu đọc toàn bộ vào RAM sẽ làm crash ứng dụng. | Cơ chế truyền file phải là **Streaming theo Chunks** (ghi trực tiếp xuống đĩa ở máy nhận), không được buffer toàn bộ vào RAM. |
| **REV-04** | Kẹt phím (Sticky Keys) khi rớt mạng | **Medium** | Nếu phím KeyDown được gửi nhưng mạng ngắt trước khi gửi KeyUp, máy nhận sẽ bị kẹt phím liên tục. | Thiết kế nhịp tim (Heartbeat 500ms). Nếu mất kết nối, máy nhận tự động phát hành lệnh release toàn bộ modifiers/keys. |

---

## 2. KẾT LUẬN & ĐỀ XUẤT CHO APPROVAL GATE
- Bản thiết kế `SPEC.md` có tính khả thi kỹ thuật rất cao, tận dụng thế mạnh hiệu năng và độ an toàn của Rust.
- Yêu cầu @dev thực hiện nghiêm ngặt: Mã hóa kết nối AES-GCM/ChaCha20, truyền file dạng chunked stream, và cơ chế an toàn chống kẹt phím.
