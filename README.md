# Asteroids
Jogo estilo Asteroids feito em Rust com [macroquad](https://macroquad.rs), com suporte para Linux e Windows.

## Como funciona

- Cada asteroide que você destrói se divide em 3 ou 4 asteroides menores, com um lado a menos.
- Ao chegar em um quadrado (4 lados), o próximo tiro o destrói de vez.
- Destrua todos os asteroides para vencer. Se encostar em um, você perde.


## Como Baixar
### Linux
1. Baixe o arquivo `Asteroides_Linux`
2. Abra o terminal na pasta do Download
```bash
cd ~/Pasta
```
3. Dê permissão de execução ao arquivo com
```bash
chmod +x Asteroides_Linux
```
4. E abra o arquivo usando
```bash
./Asteroides_Linux
```

### Windows

1. Baixe o arquivo `Asteroides_Windows.exe`.
2. Dê dois cliques para abrir.

> [!NOTE]
> O Windows pode mostrar um aviso por o programa ser desconhecido.
> Clique em **Mais informações** e depois em **Executar assim mesmo**.

## Controles

| Tecla | Ação |
|---|---|
| ↑ | Acelerar |
| ← / → | Girar a nave |
| Espaço | Atirar (máximo de 5 tiros na tela) |
| Enter | Jogar novamente (na tela final) |
