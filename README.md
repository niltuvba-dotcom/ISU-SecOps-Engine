# 🌌 Aetheris Engine (v1.4.0)

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Aether](https://img.shields.io/badge/Aether-7000ff?style=for-the-badge)
![SQLite](https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white)

**Aetheris Engine**, profesyonel güvenlik uzmanları için tasarlanmış, "Specter-Grade" (hayalet seviyesi) hızda çalışan bir servis parmak izi (fingerprinting) ve ağ keşif asistanıdır. Rust dili ile geliştirilmiş çekirdek motoru, modern ve etkileşimli bir web dashboard'u ile birleşerek ağ üzerindeki görünmez servisleri görünür kılar.

---

## 📉 Sürüm Geçmişi ve Özellikler

### [v1.4.0] - Rebranding & Premium CLI
- **Modern Marka Kimliği:** Proje "Aetheris" olarak yeniden markalandı.
- **Premium CLI:** Renkli tablolar (`comfy-table`), canlı ilerleme çubukları (`indicatif`) ve geliştirilmiş terminal UX.
- **Subcommand Yapısı:** `aetheris pentest fingerprint` hiyerarşisi ile profesyonel kullanım.

### [v1.3.0] - İstihbarat & Profesyonel Katman
- **Akıllı Host Keşfi:** Geniş ağlarda sadece aktif hostları tespit ederek süreyi %80 optimize eden motor.
- **CVE Zafiyet Bilgisi:** Tespit edilen servisler için otomatik zafiyet araştırma (CVE) linkleri.
- **Profesyonel Analitik:** Servis dağılımını gösteren dinamik dashboard bileşenleri.
- **PDF Raporlama:** Taramaları profesyonel PDF formatında dışa aktarma.

---

## 🛠️ Teknoloji Yığını

- **Backend:** `Rust`, `Axum`, `Tokio`, `Rusqlite`.
- **Frontend:** `Vanilla JavaScript`, `CSS3 Glassmorphism`, `WebSockets`.
- **CLI:** `Clap`, `Indicatif`, `Comfy-Table`, `Colored`.

## 🚀 Kurulum ve Çalıştırma

1. **Ön Koşullar:** [Rust](https://rustup.rs/) yüklü olmalıdır.
2. **Klonlama:** `git clone https://github.com/your-repo/Aetheris-Engine.git`
3. **Çalıştırma:** 
   ```bash
   cargo run -- web
   ```
4. **Pentest (CLI):**
   ```bash
   cargo run -- pentest fingerprint scanme.nmap.org --ports 22,80,443
   ```

---
*Developed by Antigravity AI - Aetheris Ecosystem.*
