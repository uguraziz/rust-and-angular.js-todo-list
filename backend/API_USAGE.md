# Rust Todo API Kullanım Örnekleri

## Yeni todo oluştur
```
curl -X POST http://localhost:8080/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"İlk Todo","description":"Test açıklaması"}'
```

## Tüm todoları listele
```
curl http://localhost:8080/api/todos
```

## Belirli bir todo getir
```
curl http://localhost:8080/api/todos/1
```

## Todo durumunu değiştir
```
curl -X PATCH http://localhost:8080/api/todos/1/toggle
```

## Todo güncelle
```
curl -X PUT http://localhost:8080/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Güncellenmiş Todo","completed":true}'
```

## Todo sil
```
curl -X DELETE http://localhost:8080/api/todos/1
```
