# 🌌 Aetheris Engine (v1.4.0)

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/CLI-Premium-7000ff?style=for-the-badge)
![Security](https://img.shields.io/badge/SecOps-Advanced-00f3ff?style=for-the-badge)

**Aetheris Engine**, profesyonel güvenlik uzmanları için tasarlanmış, **öncelikli olarak terminal (CLI) performansına odaklanan**, yüksek hızlı bir ağ keşif ve servis parmak izi (fingerprinting) asistanıdır. "Specter-Grade" hızı ve modern terminal UX/UI tasarımıyla ağınızdaki görünmez servisleri saniyeler içinde raporlar.

---

## ⌨️ Terminal (CLI) Deneyimi

Aetheris, terminalde sadece veri basmaz; modern, renkli ve etkileşimli bir deneyim sunar.

### 🚀 Temel Kullanım (Pentest)
```bash
# Servis Parmak İzi Tespiti (Gelişmiş Tablo Çıktısı)
aetheris pentest fingerprint <target> --ports 22,80,443

# Tüm Port Aralığını Tara
aetheris pentest fingerprint 192.168.1.1 --ports 1-1000 --concurrency 500

# Ağ Bloğu (CIDR) Taraması
aetheris pentest fingerprint 10.0.0.0/24 --ports 80,443
```

### 📊 Çıktı Formatları
Terminal sonuçlarını farklı formatlarda dışa aktarabilirsiniz:
-   **Table (Varsayılan):** İnsan dostu, renkli ve hizalanmış UTF-8 tablolar.
-   **JSON:** Otomasyon ve scripting için tam uyumlu veri yapısı.
-   **CSV:** Raporlama ve Excel entegrasyonu için.

---

## 📉 Sürüm Notları (v1.4.0)

### 💎 Terminal Devrimi
- **Aetheris CLI Hiyerarşisi:** `pentest fingerprint` yapısıyla profesyonel komut akışı.
- **Canlı Progress Bar:** `indicatif` ile her bir portun ve hostun tarama ilerlemesi anlık takip edilir.
- **Premium Tablolar:** `comfy-table` ile servisin versiyonu, CVE linkleri ve durumu kusursuz bir tabloda sunulur.

### 🌐 Web Dashboard (Companion)
*Terminal tercih etmeyen anlar için* dashboard üzerinden gerçek zamanlı WebSocket akışı, servis analitikleri ve PDF raporlama modülleri mevcuttur.

---

## 🛠️ Hızlı Komutlar (Justfile)

Proje yönetimini ve testleri terminalden tek kelimeyle halledin:
- `just fmt`: Kod formatını düzeltir.
- `just lint`: Güvenlik ve performans denetimi yapar.
- `just test`: Tüm birim testleri çalıştırır.
- `just scan`: Hızlı bir lokal tarama başlatır.

---

## 🚀 Kurulum

1. **Rust Yükle:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **Klonla & Derle:**
   ```bash
   git clone https://github.com/aetheris/engine.git
   cd engine
   cargo build --release
   ```

---
*Developed with Aetheris Protocol. Focus on Terminal. Built for Security.*
