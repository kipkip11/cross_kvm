# SPEC: CROSS-KVM (macOS <-> Windows Mouse, Keyboard & Clipboard/File Sharing)

**Task ID:** TASK-20261122-KVM  
**Tác giả:** @phan-tich  
**Trạng thái:** DRAFT / PENDING_REVIEW  
**Phiên bản:** 1.0  

---

## 1. BỐI CẢNH & HIỆN TRẠNG
- Người dùng cần một phần mềm KVM ảo (Software KVM) chia sẻ chuột, bàn phím mượt mà giữa **macOS** (Mac mini M4) và **Windows** qua mạng LAN nội bộ.
- Yêu cầu đặc biệt quan trọng: **Đồng bộ Clipboard 2 chiều hoàn hảo (Text, Rich Text và File Copy/Paste xuyên suốt hai máy)**.
- Môi trường sẵn có: Rust (v1.98), Python 3.9, Node.js v24.

---

## 2. PHẠM VI (SCOPE)
### Trong phạm vi:
1. **Chia sẻ Chuột & Bàn phím không dây:**
   - Chuyển đổi con trỏ chuột mượt mà khi chạm mép màn hình (Edge Transition) hoặc qua phím tắt nóng (Hotkeys).
   - Truyền tọa độ di chuyển, click, scroll và phím gõ thời gian thực (< 5ms latency).
2. **Clipboard Sync 2 chiều (Text & Images):**
   - Tự động nhận diện thay đổi trên Clipboard của máy A và đồng bộ sang máy B.
3. **File Copy-Paste & Transfer:**
   - Khi copy file trên macOS/Windows, clipboard được đánh dấu định dạng file ảo. Khi paste sang máy kia, luồng dữ liệu file được stream trực tiếp qua TCP Socket nội bộ với thanh tiến trình.
4. **Bảo mật:**
   - Xác thực bắt tay bằng mã PIN / Pre-shared Key (PSK) trong mạng LAN để ngăn máy lạ chiếm quyền chuột/phím.

### Ngoài phạm vi:
- Audio streaming / Màn hình phụ (Screen mirroring).

---

## 3. KIẾN TRÚC KỸ THUẬT ĐỀ XUẤT
- **Ngôn ngữ lõi (Core Engine):** **Rust** (Hiệu năng cực đại, zero-cost abstractions, an toàn bộ nhớ, độ trễ sub-millisecond).
- **Network Protocol:**
  - **Kênh Input (Mouse/Keyboard):** UDP / Raw TCP Socket với cơ chế gộp packet (coalescing) và TCP_NODELAY để giảm độ trễ về mức tối thiểu.
  - **Kênh File & Clipboard:** TCP Stream chuyên biệt với buffer chunking (64KB - 1MB) kèm checksum SHA-256.
- **Hệ thống Hook Input:**
  - **macOS:** Sử dụng `CoreGraphics` (`CGEventTap` để bắt input và `CGEventPost` để bơm input ảo, yêu cầu quyền Accessibility).
  - **Windows:** Sử dụng `SetWindowsHookEx` (WH_MOUSE_LL, WH_KEYBOARD_LL) và `SendInput`.

---

## 4. ACCEPTANCE CRITERIA (AC)
- **AC-1:** Con trỏ chuột chuyển từ macOS sang mép màn hình Windows mượt mà không giật lag.
- **AC-2:** Bàn phím gõ được đầy đủ các ký tự, modifiers (Cmd/Ctrl, Shift, Alt/Option), không bị kẹt phím (sticky keys).
- **AC-3:** Copy đoạn văn bản (text) trên macOS -> Paste tức thì trên Windows (và ngược lại).
- **AC-4:** Copy 1 hoặc nhiều file trên máy A -> Paste vào thư mục trên máy B, toàn vẹn dữ liệu (checksum khớp).
- **AC-5:** Đảm bảo có xác thực kết nối, không cho phép client lạ gửi keystroke.

---

## 5. KẾ HOẠCH TRIỂN KHAI & PHÂN CÔNG
- **Bước 1 (Protocol & Network Core):** Xây dựng server/client daemon truyền thông điệp chuột/phím và file.
- **Bước 2 (macOS Native Agent):** Xây dựng module bắt và điều khiển input qua CoreGraphics.
- **Bước 3 (Windows Native Agent):** Xây dựng module nhận diện hook trên Windows.
- **Bước 4 (Clipboard & File Engine):** Xây dựng engine giám sát clipboard và socket truyền file.
- **Bước 5 (@qc):** Kiểm thử độ trễ, kiểm thử bắt phím, kiểm thử copy file lớn và rà soát memory leaks.
