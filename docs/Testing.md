# Test ve Doğrulama (Testing)

ISU-SecOps-Engine'in düzgün çalıştığını doğrulamak için aşağıdaki yöntemler kullanılabilir:

## 1. Birim Testler (Unit Testing)
Rust tarafındaki iç mantığı test etmek için şu komutu çalıştırın:
```bash
cargo test
```
Bu komut, IP çözümleme (`expand_target`) ve fingerprinting yapılarını kontrol eder.

## 2. Entegrasyon ve Manuel Testler
Uygulamayı başlatın:
```bash
cargo run -- web
```
Ardından tarayıcıyı açın (`http://127.0.0.1:8080`) ve şu senaryoları uygulayın:

### Senaryo A: Port Profili Testi
1. "Web" butonuna tıklayın.
2. Port listesinin `80,443,8080,8443` olarak değiştiğini doğrulayın.

### Senaryo B: Canlı Tarama ve WebSocket Testi
1. Hedef IP hanesine `scanme.nmap.org` girin.
2. "Initiate Scan" butonuna basın.
3. Sonuçların tabloda anlık olarak (her port taraması bittiğinde) akış sağladığını doğrulayın.

### Senaryo C: Kalıcılık (Persistency) Testi
1. Bir tarama yapın ve sonuçları görün.
2. Sayfayı yenileyin (F5).
3. Sonuçların tablodan kaybolmadığını ve veritabanı üzerinden tekrar yüklendiğini doğrulayın.

### Senaryo D: Girdi Doğrulama (Validation) Testi
1. Geçersiz bir IP girin (Örn: `999.999.999.999`).
2. Uygulamanın taramayı başlatmadığını ve sallanma (shake) animasyonuyla hata verdiğini kontrol edin.
