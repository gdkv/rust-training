### Прохождение тренажера по Rust


В данном репозитории Rust полностью завернут в Докер, 
сборка и компиляция выполняется следующими командами:

```
docker-compose up
```

Затем уже потом заходим в контейнер и компилируем/запускаем программу

```
docker-compose run test-app ash
cd /usr/src/test-app
cargo build --release
./target/release/test-app
```
