# 🏛️ Proje Mimarisi (Architecture)

Aetheris Engine, yüksek eşzamanlılık (High Concurrency) ve gerçek zamanlı veri akışı üzerine inşa edilmiş, katmanlı bir yazılım mimarisine sahiptir.

## 1. Backend: Rust Core Motoru
Uygulamanın kalbi, Rust dilinin güvenli ve asenkron (Tokio) yapısı üzerinde çalışır.

- **Asenkron Tarama Sistemi:** `tokio::spawn` ile her hedef ve port için bağımsız görevler (Tasks) oluşturulur. `tokio::sync::Semaphore` kullanılarak sistem kaynaklarının aşırı tüketilmesi önlenir.
- **Banner Grabbing:** TCP bağlantısı kurulduktan sonra pasif banner okuma ve regex tabanlı servis tespiti yapılır.
- **Akıllı Keşif (Smart Discovery):** Port taramasına başlamadan önce hedefin ayakta olup olmadığını hızlı bir TCP-Check ile doğrulayan katmandır.
- **Veritabanı Katmanı:** `rusqlite` aracılığıyla tarama sonuçları SQLite üzerinde yapılandırılmış bir şema ile saklanır.

## 2. İletişim: WebSocket Hub
HTTP API'ların aksine, Aetheris Engine kesintisiz veri iletimi için WebSockets tercih eder.

- **Unbounded Channels:** Backend motorundan gelen sonuçlar `mpsc::unbounded_channel` üzerinden WebSocket handler'ına iletilir.
- **Anlık Akış:** Kullanıcı arayüzü, tarama bitmeden her bulunan açık portu milisaniyeler içinde ekranda görüntüler.

## 3. Frontend: Modern Dashboard
Arayüz tasarımı, "Aesthetics matter" felsefesiyle kurumsal bir görünüm sunar.

- **Glassmorphism Design:** Yarı saydam paneller, neon efektleri ve hareketli SVG arka planlar ile modern bir deneyim sunulur.
- **İstemci Tarafı İşleme:** JS modülleri, gelen verileri yerel olarak filtreler, sıralar ve istatistiklere dönüştürür.
- **Raporlama Motoru:** `window.print()` ve `@media print` CSS kuralları ile tarayıcı üzerinden yüksek çözünürlüklü PDF raporları üretilir.

## 4. Güvenlik ve Doğrulama
- **Input Validation:** Geçersiz formatta IP, CIDR veya Port girişleri frontend düzeyinde Regex ile doğrulanır.
- **Memory Safety:** Rust'ın mülkiyet (ownership) modeli sayesinde bellek sızıntıları ve race-condition gibi hatalar çekirdek seviyesinde engellenmiştir.
