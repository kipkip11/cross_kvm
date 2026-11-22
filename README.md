# Cross-KVM (v1.0.0)

Phần mềm chia sẻ chuột, bàn phím và đồng bộ Clipboard/File hai chiều (Zero-Lag, Encrypted) giữa **macOS** và **Windows** qua mạng LAN nội bộ.

---

## ✨ Tính năng chính
- 🖱️ **Chia sẻ Chuột & Bàn phím:** Di chuyển chuột xuyên suốt mép màn hình giữa macOS và Windows với độ trễ siêu thấp (<2ms).
- 📋 **Clipboard Sync hai chiều:** Tự động đồng bộ văn bản (Text) và hình ảnh (Images).
- 📁 **File Copy-Paste (Chunked Stream):** Copy file trực tiếp từ máy tính này và Paste sang máy tính kia, lưu trữ an toàn tại `~/Downloads/LanDrop/`.
- 🔒 **Mã hóa E2E:** Toàn bộ dữ liệu phím, chuột và tệp tin được mã hóa bằng thuật toán dòng **ChaCha20-Poly1305** với mã PIN / Pre-Shared Key (PSK).

---

## 🚀 Hướng dẫn sử dụng

### 1. Khởi động trên Server (Ví dụ macOS)
```bash
cross_kvm --mode server --bind 0.0.0.0:24900 --pin 888888
```

### 2. Kết nối từ Client (Ví dụ Windows)
```bash
cross_kvm.exe --mode client --connect 192.168.1.x:24900 --pin 888888
```

---

## 🛠️ Build & Test
```bash
cargo build --release
cargo test
```
