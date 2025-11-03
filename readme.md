# jogo do dino
eu não quero trabalhar com jogos mas a professora da minha escola disse que era pra criar um jogo do dino em scracth eu, fiz em rust ao invez de scrath

# como compilar
digite: 
```
cd dino
cargo bild --release
cargo rustc --release --bin dino -- -C link-args=/ENTRY:mainCRTStartup -C link-args=/SUBSYSTEM:WINDOWS
``` 
# extra 
perdi muito tempo aprendendo a usar o macroquad e fazendo esse jogo do dino então fiquei com preguiça de recompilar pra Web Assembly

e tmbem criei um executavel na pasta na pasta  ```executavel``` pra não ter que precisar instalar o rustup e suas depencias pesadas (ocupando cerca de 9gb)