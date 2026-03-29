# 🛡️ ISU-SEC-OPS ENGINE

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-7000ff?style=for-the-badge)
![SQLite](https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white)

**ISU-SecOps-Engine**, profesyonel güvenlik uzmanları için tasarlanmış, yüksek performanslı ve asenkron çalışan bir servis parmak izi (fingerprinting) ve ağ keşif aracıdır. Rust dili ile geliştirilmiş çekirdek motoru, modern ve etkileşimli bir web arayüzü ile birleşerek gerçek zamanlı analiz imkanı sunar.

---

## 📈 Sürüm Geçmişi ve Özellikler

### [v1.3.0] - İstihbarat & Profesyonel Katman (Mevcut)
- **Akıllı Host Keşfi:** Geniş ağlarda (CIDR) sadece aktif hostları tespit ederek tarama süresini %80 optimize eden "Smart Discovery" motoru.
- **CVE Zafiyet Bilgisi:** Tespit edilen servis versiyonları için otomatik oluşturulan zafiyet araştırma (CVE) linkleri.
- **Profesyonel Analitik:** Servis dağılımını görsel barlarla gösteren dinamik dashboard bileşenleri.
- **PDF Raporlama:** Taramaları profesyonel bir formatta PDF olarak dışa aktarma (Print-Media).
- **Kurumsal Kimlik:** Hareketli (animated) SVG logo, favicon ve premium glassmorphism tasarımı.
- **Gelişmiş Form Doğrulama:** Regex tabanlı akıllı girdi kontrolü ve hata animasyonları.

### [v1.2.0] - Gelişmiş UX & CIDR Desteği
- **CIDR Tarama:** `/24`, `/16` gibi ağ aralıklarını tarama yeteneği.
- **Tarama Profilleri:** Sık kullanılan port grupları için "Web", "DB", "Popular" hazır seçim butonları.
- **Gerçek Zamanlı Metrikler:** Taranan host, bulunan açık port ve benzersiz servis sayısı sayaçları.
- **Dinamik İlerleme Çubuğu:** Tarama yüzdesini ve hedeflenen host sayısını gösteren akış.

### [v1.1.0] - Web Kontrol Paneli & Kalıcılık
- **WebSocket Entegrasyonu:** Backend'den frontend'e kesintisiz ve anlık veri akışı.
- **SQLite Geçmişi:** Yapılan tüm taramaların veritabanında saklanması ve geçmişe dönük analiz.
- **Dışa Aktarma:** Sonuçları JSON ve CSV formatlarında indirme desteği.

### [v1.0.0] - Temel Tarama Çekirdeği
- **Rust TCP Engine:** Tokio tabanlı yüksek eşzamanlılıklı (concurrent) bağlantı yönetimi.
- **Service Fingerprinting:** Banner grabbing yöntemiyle servis ve versiyon tespiti.
- **Performans Ayarları:** Eşzamanlılık (Semaphore) ve zaman aşımı (Timeout) yönetimi.

---

## 🛠️ Teknoloji Yığını

- **Backend:** `Rust`, `Axum` (Web Framework), `Tokio` (Async Runtime), `Rusqlite` (Database).
- **Frontend:** `Vanilla HTML5/CSS3`, `JavaScript ES6+`, `WebSockets`.
- **Ek Katmanlar:** `Serde` (JSON/Serialization), `IpNet` (Network calculation).

## 🚀 Kurulum ve Çalıştırma

1. **Ön Koşullar:** [Rust](https://rustup.rs/) ve C++ Build Tools (Windows için) yüklü olmalıdır.
2. **Klonlama:** `git clone https://github.com/your-repo/ISU-SecOps-Engine.git`
3. **Çalıştırma:** 
   ```bash
   cargo run -- web
   ```
4. **Erişim:** Tarayıcınızda `http://127.0.0.1:8080` adresini açın.

---
*Developed for ISU SecOps by Antigravity AI.*
