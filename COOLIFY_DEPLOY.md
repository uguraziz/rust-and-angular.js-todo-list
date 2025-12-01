# Coolify ile Deployment Rehberi

Bu proje Coolify kullanarak kolayca deploy edilebilir. Aşağıdaki adımları takip edin.

## Gereksinimler

- Coolify kurulu bir sunucu
- Git repository'ye erişim
- Docker desteği

## Deployment Adımları

### 1. Git Repository'yi Hazırlayın

Projenizi bir Git repository'ye push edin (GitHub, GitLab, vb.)

```bash
git add .
git commit -m "Coolify deployment için hazırlandı"
git push origin main
```

### 2. Coolify'da Yeni Bir Resource Oluşturun

1. Coolify dashboard'una giriş yapın
2. "New Resource" butonuna tıklayın
3. "Docker Compose" seçeneğini seçin
4. Git repository URL'nizi girin
5. Branch'i seçin (genellikle `main` veya `master`)

### 3. Environment Variables Ayarlayın

Coolify'da aşağıdaki environment variable'ları ayarlayın:

**Backend için:**
- `PORT=8080` (Coolify otomatik olarak ayarlayabilir)
- `HOST=0.0.0.0`
- `DATABASE_URL=sqlite:./data/todos.db`
- `RUST_LOG=info`

**Frontend için:**
- `API_URL=http://backend:8080/api` (opsiyonel, nginx proxy kullanılıyor)

### 4. Port Ayarları

Coolify otomatik olarak port'ları yönetir, ancak manuel ayarlama yapmak isterseniz:

- **Backend**: Port 8080 (internal)
- **Frontend**: Port 80 (public erişim için)

### 5. Build ve Deploy

1. Coolify otomatik olarak `docker-compose.yml` dosyasını algılayacak
2. "Deploy" butonuna tıklayın
3. Build işlemi başlayacak (ilk build biraz zaman alabilir)

### 6. Domain Ayarları

1. Coolify'da resource'unuzun ayarlarına gidin
2. "Domains" bölümünden domain ekleyin
3. Frontend servisi için domain'i ayarlayın
4. SSL sertifikası otomatik olarak oluşturulacak

## Alternatif: Manuel Docker Compose Deploy

Eğer Coolify'ın Docker Compose desteğini kullanmak istemiyorsanız, her servisi ayrı ayrı deploy edebilirsiniz:

### Backend Deploy

1. Coolify'da "New Resource" > "Dockerfile" seçin
2. Path: `./backend`
3. Dockerfile path: `./backend/Dockerfile`
4. Port: 8080

### Frontend Deploy

1. Coolify'da "New Resource" > "Dockerfile" seçin
2. Path: `./frontend`
3. Dockerfile path: `./frontend/Dockerfile`
4. Port: 80
5. Backend servisini "depends_on" olarak ekleyin

## Veritabanı

Proje SQLite kullanıyor ve veritabanı dosyası `backend/data/todos.db` konumunda saklanıyor. Coolify volume'ları kullanarak verilerin kalıcı olmasını sağlayın.

## Troubleshooting

### Backend başlamıyor

- Logları kontrol edin: `docker logs todo-backend`
- Port'un doğru ayarlandığından emin olun
- Database path'inin yazılabilir olduğundan emin olun

### Frontend backend'e bağlanamıyor

- Nginx proxy ayarlarını kontrol edin
- Backend servisinin çalıştığından emin olun
- Network ayarlarını kontrol edin (aynı network'te olmalılar)

### Build hataları

- Rust toolchain'in doğru yüklendiğinden emin olun
- Node.js versiyonunun uyumlu olduğundan emin olun
- Dependencies'in güncel olduğundan emin olun

## Önemli Notlar

1. **İlk build uzun sürebilir**: Rust ve Angular build işlemleri zaman alabilir
2. **Volume'lar**: Veritabanı verilerinin kaybolmaması için volume'ları ayarlayın
3. **Environment Variables**: Production'da güvenli environment variable'lar kullanın
4. **SSL**: Coolify otomatik SSL sertifikası sağlar (Let's Encrypt)

## Destek

Sorun yaşarsanız:
- Coolify loglarını kontrol edin
- Docker container loglarını inceleyin
- Network bağlantılarını test edin

