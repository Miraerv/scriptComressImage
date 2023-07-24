# Скрипт для сжатия изображений

Этот скрипт на Rust, который сжимает изображения в заданной директории.

## Установка и использование через Docker

1. Соберите Docker образ:
```bash
docker build -t script-compress-image .
```


2. Соберите Docker образ:
```bash
docker run -v <путь к директории с изображениями на хосте>:/data script-compress-image
```



