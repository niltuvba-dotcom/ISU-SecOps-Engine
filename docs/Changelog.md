# 📜 Değişiklik Günlüğü (Changelog)

Tüm önemli değişiklikler ve özellik geliştirmeleri sürüm numaralarıyla birlikte aşağıda listelenmiştir.

---

## [v1.3.0] - 2026-04-04 (İstihbarat & Profesyonel Optimizasyon)
### Eklenenler
- **Akıllı Host Keşfi:** /24 gibi geniş ağ aralıklarında (CIDR) taramayı %80 hızlandıran ve sadece "aktif" makineleri hedef alan motor geliştirmesi.
- **CVE/Zafiyet İstihbaratı:** Tespit edilen her servis versiyonu için otomatik oluşturulan Google/NVD/Exploit-DB arama linkleri.
- **Dinamik Analitik Panel:** Bulunan servislerin dağılımını canlı olarak gösteren görsel barlar ve istatistik kartları.
- **Profesyonel PDF Raporu:** Taramaları sunuma hazır hibrit PDF formatında dışa aktarma yeteneği (Print-optimized CSS).
- **Kurumsal Tasarım:** Hareketli SVG logo, favicon ve premium cam efekti (Glassmorphism) iyileştirmesi.
- **Gelişmiş Validasyon:** Regex tabanlı akıllı IP/Ports/CIDR girdisi kontrolü ve hata animasyonları.

---

## [v1.2.0] - 2026-04-03 (Gelişmiş UX & CIDR Desteği)
### Eklenenler
- **Ağ Seviyesi Tarama:** CIDR (192.168.1.0/24 gibi) bloklarını otomatik genişleten IP hesaplama motoru.
- **Tarama Profilleri:** Tek tıkla port seçim imkanı sağlayan "Web", "DB", "Popular" preset butonları.
- **Gerçek Zamanlı Metrikler:** Taranan Host, Açık Port ve Benzersiz Servis sayısı için canlı sayaçlar.
- **İlerleme İzleme (Progress Tracking):** Taramanın % kaç tamamlandığını ve hedeflenen toplam IP/Port sayısını gösteren akıllı ilerleme çubuğu.

---

## [v1.1.0] - 2026-04-03 (Web Arayüzü & Veritabanı Entegrasyonu)
### Eklenenler
- **Web Dashboard:** Modern, karanlık tema destekli grafiksel kullanıcı arayüzü.
- **WebSocket (Gerçek Zamanlı):** Backend tarama sonuçlarını beklemeden anlık veri akışı sağlayan iletişim katmanı.
- **SQLite Kalıcılığı:** Taramaların geçmişe dönük kaydedilmesi (`database.rs`) ve her açılışta veritabanından geri yüklenmesi.
- **Export Desteği:** Bulguların JSON ve CSV formatında indirilmesi.

---

## [v1.0.0] - 2026-04-02 (Çekirdek Tarama Motoru)
### Eklenenler
- **Asenkron Engine:** Rust ve Tokio kullanılarak geliştirilen yüksek performanslı TCP tarama motoru.
- **Fingerprinting:** Banner grabbing yöntemiyle temel servis (SSH, HTTP, Redis vb.) ve sürüm tespiti.
- **Performans Yönetimi:** Kullanıcı tanımlı eşzamanlılık (Semaphore) ve zaman aşımı limitleri.
- **CLI Modu:** Komut satırı üzerinden hızlı tarama yapabilme yeteneği.
