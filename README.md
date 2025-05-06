### Прохождение тренажера по Rust

Прохождение идет паралельно курса на 
Stepik - https://stepik.org/course/195449/syllabus
Инженерка - https://inzhenerka.tech/members/courses/course530657683705

В данном репозитории Rast полностью завернут в Докер, сборка и компиляция выполняется следующими командами:

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
