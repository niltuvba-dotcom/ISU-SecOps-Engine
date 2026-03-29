# Proje Mimarisi (Architecture)

ISU-SecOps-Engine, modern ve yüksek performanslı bir güvenlik aracı olarak üç ana katmandan oluşur:

## 1. Backend (Paslanmaz Motor - Rust)
- **Hız ve Güvenlik:** Asenkron Rust (Tokio) kullanılarak aynı anda yüzlerce portu minimum kaynakla tarayabilir.
- **Modüler Yapı:**
    - `fingerprint.rs`: Banner grabbing, servis tespiti ve CIDR çözümleme mantığı.
    - `database.rs`: SQLite entegrasyonu ve tarama geçmişi yönetimi.
    - `web.rs`: Axum tabanlı API ve WebSocket sunucusu.

## 2. İletişim Katmanı (Real-Time)
- **WebSockets:** Tarama sonuçları backend'den frontend'e anlık olarak akar. Bu sayede kullanıcı tüm taramanın bitmesini beklemeden ilk bulguları görebilir.

## 3. Frontend (Premium Dashboard)
- **Modern UI:** Vanilla JS ve CSS3 kullanılarak oluşturulan "Glassmorphism" tasarımı.
- **İstemci Tarafı Analitikler:** Gelen veriler tarayıcıda anlık olarak işlenerek servis dağılım grafikleri ve metrikler oluşturulur.
- **Raporlama:** CSS Print Media kullanılarak profesyonel PDF çıktıları üretilir.

## 4. Veri Saklama Katmanı
- **SQLite:** Tarama sonuçları yapılandırılmış bir şekilde lokal veritabanında saklanır, böylece uygulama kapatılıp açılsa bile geçmiş kaybolmaz.
