# 🧪 Test ve Doğrulama (Testing)

ISU-SecOps-Engine'in kararlılığını ve doğruluğunu garanti altına almak için hazırlanan test prosedürleri aşağıda detaylandırılmıştır.

## 1. Birim (Unit) Testleri
Rust çekirdek motoru ve IP hesaplama algoritmaları için tasarlanan birim testler.
- **Komut:** `cargo test`
- **İçerik:** 
  - `expand_target`: CIDR bloklarının (/24 vb.) doğru IP dizilerine dönüştürülmesi.
  - `FingerprintResult Serialization`: JSON verilerinin frontend uyumluluğu.
  - `IPNet` Entegrasyonu: Ağ sınırlarının doğrulanması.

## 2. Entegrasyon ve Fonksiyonel Testler
Uygulamanın hem backend hem de frontend katmanlarının uyumunu denetleyen interaktif testlerdir.

### Senaryo A: Port Profili ve UI Etkileşimi
1. **Buton Kontrolü:** "Web", "DB", "Popular" butonlarına tıklanarak port girdisinin anında değiştiği doğrulanır.
2. **Görsel Geri Bildirim:** Tıklanan profil butonunun aktif (highlight) olduğu gözlemlenir.

### Senaryo B: Akıllı Host Keşfi ve Tarama Akışı
1. **Hedef:** Gerçek bir ağ bloğu veya `scanme.nmap.org`.
2. **Doğrulama:** Tarama başladığında tepedeki metrik kartlarının (Hosts Scanned, Open Ports) canlı olarak arttığı görülmelidir.
3. **Smart Discovery:** Host kapalıysa servis taramasının atlandığı ve zamandan tasarruf edildiği kontrol edilir.

### Senaryo C: Kalıcılık ve SQLite Veritabanı
1. Başarılı bir tarama yapıldıktan sonra tarayıcı yenilenir (F5).
2. Sayfa yüklendiğinde `Recent Activity` (Son Etkinlikler) kısmında taramanın loglandığı ve tıklandığında sonuçların tekrar geldiği doğrulanır.

### Senaryo D: Girdi Doğrulama ve Güvenlik
1. **Yanlış Giriş:** IP hanesine harf veya geçersiz CIDR (Örn: `/33`) girilir.
2. **Beklenen Sonuç:** Uygulama hata mesajını gösterir ve shake (sallanma) animasyonunu tetikleyerek taramayı engeller.

## 3. Raporlama ve Dışa Aktarma
1. `JSON` ve `CSV` butonlarına basılarak verinin ham hali indirilir.
2. `PDF Report` butonuna basılarak tarayıcı baskı önizlemesi açılır ve sayfanın temiz (gereksiz UI öğelerinden arındırılmış) bir rapor sunduğu doğrulanır.
