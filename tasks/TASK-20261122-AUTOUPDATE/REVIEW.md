# REVIEW: AN TOÀN & BẢO MẬT CHO TÍNH NĂNG AUTO-UPDATE (CROSS-KVM)

**Task ID:** TASK-20261122-AUTOUPDATE  
**Người đánh giá:** @phan-bien  
**Tài liệu rà soát:** `SPEC.md` v1.0  
**Kết luận:** `APPROVE` (Đồng ý triển khai với lưu ý kiểm soát bảo mật)

---

## 1. ĐÁNH GIÁ RỦI RO & BẢO MẬT

| Mã | Phát hiện | Mức độ | Tác động | Giải pháp bắt buộc |
|---|---|---|---|---|
| **REV-SEC-01** | Tải binary từ nguồn ngoài không xác thực | **High** | Nếu tài khoản GitHub hoặc DNS bị tấn công (DNS Spoofing), client có thể tải mã độc về chạy quyền hệ thống. | 1. Chỉ gọi HTTPS tới `api.github.com` với TLS verify.<br>2. Bắt buộc kiểm tra SHA-256 Checksum công bố trên Release trước khi thực thi file. |
| **REV-SEC-02** | File bị khóa trên Windows (Sharing Violation) | **Medium** | Windows không cho phép ghi đè file `.exe` đang chạy, dễ dẫn đến lỗi crash cập nhật giữa chừng. | Đổi tên file đang chạy thành `.exe.old` trước khi ghi file mới, hoặc dùng một tiến trình update phụ tách rời. |
| **REV-SEC-03** | Rollback khi bản update bị lỗi khởi động | **Medium** | Bản cập nhật mới bị crash ngay khi bật sẽ làm máy mất luôn khả năng auto-update các bản sau. | Giữ lại file backup `.old`, nếu bản mới crash sau 3 lần thử thì tự động rollback về bản cũ. |

---

## 2. KẾT LUẬN
Bản thiết kế đạt tiêu chuẩn chất lượng. Đề xuất chuyển sang Cổng Phê duyệt (Approval Gate) để User xác nhận triển khai.
